//! # Rustodon Media Module
//!
//! 媒体附件处理模块，负责处理图片、视频、音频等媒体文件的上传、存储和处理。
//!
//! ## 功能特性
//!
//! - 支持多种媒体格式：图片（JPEG, PNG, GIF, WebP）、视频（MP4, WebM）、音频（MP3, OGG）
//! - 自动生成缩略图和预览图
//! - BlurHash 生成用于模糊预览
//! - 媒体元数据提取（尺寸、时长、编码信息等）
//! - 异步文件处理和存储
//! - 焦点坐标支持，用于智能裁剪
//! - 文件大小和格式验证
//! - 安全的文件上传处理
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use rustodon_media::{MediaProcessor, StorageConfig};
//! use sqlx::PgPool;
//!
//! async fn example() -> anyhow::Result<()> {
//!     let pool = PgPool::connect("postgresql://...").await?;
//!     let config = StorageConfig::default();
//!     let processor = MediaProcessor::new(pool, config);
//!     processor.initialize_storage().await?;
//!     Ok(())
//! }
//! ```

use std::path::PathBuf;

use anyhow::{Context, Result};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::{debug, error, info, warn};

/// 媒体类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// 静态图片
    Image,
    /// 视频文件
    Video,
    /// GIF 动画（转换为无声视频）
    Gifv,
    /// 音频文件
    Audio,
    /// 未知或不支持的格式
    Unknown,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::Image => write!(f, "image"),
            MediaType::Video => write!(f, "video"),
            MediaType::Gifv => write!(f, "gifv"),
            MediaType::Audio => write!(f, "audio"),
            MediaType::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<&str> for MediaType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "image" => MediaType::Image,
            "video" => MediaType::Video,
            "gifv" => MediaType::Gifv,
            "audio" => MediaType::Audio,
            _ => MediaType::Unknown,
        }
    }
}

/// 媒体处理状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessingStatus {
    /// 等待处理
    Pending,
    /// 正在处理
    Processing,
    /// 处理完成
    Processed,
    /// 处理失败
    Failed,
}

impl std::fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingStatus::Pending => write!(f, "pending"),
            ProcessingStatus::Processing => write!(f, "processing"),
            ProcessingStatus::Processed => write!(f, "processed"),
            ProcessingStatus::Failed => write!(f, "failed"),
        }
    }
}

impl From<&str> for ProcessingStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => ProcessingStatus::Pending,
            "processing" => ProcessingStatus::Processing,
            "processed" => ProcessingStatus::Processed,
            "failed" => ProcessingStatus::Failed,
            _ => ProcessingStatus::Pending,
        }
    }
}

/// 焦点坐标，用于智能裁剪
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocalPoint {
    /// X 坐标 (-1.0 到 1.0)
    pub x: f32,
    /// Y 坐标 (-1.0 到 1.0)
    pub y: f32,
}

impl FocalPoint {
    /// 创建新的焦点坐标
    pub fn new(x: f32, y: f32) -> Result<Self> {
        if !(-1.0..=1.0).contains(&x) || !(-1.0..=1.0).contains(&y) {
            anyhow::bail!("焦点坐标必须在 -1.0 到 1.0 范围内");
        }
        Ok(Self { x, y })
    }

    /// 从字符串解析焦点坐标 "x,y"
    pub fn from_string(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            anyhow::bail!("焦点坐标格式错误，应为 'x,y'");
        }

        let x = parts[0].trim().parse::<f32>().context("无法解析 X 坐标")?;
        let y = parts[1].trim().parse::<f32>().context("无法解析 Y 坐标")?;

        Self::new(x, y)
    }

    /// 转换为字符串格式
    pub fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

/// 媒体元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    /// 焦点坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<FocalPoint>,

    /// 原始文件信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<MediaDimensions>,

    /// 缩略图信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small: Option<MediaDimensions>,

    /// 视频/音频时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// 视频帧率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,

    /// 音频编码信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_encode: Option<String>,

    /// 音频比特率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<String>,

    /// 音频声道
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<String>,

    /// 视频比特率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
}

/// 媒体尺寸信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDimensions {
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
    /// 尺寸字符串 "宽x高"
    pub size: String,
    /// 宽高比
    pub aspect: f64,
}

