# Rustodon 自动化测试脚本

本目录包含用于自动化测试 Rustodon API 的各种脚本。

## 脚本说明

### 1. `simple_api_test.sh` - 简单 API 测试
**功能**: 快速测试基础 API 功能
**特点**:
- 轻量级，执行速度快
- 测试当前已实现的功能
- 适合日常开发测试

**使用方法**:
```bash
./scripts/simple_api_test.sh
```

### 2. `api_test_suite.sh` - 全面 API 测试套件
**功能**: 完整的 API 功能测试，包括注册、登录、发帖等
**特点**:
- 测试所有 API 端点
- 包含用户流程测试
- 详细的测试报告

**使用方法**:
```bash
./scripts/api_test_suite.sh
```

### 3. `automated_test_workflow.sh` - 自动化测试工作流
**功能**: 一键启动服务器、运行测试、生成报告
**特点**:
- 完整的自动化流程
- 自动启动和停止服务器
- 生成详细的测试报告
- 包含性能测试

**使用方法**:
```bash
./scripts/automated_test_workflow.sh
```

## 测试覆盖范围

### 基础 API 测试
- ✅ 根路径 (`/`)
- ✅ 实例信息 (`/api/v1/instance`)
- ✅ 状态列表 (`/api/v1/statuses`)
- ✅ 账户列表 (`/api/v1/accounts`)

### 用户功能测试
- 🔄 用户注册 (`POST /api/v1/accounts`)
- 🔄 用户认证 (`POST /api/v1/oauth/token`)
- 🔄 用户资料 (`GET /api/v1/accounts/verify_credentials`)

### 状态功能测试
- ✅ 状态发布 (`POST /api/v1/statuses`)
- 🔄 状态获取 (`GET /api/v1/statuses/:id`)
- 🔄 状态点赞 (`POST /api/v1/statuses/:id/favourite`)
- 🔄 状态转发 (`POST /api/v1/statuses/:id/reblog`)

### 用户关系测试
- 🔄 关注用户 (`POST /api/v1/accounts/:id/follow`)
- 🔄 获取关注者 (`GET /api/v1/accounts/:id/followers`)
- 🔄 获取关注列表 (`GET /api/v1/accounts/:id/following`)

### 通知测试
- 🔄 获取通知 (`GET /api/v1/notifications`)
- 🔄 清除通知 (`POST /api/v1/notifications/clear`)

### 搜索测试
- 🔄 搜索账户 (`GET /api/v1/accounts/search`)
- 🔄 搜索状态 (`GET /api/v1/search`)

### 错误处理测试
- ✅ 404 错误处理
- 🔄 400 错误处理

### 性能测试
- ✅ 响应时间测试
- ✅ 并发请求测试

## 状态说明

- ✅ **已实现**: 功能已完全实现并通过测试
- 🔄 **部分实现**: 功能部分实现或返回 404（待开发）
- ❌ **未实现**: 功能尚未实现

## 测试结果示例

### 成功测试结果
```
=== Rustodon API 测试 ===
服务器地址: http://127.0.0.1:3000

1. 基础 API 测试
测试 根路径... ✓ 通过 (200)
测试 实例信息... ✓ 通过 (200)
测试 状态列表... ✓ 通过 (200)
测试 账户列表... ✓ 通过 (200)

=== 测试结果汇总 ===
总测试数: 16
通过: 6
失败: 10
成功率: 37%
⚠️  部分测试失败，这是正常的，因为某些功能可能尚未实现
```

## 环境要求

### 系统依赖
- Rust 和 Cargo
- PostgreSQL 客户端
- curl
- bash

### 环境变量
确保 `.env` 文件包含正确的数据库连接信息：
```env
DATABASE_URL=postgresql://rustodon:password@localhost:5432/rustodon_test
```

## 故障排除

### 常见问题

1. **服务器启动失败**
   ```bash
   # 检查端口是否被占用
   lsof -i :3000

   # 停止现有进程
   pkill -f rustodon-server
   ```

2. **数据库连接失败**
   ```bash
   # 检查数据库状态
   psql -h localhost -U postgres -c "SELECT version();"

   # 重新创建测试数据库
   psql -h localhost -U postgres -c "DROP DATABASE IF EXISTS rustodon_test;"
   psql -h localhost -U postgres -c "CREATE DATABASE rustodon_test;"
   ```

3. **测试脚本权限问题**
   ```bash
   # 添加执行权限
   chmod +x scripts/*.sh
   ```

### 日志文件

- `test_results_YYYYMMDD_HHMMSS.log`: 详细测试日志
- `api_test_report_YYYYMMDD_HHMMSS.md`: 测试报告
- `server.log`: 服务器运行日志

## 开发建议

1. **日常开发**: 使用 `simple_api_test.sh` 进行快速测试
2. **功能验证**: 使用 `api_test_suite.sh` 进行完整测试
3. **CI/CD**: 使用 `automated_test_workflow.sh` 进行自动化测试

## 扩展测试

如需添加新的测试用例，可以：

1. 在现有脚本中添加新的测试函数
2. 创建专门的测试脚本
3. 使用 curl 命令直接测试特定端点

示例：
```bash
# 测试特定端点
curl -v http://127.0.0.1:3000/api/v1/instance

# 测试 POST 请求
curl -v -X POST http://127.0.0.1:3000/api/v1/statuses \
  -H "Content-Type: application/json" \
  -d '{"status":"测试状态"}'
```

---

*最后更新: 2025-07-03*
*作者: arkSong (arksong2018@gmail.com)*
