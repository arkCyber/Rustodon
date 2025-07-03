-- 创建媒体附件表
-- 用于存储用户上传的图片、视频、音频等媒体文件信息
-- 对应 Mastodon 的 media_attachments 表

CREATE TABLE media_attachments (
    -- 主键，媒体附件的唯一标识符
    id BIGSERIAL PRIMARY KEY,
    
    -- 关联的用户账户ID，外键引用 users 表
    account_id BIGINT NOT NULL,
    
    -- 关联的状态ID，可为空（上传但未发布的媒体）
    status_id BIGINT,
    
    -- 媒体类型：image, video, gifv, audio, unknown
    type VARCHAR(20) NOT NULL DEFAULT 'unknown',
    
    -- 原始文件的URL路径
    url TEXT,
    
    -- 预览图/缩略图的URL路径
    preview_url TEXT,
    
    -- 远程媒体的原始URL（用于联邦实例）
    remote_url TEXT,
    
    -- 原始文件名
    file_name TEXT,
    
    -- 文件大小（字节）
    file_size BIGINT,
    
    -- MIME类型
    file_content_type VARCHAR(255),
    
    -- 媒体元数据（JSON格式）
    -- 包含尺寸、时长、焦点等信息
    meta JSONB DEFAULT '{}',
    
    -- 媒体描述（用于无障碍访问）
    description TEXT,
    
    -- BlurHash 字符串（用于生成模糊预览）
    blurhash VARCHAR(255),
    
    -- 处理状态：pending, processing, processed, failed
    processing_status VARCHAR(20) DEFAULT 'pending',
    
    -- 焦点坐标 X (-1.0 到 1.0)
    focus_x FLOAT,
    
    -- 焦点坐标 Y (-1.0 到 1.0)
    focus_y FLOAT,
    
    -- 创建时间
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- 更新时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 创建索引以提高查询性能
CREATE INDEX idx_media_attachments_account_id ON media_attachments(account_id);
CREATE INDEX idx_media_attachments_status_id ON media_attachments(status_id);
CREATE INDEX idx_media_attachments_type ON media_attachments(type);
CREATE INDEX idx_media_attachments_created_at ON media_attachments(created_at);
CREATE INDEX idx_media_attachments_processing_status ON media_attachments(processing_status);

-- 添加外键约束
-- ALTER TABLE media_attachments ADD CONSTRAINT fk_media_attachments_account_id 
--     FOREIGN KEY (account_id) REFERENCES users(id) ON DELETE CASCADE;
-- ALTER TABLE media_attachments ADD CONSTRAINT fk_media_attachments_status_id 
--     FOREIGN KEY (status_id) REFERENCES statuses(id) ON DELETE CASCADE;

-- 添加检查约束
ALTER TABLE media_attachments ADD CONSTRAINT chk_media_type 
    CHECK (type IN ('image', 'video', 'gifv', 'audio', 'unknown'));

ALTER TABLE media_attachments ADD CONSTRAINT chk_processing_status 
    CHECK (processing_status IN ('pending', 'processing', 'processed', 'failed'));

ALTER TABLE media_attachments ADD CONSTRAINT chk_focus_x_range 
    CHECK (focus_x IS NULL OR (focus_x >= -1.0 AND focus_x <= 1.0));

ALTER TABLE media_attachments ADD CONSTRAINT chk_focus_y_range 
    CHECK (focus_y IS NULL OR (focus_y >= -1.0 AND focus_y <= 1.0));

-- 添加触发器以自动更新 updated_at 字段
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_media_attachments_updated_at 
    BEFORE UPDATE ON media_attachments 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