impl MediaDimensions {
    /// 创建新的尺寸信息
    pub fn new(width: u32, height: u32) -> Self {
        let aspect = if height > 0 {
            width as f64 / height as f64
        } else {
            0.0
        };

        Self {
            width,
            height,
            size: format!("{}x{}", width, height),
            aspect,
        }
    }
}

/// 媒体上传请求
#[derive(Debug, Clone)]
pub struct MediaUploadRequest {
    /// 用户账户ID
    pub account_id: i64,

    /// 文件数据
    pub file_data: Bytes,

    /// 原始文件名
    pub file_name: String,

    /// MIME 类型
    pub content_type: String,

    /// 媒体描述（可选）
    pub description: Option<String>,

    /// 焦点坐标（可选）
    pub focus: Option<FocalPoint>,

    /// 自定义缩略图数据（可选）
    pub thumbnail_data: Option<Bytes>,
}

/// 媒体附件数据库模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAttachment {
    /// 媒体ID
    pub id: i64,

    /// 账户ID
    pub account_id: i64,

    /// 状态ID（可选）
    pub status_id: Option<i64>,

    /// 媒体类型
    #[serde(rename = "type")]
    pub media_type: MediaType,

    /// 原始文件URL
    pub url: Option<String>,

    /// 预览图URL
    pub preview_url: Option<String>,

    /// 远程URL
    pub remote_url: Option<String>,

    /// 文件名
    pub file_name: Option<String>,

    /// 文件大小
    pub file_size: Option<i64>,

    /// 文件MIME类型
    pub file_content_type: Option<String>,

    /// 元数据
    pub meta: MediaMetadata,

    /// 描述
    pub description: Option<String>,

    /// BlurHash
    pub blurhash: Option<String>,

    /// 处理状态
    pub processing_status: ProcessingStatus,

    /// 焦点X坐标
    pub focus_x: Option<f32>,

    /// 焦点Y坐标
    pub focus_y: Option<f32>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 存储配置
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// 媒体文件存储根目录
    pub media_root: PathBuf,

    /// 基础URL前缀
    pub base_url: String,

    /// 最大文件大小（字节）
    pub max_file_size: u64,

    /// 支持的图片格式
    pub supported_image_formats: Vec<String>,

    /// 支持的视频格式
    pub supported_video_formats: Vec<String>,

    /// 支持的音频格式
    pub supported_audio_formats: Vec<String>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            media_root: PathBuf::from("./storage/media"),
            base_url: "http://localhost:3000".to_string(),
            max_file_size: 40 * 1024 * 1024, // 40MB
            supported_image_formats: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "image/webp".to_string(),
            ],
            supported_video_formats: vec![
                "video/mp4".to_string(),
                "video/webm".to_string(),
                "video/quicktime".to_string(),
            ],
            supported_audio_formats: vec![
                "audio/mpeg".to_string(),
                "audio/ogg".to_string(),
                "audio/wav".to_string(),
                "audio/mp4".to_string(),
            ],
        }
    }
}

/// 媒体处理器
#[derive(Clone)]
pub struct MediaProcessor {
    /// 数据库连接池
    pool: PgPool,

    /// 存储配置
    config: StorageConfig,
}

impl MediaProcessor {
    /// 创建新的媒体处理器
    pub fn new(pool: PgPool, config: StorageConfig) -> Self {
        info!("初始化媒体处理器，存储根目录: {:?}", config.media_root);
        Self { pool, config }
    }

    /// 使用默认配置创建媒体处理器
    pub fn with_default_config(pool: PgPool) -> Self {
        Self::new(pool, StorageConfig::default())
    }

    /// 初始化存储目录
    pub async fn initialize_storage(&self) -> Result<()> {
        info!("初始化存储目录: {:?}", self.config.media_root);

        // 创建主目录
        fs::create_dir_all(&self.config.media_root)
            .await
            .context("创建媒体存储根目录失败")?;

        // 创建子目录
        let subdirs = ["original", "small", "thumbnails"];
        for subdir in &subdirs {
            let path = self.config.media_root.join(subdir);
            fs::create_dir_all(&path)
                .await
                .with_context(|| format!("创建子目录 {} 失败", subdir))?;
        }

        info!("存储目录初始化完成");
        Ok(())
    }

