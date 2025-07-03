# Rustodon 环境配置总结

## 当前状态

### ✅ 已解决的问题
1. **Workspace 配置**：已设置 resolver = "2"
2. **SQLx 版本统一**：所有 crate 使用 sqlx 0.7.3
3. **依赖管理**：添加了 prometheus、cidr 等缺失依赖
4. **语法错误**：修复了多个文件的语法错误
5. **环境变量**：设置了 DATABASE_URL 等环境变量

### ⚠️ 剩余问题

#### 1. 数据库表结构问题
- **follows 表**：缺少 `follower_id` 和 `followed_id` 列
- **users 表**：字段类型不匹配（Option<bool> vs bool, Option<i64> vs i64）

#### 2. 类型系统问题
- **rustodon-ip-blocks**：IpNetwork 和 String 类型转换问题
- **rustodon-db**：User 模型字段类型定义问题

#### 3. 未使用的导入
- 多个文件有未使用的 tracing 导入

## 环境配置

### 环境变量
```bash
DATABASE_URL=postgresql://rustodon:rustodon@localhost:5432/rustodon
REDIS_URL=redis://localhost:6379
RUST_LOG=debug
RUST_BACKTRACE=1
ENVIRONMENT=development
```

### 数据库设置
需要创建 PostgreSQL 数据库：
```sql
CREATE DATABASE rustodon;
CREATE USER rustodon WITH PASSWORD 'rustodon';
GRANT ALL PRIVILEGES ON DATABASE rustodon TO rustodon;
```

### 数据库表结构
需要创建以下表：
- users
- follows
- ip_blocks
- 其他 Mastodon 相关表

## 下一步行动

### 1. 数据库迁移
```bash
# 创建数据库迁移
cargo run -p rustodon-migrations create initial_schema

# 运行迁移
cargo run -p rustodon-migrations up
```

### 2. 修复类型问题
- 更新 User 模型字段类型
- 修复 IpBlock 模型的 IpNetwork 处理

### 3. 清理未使用的导入
```bash
cargo fix --workspace
```

### 4. 测试编译
```bash
cargo check
cargo test
```

## 项目结构

```
rustodon/
├── Cargo.toml (workspace)
├── crates/
│   ├── core/           # 核心类型和特征
│   ├── api/            # API 层
│   ├── auth/           # 认证和授权
│   ├── database/       # 数据库操作
│   ├── features/       # 功能模块
│   ├── admin/          # 管理功能
│   ├── federation/     # 联邦功能
│   ├── media/          # 媒体处理
│   ├── utils/          # 工具模块
│   ├── cli/            # 命令行工具
│   └── server/         # 服务器
├── migrations/         # 数据库迁移
├── tests/             # 测试
└── docs/              # 文档
```

## 开发命令

```bash
# 检查编译
cargo check

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 构建发布版本
cargo build --release

# 运行特定 crate
cargo run -p rustodon-server
```

## 注意事项

1. **数据库连接**：确保 PostgreSQL 服务运行且可访问
2. **环境变量**：使用 `source setup_environment.sh` 设置环境变量
3. **依赖版本**：保持所有 crate 的依赖版本一致
4. **类型安全**：注意 SQLx 查询宏的类型匹配

## 贡献指南

1. 遵循 Rust 编码规范
2. 添加适当的错误处理
3. 编写测试用例
4. 更新文档
5. 确保所有测试通过

---

**作者**: arkSong (arksong2018@gmail.com)
**项目**: Rustodon - Rust implementation of Mastodon server backend
