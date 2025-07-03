# Rustodon Project Summary

**作者**: arkSong (arksong2018@gmail.com)
**项目**: rustodon
**创建日期**: 2025-07-03
**状态**: 开发中

## 项目概述

Rustodon是一个用Rust语言实现的高性能Mastodon服务器后端，目标是100%兼容原始Mastodon的功能，同时提供更好的性能和类型安全。

## 项目结构

### 核心模块

```
rustodon/
├── rustodon-core/           # 核心类型和特征
├── rustodon-db/            # 数据库操作
├── rustodon-api/           # HTTP API层
├── rustodon-auth/          # 认证和授权
├── rustodon-activitypub/   # ActivityPub协议
├── rustodon-workers/       # 后台任务处理
├── rustodon-search/        # 搜索功能
├── rustodon-mailer/        # 邮件功能
├── rustodon-admin/         # 管理界面
├── rustodon-config/        # 配置管理
├── rustodon-logging/       # 日志基础设施
├── rustodon-metrics/       # 指标和监控
├── rustodon-cache/         # 缓存层
├── rustodon-queue/         # 消息队列
├── rustodon-storage/       # 文件存储
├── rustodon-notifications/ # 通知系统
├── rustodon-media/         # 媒体处理
├── rustodon-federation/    # 联邦逻辑
├── rustodon-webhooks/      # Webhook处理
├── rustodon-scheduler/     # 计划任务
├── rustodon-migrations/    # 数据库迁移
├── rustodon-cli/           # 命令行界面
└── rustodon-server/        # 主服务器二进制文件
```

### 用户管理模块

```
rustodon-accounts/          # 账户管理
rustodon-sessions/          # 会话管理
rustodon-user-settings/     # 用户设置
rustodon-devices/           # 设备管理
rustodon-applications/      # 应用程序
rustodon-access-tokens/     # 访问令牌
rustodon-access-grants/     # 访问授权
```

### 社交功能模块

```
rustodon-statuses/          # 状态管理
rustodon-reblogs/           # 转推功能
rustodon-favourites/        # 收藏功能
rustodon-follows/           # 关注功能
rustodon-follow-requests/   # 关注请求
rustodon-mentions/          # 提及功能
rustodon-polls/             # 投票功能
rustodon-conversations/     # 对话功能
rustodon-lists/             # 列表功能
rustodon-bookmarks/         # 书签功能
```

### 内容管理模块

```
rustodon-media/             # 媒体处理
rustodon-preview-cards/     # 预览卡片
rustodon-custom-emojis/     # 自定义表情
rustodon-tags/              # 标签管理
rustodon-tag-follows/       # 标签关注
rustodon-trends/            # 趋势话题
rustodon-announcements/     # 公告功能
```

### 安全和审核模块

```
rustodon-blocks/            # 屏蔽功能
rustodon-mutes/             # 静音功能
rustodon-reports/           # 举报功能
rustodon-filters/           # 过滤器
rustodon-ip-blocks/         # IP屏蔽
rustodon-email-domain-blocks/ # 邮箱域名屏蔽
rustodon-canonical-email-blocks/ # 规范邮箱屏蔽
rustodon-account-warnings/  # 账户警告
rustodon-account-notes/     # 账户备注
rustodon-account-moderation-notes/ # 账户审核备注
rustodon-appeals/           # 申诉功能
```

### 高级功能模块

```
rustodon-groups/            # 群组功能
rustodon-encrypted-messages/ # 加密消息
rustodon-webauthn-credentials/ # WebAuthn凭证
rustodon-annual-reports/    # 年度报告
rustodon-software-updates/  # 软件更新
rustodon-severed-relationships/ # 断绝关系
rustodon-follow-recommendation-suppressions/ # 关注推荐抑制
rustodon-account-suggestions/ # 账户建议
rustodon-account-aliases/   # 账户别名
rustodon-account-conversations/ # 账户对话
rustodon-account-deletion-requests/ # 账户删除请求
```

## 技术栈

### 核心技术
- **语言**: Rust 1.77+
- **异步运行时**: Tokio
- **Web框架**: Axum
- **数据库**: PostgreSQL (SQLx)
- **序列化**: Serde
- **日志**: Tracing
- **错误处理**: Thiserror

### 开发工具
- **包管理**: Cargo
- **测试**: Cargo test
- **代码检查**: Clippy
- **格式化**: rustfmt
- **文档**: rustdoc

### 部署技术
- **容器化**: Docker
- **编排**: Docker Compose
- **数据库**: PostgreSQL 15+
- **缓存**: Redis (可选)
- **搜索**: Elasticsearch (可选)

## 开发状态

### 已完成
- ✅ 项目结构设计
- ✅ 模块化架构
- ✅ 基础配置
- ✅ Docker支持
- ✅ API测试框架
- ✅ 数据库设计
- ✅ 认证系统设计

### 进行中
- 🔄 核心模块实现
- 🔄 API端点开发
- 🔄 数据库迁移
- 🔄 测试覆盖

### 计划中
- 📋 完整API实现
- 📋 前端界面
- 📋 性能优化
- 📋 安全审计
- 📋 文档完善

## API测试

项目包含完整的API测试框架：

### 测试脚本
- `comprehensive_curl_test.sh` - 基础API测试
- `advanced_api_test.sh` - 高级API测试 (40个端点)
- `simple_test_server.py` - Python测试服务器

### 测试覆盖
- ✅ 健康检查
- ✅ 用户认证
- ✅ 状态管理
- ✅ 社交功能
- ✅ 媒体上传
- ✅ 搜索功能
- ✅ 通知系统
- ✅ 时间线功能

### 测试报告
详细测试报告请查看 `API_TEST_REPORT.md`

## 部署指南

### 开发环境
```bash
# 克隆仓库
git clone https://github.com/yourusername/rustodon.git
cd rustodon

# 设置数据库
./setup_database.sh

# 构建项目
cargo build

# 运行测试
cargo test
```

### Docker部署
```bash
# 使用Docker Compose
docker-compose up -d

# 或使用简化版本
docker-compose -f docker-compose.simple.yml up -d
```

### 生产部署
```bash
# 构建发布版本
cargo build --release

# 运行迁移
cargo run -p rustodon-migrations

# 启动服务器
./target/release/rustodon-server
```

## 贡献指南

### 开发流程
1. Fork项目
2. 创建功能分支
3. 编写代码和测试
4. 运行测试套件
5. 提交Pull Request

### 代码标准
- 遵循Rust最佳实践
- 使用async/await进行I/O操作
- 实现适当的错误处理
- 编写完整的文档
- 添加单元测试

### 提交规范
```
feat: 添加新功能
fix: 修复bug
docs: 更新文档
style: 代码格式调整
refactor: 代码重构
test: 添加测试
chore: 构建过程或辅助工具的变动
```

## 许可证

本项目采用MIT许可证 - 详见 [LICENSE](LICENSE) 文件

## 联系方式

- **作者**: arkSong
- **邮箱**: arksong2018@gmail.com
- **GitHub**: https://github.com/arksong/rustodon

## 致谢

- 原始Mastodon项目提供的API规范
- Rust社区提供的优秀工具和库
- 所有为项目做出贡献的开发者

---

**最后更新**: 2025-07-03
**版本**: 0.1.0
**状态**: 活跃开发中