    /// 上传媒体文件
    pub async fn upload_media(&self, request: MediaUploadRequest) -> Result<MediaAttachment> {
        info!(
            "开始处理媒体上传，用户ID: {}, 文件名: {}",
            request.account_id, request.file_name
        );

        // 验证文件大小
        if request.file_data.len() as u64 > self.config.max_file_size {
            anyhow::bail!("文件大小超过限制: {} bytes", request.file_data.len());
        }

        // 检测媒体类型
        let media_type = self.detect_media_type(&request.content_type)?;
        info!("检测到媒体类型: {:?}", media_type);

        // 验证文件格式
        self.validate_file_format(&media_type, &request.content_type)?;

        // 生成文件哈希
        let file_hash = self.calculate_file_hash(&request.file_data);

        // 创建数据库记录
        let mut attachment = self
            .create_database_record(&request, &media_type, &file_hash)
            .await?;

        // 对于小文件（图片），同步处理并返回完整结果
        if media_type == MediaType::Image && request.file_data.len() < 1024 * 1024 {
            attachment = self.process_image_sync(attachment.id, &request).await?;
        } else {
            // 异步处理文件
            let processor = self.clone();
            let attachment_id = attachment.id;
            let request_clone = request.clone();
            tokio::spawn(async move {
                if let Err(e) = processor
                    .process_media_async(attachment_id, request_clone)
                    .await
                {
                    error!("异步媒体处理失败: {}", e);
                    // 更新处理状态为失败
                    if let Err(update_err) = processor
                        .update_processing_status(attachment_id, ProcessingStatus::Failed)
                        .await
                    {
                        error!("更新处理状态失败: {}", update_err);
                    }
                }
            });
        }

        info!("媒体上传处理完成，ID: {}", attachment.id);
        Ok(attachment)
    }

    /// 检测媒体类型
    fn detect_media_type(&self, content_type: &str) -> Result<MediaType> {
        debug!("检测媒体类型: {}", content_type);

        if self
            .config
            .supported_image_formats
            .contains(&content_type.to_string())
        {
            // 特殊处理 GIF
            if content_type == "image/gif" {
                return Ok(MediaType::Gifv);
            }
            return Ok(MediaType::Image);
        }

        if self
            .config
            .supported_video_formats
            .contains(&content_type.to_string())
        {
            return Ok(MediaType::Video);
        }

        if self
            .config
            .supported_audio_formats
            .contains(&content_type.to_string())
        {
            return Ok(MediaType::Audio);
        }

        warn!("不支持的媒体类型: {}", content_type);
        Ok(MediaType::Unknown)
    }

    /// 验证文件格式
    fn validate_file_format(&self, media_type: &MediaType, content_type: &str) -> Result<()> {
        match media_type {
            MediaType::Image | MediaType::Gifv => {
                if !self
                    .config
                    .supported_image_formats
                    .contains(&content_type.to_string())
                {
                    anyhow::bail!("不支持的图片格式: {}", content_type);
                }
            }
            MediaType::Video => {
                if !self
                    .config
                    .supported_video_formats
                    .contains(&content_type.to_string())
                {
                    anyhow::bail!("不支持的视频格式: {}", content_type);
                }
            }
            MediaType::Audio => {
                if !self
                    .config
                    .supported_audio_formats
                    .contains(&content_type.to_string())
                {
                    anyhow::bail!("不支持的音频格式: {}", content_type);
                }
            }
            MediaType::Unknown => {
                anyhow::bail!("未知的媒体类型: {}", content_type);
            }
        }
        Ok(())
    }

    /// 计算文件哈希
    fn calculate_file_hash(&self, data: &Bytes) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// 创建数据库记录
    async fn create_database_record(
        &self,
        request: &MediaUploadRequest,
        media_type: &MediaType,
        _file_hash: &str,
    ) -> Result<MediaAttachment> {
        debug!("创建媒体附件数据库记录");

        let (focus_x, focus_y) = if let Some(ref focus) = request.focus {
            (Some(focus.x), Some(focus.y))
        } else {
            (None, None)
        };

        let row = sqlx::query(
            r#"
            INSERT INTO media_attachments (
                account_id, type, file_name, file_size, file_content_type,
                description, focus_x, focus_y, processing_status, meta
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, created_at, updated_at
            "#,
        )
        .bind(request.account_id)
        .bind(media_type.to_string())
        .bind(&request.file_name)
        .bind(request.file_data.len() as i64)
        .bind(&request.content_type)
        .bind(&request.description)
        .bind(focus_x)
        .bind(focus_y)
        .bind(ProcessingStatus::Pending.to_string())
        .bind(serde_json::json!({}))
        .fetch_one(&self.pool)
        .await
        .context("创建媒体附件记录失败")?;

        let meta = MediaMetadata {
            focus: request.focus.clone(),
            original: None,
            small: None,
            duration: None,
            fps: None,
            audio_encode: None,
            audio_bitrate: None,
            audio_channels: None,
            bitrate: None,
        };

        Ok(MediaAttachment {
            id: row.get("id"),
            account_id: request.account_id,
            status_id: None,
            media_type: media_type.clone(),
            url: None,
            preview_url: None,
            remote_url: None,
            file_name: Some(request.file_name.clone()),
            file_size: Some(request.file_data.len() as i64),
            file_content_type: Some(request.content_type.clone()),
            meta,
            description: request.description.clone(),
            blurhash: None,
            processing_status: ProcessingStatus::Pending,
            focus_x,
            focus_y,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// 同步处理图片
    async fn process_image_sync(
        &self,
        attachment_id: i64,
        request: &MediaUploadRequest,
    ) -> Result<MediaAttachment> {
        debug!("同步处理图片，ID: {}", attachment_id);

        // 更新处理状态
        self.update_processing_status(attachment_id, ProcessingStatus::Processing)
            .await?;

        // 处理图片
        let result = self.process_image(attachment_id, request).await;

        match result {
            Ok(attachment) => {
                self.update_processing_status(attachment_id, ProcessingStatus::Processed)
                    .await?;
                Ok(attachment)
            }
            Err(e) => {
                error!("图片处理失败: {}", e);
                self.update_processing_status(attachment_id, ProcessingStatus::Failed)
                    .await?;
                Err(e)
            }
        }
    }

    /// 异步处理媒体
    async fn process_media_async(
        &self,
        attachment_id: i64,
        request: MediaUploadRequest,
    ) -> Result<()> {
        debug!("异步处理媒体，ID: {}", attachment_id);

        // 更新处理状态
        self.update_processing_status(attachment_id, ProcessingStatus::Processing)
            .await?;

        let result = match self.detect_media_type(&request.content_type)? {
            MediaType::Image | MediaType::Gifv => self.process_image(attachment_id, &request).await,
            MediaType::Video => self.process_video(attachment_id, &request).await,
            MediaType::Audio => self.process_audio(attachment_id, &request).await,
            MediaType::Unknown => {
                anyhow::bail!("无法处理未知媒体类型");
            }
        };

        match result {
            Ok(_) => {
                self.update_processing_status(attachment_id, ProcessingStatus::Processed)
                    .await?;
                info!("媒体处理完成，ID: {}", attachment_id);
            }
            Err(e) => {
                error!("媒体处理失败，ID: {}, 错误: {}", attachment_id, e);
                self.update_processing_status(attachment_id, ProcessingStatus::Failed)
                    .await?;
                return Err(e);
            }
        }

        Ok(())
    }

    /// 处理图片
    async fn process_image(
        &self,
        attachment_id: i64,
        request: &MediaUploadRequest,
    ) -> Result<MediaAttachment> {
        debug!("处理图片，ID: {}", attachment_id);

        // 加载图片
        let img = image::load_from_memory(&request.file_data).context("无法加载图片数据")?;

        let (width, height) = (img.width(), img.height());
        info!("图片尺寸: {}x{}", width, height);

        // 生成文件路径
        let file_id = format!("{:016x}", attachment_id);
        let original_path = self
            .config
            .media_root
            .join("original")
            .join(format!("{}.jpg", file_id));
        let thumbnail_path = self
            .config
            .media_root
            .join("small")
            .join(format!("{}.jpg", file_id));

        // 保存原始图片
        let original_img = if request.content_type == "image/jpeg" {
            img.clone()
        } else {
            // 转换为 JPEG 格式
            img.clone()
        };

        original_img
            .save_with_format(&original_path, ImageFormat::Jpeg)
            .context("保存原始图片失败")?;

        // 生成缩略图
        let thumbnail_img = self.generate_thumbnail(&img, 400, 400)?;
        thumbnail_img
            .save_with_format(&thumbnail_path, ImageFormat::Jpeg)
            .context("保存缩略图失败")?;

        let thumbnail_size = (thumbnail_img.width(), thumbnail_img.height());

        // 生成 BlurHash
        let blurhash = self.generate_blurhash(&img)?;

        // 构建元数据
        let meta = MediaMetadata {
            focus: request.focus.clone(),
            original: Some(MediaDimensions::new(width, height)),
            small: Some(MediaDimensions::new(thumbnail_size.0, thumbnail_size.1)),
            duration: None,
            fps: None,
            audio_encode: None,
            audio_bitrate: None,
            audio_channels: None,
            bitrate: None,
        };

        // 生成 URL
        let original_url = format!("{}/media/original/{}.jpg", self.config.base_url, file_id);
        let preview_url = format!("{}/media/small/{}.jpg", self.config.base_url, file_id);

        // 更新数据库记录
        let row = sqlx::query(
            r#"
            UPDATE media_attachments
            SET url = $1, preview_url = $2, meta = $3, blurhash = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(&original_url)
        .bind(&preview_url)
        .bind(serde_json::to_value(&meta)?)
        .bind(&blurhash)
        .bind(attachment_id)
        .fetch_one(&self.pool)
        .await
        .context("更新媒体附件记录失败")?;

        self.row_to_attachment(row)
    }

    /// 生成缩略图
    fn generate_thumbnail(
        &self,
        img: &DynamicImage,
        max_width: u32,
        max_height: u32,
    ) -> Result<DynamicImage> {
        let (width, height) = (img.width(), img.height());

        // 计算缩放比例
        let scale_x = max_width as f32 / width as f32;
        let scale_y = max_height as f32 / height as f32;
        let scale = scale_x.min(scale_y);

        if scale >= 1.0 {
            // 不需要缩放
            return Ok(img.clone());
        }

        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;

        Ok(img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3))
    }

    /// 生成 BlurHash
    fn generate_blurhash(&self, img: &DynamicImage) -> Result<String> {
        // 缩小图片以提高 BlurHash 生成速度
        let small_img = img.resize(32, 32, image::imageops::FilterType::Nearest);
        let rgb_img = small_img.to_rgb8();

        let (width, height) = rgb_img.dimensions();
        let pixels: Vec<u8> = rgb_img.pixels().flat_map(|p| [p[0], p[1], p[2]]).collect();

        Ok(blurhash::encode(4, 4, width, height, &pixels))
    }

    /// 处理视频（占位符实现）
    async fn process_video(
        &self,
        attachment_id: i64,
        request: &MediaUploadRequest,
    ) -> Result<MediaAttachment> {
        warn!("视频处理功能尚未实现，ID: {}", attachment_id);

        // 简单保存文件并返回基本信息
        let file_id = format!("{:016x}", attachment_id);
        let file_path = self
            .config
            .media_root
            .join("original")
            .join(format!("{}.mp4", file_id));

        let mut file = tokio::fs::File::create(&file_path)
            .await
            .context("创建视频文件失败")?;
        file.write_all(&request.file_data)
            .await
            .context("写入视频文件失败")?;

        let original_url = format!("{}/media/original/{}.mp4", self.config.base_url, file_id);

        // 更新数据库
        let row = sqlx::query(
            "UPDATE media_attachments SET url = $1, updated_at = NOW() WHERE id = $2 RETURNING *",
        )
        .bind(&original_url)
        .bind(attachment_id)
        .fetch_one(&self.pool)
        .await
        .context("更新视频附件记录失败")?;

        self.row_to_attachment(row)
    }

    /// 处理音频（占位符实现）
    async fn process_audio(
        &self,
        attachment_id: i64,
        request: &MediaUploadRequest,
    ) -> Result<MediaAttachment> {
        warn!("音频处理功能尚未实现，ID: {}", attachment_id);

        // 简单保存文件并返回基本信息
        let file_id = format!("{:016x}", attachment_id);
        let extension = mime_guess::from_path(&request.file_name)
            .first()
            .and_then(|mime| {
                mime.subtype()
                    .as_str()
                    .split('+')
                    .next()
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "mp3".to_string());

        let file_path = self
            .config
            .media_root
            .join("original")
            .join(format!("{}.{}", file_id, extension));

        let mut file = tokio::fs::File::create(&file_path)
            .await
            .context("创建音频文件失败")?;
        file.write_all(&request.file_data)
            .await
            .context("写入音频文件失败")?;

        let original_url = format!(
            "{}/media/original/{}.{}",
            self.config.base_url, file_id, extension
        );

        // 更新数据库
        let row = sqlx::query(
            "UPDATE media_attachments SET url = $1, updated_at = NOW() WHERE id = $2 RETURNING *",
        )
        .bind(&original_url)
        .bind(attachment_id)
        .fetch_one(&self.pool)
        .await
        .context("更新音频附件记录失败")?;

        self.row_to_attachment(row)
    }

    /// 更新处理状态
    async fn update_processing_status(
        &self,
        attachment_id: i64,
        status: ProcessingStatus,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE media_attachments SET processing_status = $1, updated_at = NOW() WHERE id = $2",
        )
        .bind(status.to_string())
        .bind(attachment_id)
        .execute(&self.pool)
        .await
        .context("更新处理状态失败")?;

        debug!(
            "更新媒体处理状态，ID: {}, 状态: {:?}",
            attachment_id, status
        );
        Ok(())
    }

    /// 将数据库行转换为 MediaAttachment
    fn row_to_attachment(&self, row: sqlx::postgres::PgRow) -> Result<MediaAttachment> {
        let meta_json: serde_json::Value = row
            .try_get("meta")
            .unwrap_or_else(|_| serde_json::json!({}));
        let meta: MediaMetadata = serde_json::from_value(meta_json).unwrap_or(MediaMetadata {
            focus: None,
            original: None,
            small: None,
            duration: None,
            fps: None,
            audio_encode: None,
            audio_bitrate: None,
            audio_channels: None,
            bitrate: None,
        });

        let media_type_str: String = row.try_get("type")?;
        let processing_status_str: String = row.try_get("processing_status")?;

        Ok(MediaAttachment {
            id: row.try_get("id")?,
            account_id: row.try_get("account_id")?,
            status_id: row.try_get("status_id")?,
            media_type: MediaType::from(media_type_str.as_str()),
            url: row.try_get("url")?,
            preview_url: row.try_get("preview_url")?,
            remote_url: row.try_get("remote_url")?,
            file_name: row.try_get("file_name")?,
            file_size: row.try_get("file_size")?,
            file_content_type: row.try_get("file_content_type")?,
            meta,
            description: row.try_get("description")?,
            blurhash: row.try_get("blurhash")?,
            processing_status: ProcessingStatus::from(processing_status_str.as_str()),
            focus_x: row.try_get("focus_x")?,
            focus_y: row.try_get("focus_y")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// 获取媒体附件
    pub async fn get_media_attachment(
        &self,
        attachment_id: i64,
        account_id: i64,
    ) -> Result<MediaAttachment> {
        debug!(
            "获取媒体附件，ID: {}, 用户ID: {}",
            attachment_id, account_id
        );

        let row = sqlx::query("SELECT * FROM media_attachments WHERE id = $1 AND account_id = $2")
            .bind(attachment_id)
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
            .context("媒体附件不存在或无权访问")?;

        self.row_to_attachment(row)
    }

    /// 更新媒体附件
    pub async fn update_media_attachment(
        &self,
        attachment_id: i64,
        account_id: i64,
        description: Option<String>,
        focus: Option<FocalPoint>,
    ) -> Result<MediaAttachment> {
        debug!("更新媒体附件，ID: {}", attachment_id);

        let (focus_x, focus_y) = if let Some(ref focus) = focus {
            (Some(focus.x), Some(focus.y))
        } else {
            (None, None)
        };

        let row = sqlx::query(
            r#"
            UPDATE media_attachments
            SET description = COALESCE($1, description),
                focus_x = COALESCE($2, focus_x),
                focus_y = COALESCE($3, focus_y),
                updated_at = NOW()
            WHERE id = $4 AND account_id = $5
            RETURNING *
            "#,
        )
        .bind(description)
        .bind(focus_x)
        .bind(focus_y)
        .bind(attachment_id)
        .bind(account_id)
        .fetch_one(&self.pool)
        .await
        .context("更新媒体附件失败")?;

        self.row_to_attachment(row)
    }

    /// 删除媒体附件
    pub async fn delete_media_attachment(&self, attachment_id: i64, account_id: i64) -> Result<()> {
        debug!("删除媒体附件，ID: {}", attachment_id);

        // 获取文件信息
        let attachment = self.get_media_attachment(attachment_id, account_id).await?;

        // 删除数据库记录
        let result = sqlx::query("DELETE FROM media_attachments WHERE id = $1 AND account_id = $2")
            .bind(attachment_id)
            .bind(account_id)
            .execute(&self.pool)
            .await
            .context("删除媒体附件记录失败")?;

        if result.rows_affected() == 0 {
            anyhow::bail!("媒体附件不存在或无权删除");
        }

        // 删除文件（异步执行，不阻塞响应）
        if let Some(url) = attachment.url {
            let config = self.config.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::delete_media_files(&config, &url).await {
                    error!("删除媒体文件失败: {}", e);
                }
            });
        }

        info!("媒体附件删除成功，ID: {}", attachment_id);
        Ok(())
    }

    /// 删除媒体文件
    async fn delete_media_files(config: &StorageConfig, url: &str) -> Result<()> {
        // 从 URL 提取文件名
        if let Some(file_name) = url.split('/').next_back() {
            let original_path = config.media_root.join("original").join(file_name);
            let small_path = config.media_root.join("small").join(file_name);

            // 删除原始文件
            if original_path.exists() {
                fs::remove_file(&original_path)
                    .await
                    .with_context(|| format!("删除原始文件失败: {:?}", original_path))?;
            }

            // 删除缩略图
            if small_path.exists() {
                fs::remove_file(&small_path)
                    .await
                    .with_context(|| format!("删除缩略图失败: {:?}", small_path))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    /// 创建测试用的 1x1 像素 PNG 图片
    fn create_test_image() -> Bytes {
        // 1x1 像素的透明 PNG 图片
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0B, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];
        Bytes::from(png_data)
    }

    #[test]
    fn test_media_type_conversion() {
        assert_eq!(MediaType::from("image"), MediaType::Image);
        assert_eq!(MediaType::from("video"), MediaType::Video);
        assert_eq!(MediaType::from("audio"), MediaType::Audio);
        assert_eq!(MediaType::from("unknown"), MediaType::Unknown);

        assert_eq!(MediaType::Image.to_string(), "image");
        assert_eq!(MediaType::Video.to_string(), "video");
    }

    #[test]
    fn test_processing_status_conversion() {
        assert_eq!(ProcessingStatus::from("pending"), ProcessingStatus::Pending);
        assert_eq!(
            ProcessingStatus::from("processing"),
            ProcessingStatus::Processing
        );
        assert_eq!(
            ProcessingStatus::from("processed"),
            ProcessingStatus::Processed
        );
        assert_eq!(ProcessingStatus::from("failed"), ProcessingStatus::Failed);

        assert_eq!(ProcessingStatus::Pending.to_string(), "pending");
        assert_eq!(ProcessingStatus::Processing.to_string(), "processing");
    }

    #[test]
    fn test_focal_point() {
        let focus = FocalPoint::new(0.5, -0.3).unwrap();
        assert_eq!(focus.x, 0.5);
        assert_eq!(focus.y, -0.3);

        let focus_str = focus.to_string();
        assert_eq!(focus_str, "0.5,-0.3");

        let parsed = FocalPoint::from_string(&focus_str).unwrap();
        assert_eq!(parsed.x, focus.x);
        assert_eq!(parsed.y, focus.y);

        // 测试边界值
        assert!(FocalPoint::new(-1.1, 0.0).is_err());
        assert!(FocalPoint::new(0.0, 1.1).is_err());
        assert!(FocalPoint::new(-1.0, 1.0).is_ok());
    }

    #[test]
    fn test_media_dimensions() {
        let dims = MediaDimensions::new(1920, 1080);
        assert_eq!(dims.width, 1920);
        assert_eq!(dims.height, 1080);
        assert_eq!(dims.size, "1920x1080");
        assert!((dims.aspect - 1.7777777777777777).abs() < f64::EPSILON);

        // 测试零高度
        let dims_zero = MediaDimensions::new(100, 0);
        assert_eq!(dims_zero.aspect, 0.0);
    }

    #[test]
    fn test_storage_config_default() {
        let config = StorageConfig::default();
        assert!(config
            .supported_image_formats
            .contains(&"image/jpeg".to_string()));
        assert!(config
            .supported_video_formats
            .contains(&"video/mp4".to_string()));
        assert!(config
            .supported_audio_formats
            .contains(&"audio/mpeg".to_string()));
        assert_eq!(config.max_file_size, 40 * 1024 * 1024);
    }
}
