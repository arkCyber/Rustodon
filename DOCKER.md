# Rustodon Docker 环境指南

## 概述

本文档介绍如何使用 Docker 运行 Rustodon 项目。我们提供了多种 Docker 配置选项来满足不同的需求。

## 快速开始

### 1. 简单环境（推荐用于开发）

使用简化的 docker-compose 配置，只包含必要的服务：

```bash
# 构建并启动服务
docker-compose -f docker-compose.simple.yml up --build

# 后台运行
docker-compose -f docker-compose.simple.yml up -d --build

# 停止服务
docker-compose -f docker-compose.simple.yml down
```

### 2. 完整环境

使用完整的 docker-compose 配置，包含数据库、Redis、Nginx 等：

```bash
# 构建并启动所有服务
docker-compose up --build

# 后台运行
docker-compose up -d --build

# 停止所有服务
docker-compose down
```

### 3. 使用自动化脚本

我们提供了自动化脚本来简化 Docker 操作：

```bash
# 构建镜像
./scripts/docker-setup.sh build

# 启动服务
./scripts/docker-setup.sh start

# 停止服务
./scripts/docker-setup.sh stop

# 重启服务
./scripts/docker-setup.sh restart

# 查看日志
./scripts/docker-setup.sh logs

# 查看状态
./scripts/docker-setup.sh status

# 清理资源
./scripts/docker-setup.sh clean
```

## 服务说明

### 简单环境服务

- **rustodon-server**: Rustodon 主服务 (端口 3000)
- **rustodon-db**: PostgreSQL 数据库 (端口 5432)

### 完整环境服务

- **rustodon-server**: Rustodon 主服务 (端口 3000)
- **rustodon-db**: PostgreSQL 数据库 (端口 5432)
- **rustodon-redis**: Redis 缓存 (端口 6379)
- **rustodon-nginx**: Nginx 反向代理 (端口 80, 443)

## 环境变量

### 必需环境变量

- `DATABASE_URL`: PostgreSQL 数据库连接字符串
- `RUST_LOG`: 日志级别 (默认: info)

### 可选环境变量

- `REDIS_URL`: Redis 连接字符串
- `RUST_BACKTRACE`: 启用回溯跟踪 (1 或 0)

## 数据持久化

### 数据库数据

PostgreSQL 数据存储在 Docker 卷中：
- 简单环境: `pgdata`
- 完整环境: `pgdata` 和 `redisdata`

### 配置文件

可以将配置文件挂载到容器中：
```yaml
volumes:
  - ./config:/app/config:ro
  - ./logs:/app/logs
```

## 故障排除

### 1. 网络连接问题

如果遇到网络超时问题，可以：

1. 检查 Docker 网络设置
2. 配置 Docker 镜像加速器
3. 使用本地构建的 Dockerfile

### 2. 端口冲突

如果端口被占用，可以修改 docker-compose.yml 中的端口映射：

```yaml
ports:
  - "3001:3000"  # 将主机的 3001 端口映射到容器的 3000 端口
```

### 3. 权限问题

如果遇到权限问题，确保：

1. Docker 守护进程正在运行
2. 用户有足够的权限运行 Docker 命令
3. 脚本文件有执行权限

### 4. 构建失败

如果构建失败，可以：

1. 清理 Docker 缓存：`docker system prune -a`
2. 使用 `--no-cache` 选项重新构建
3. 检查网络连接

## 开发模式

### 本地开发

对于本地开发，建议：

1. 使用简单环境配置
2. 将源代码挂载到容器中
3. 启用热重载功能

```yaml
volumes:
  - .:/app
  - /app/target  # 排除 target 目录
```

### 调试

启用调试模式：

```bash
# 设置环境变量
export RUST_LOG=debug
export RUST_BACKTRACE=1

# 启动服务
docker-compose up
```

## 生产部署

### 安全配置

生产环境中应该：

1. 使用强密码
2. 启用 HTTPS
3. 配置防火墙
4. 使用非 root 用户运行服务

### 性能优化

1. 使用多阶段构建
2. 配置资源限制
3. 启用健康检查
4. 使用负载均衡

## 监控和日志

### 查看日志

```bash
# 查看所有服务日志
docker-compose logs

# 查看特定服务日志
docker-compose logs rustodon

# 实时查看日志
docker-compose logs -f
```

### 健康检查

服务包含健康检查配置，可以通过以下命令查看状态：

```bash
docker-compose ps
```

## 清理和维护

### 清理资源

```bash
# 停止并删除容器
docker-compose down

# 删除卷（会丢失数据）
docker-compose down -v

# 清理所有 Docker 资源
docker system prune -a
```

### 更新镜像

```bash
# 重新构建镜像
docker-compose build --no-cache

# 拉取最新基础镜像
docker-compose pull
```

## 支持

如果遇到问题，请：

1. 检查本文档的故障排除部分
2. 查看项目 GitHub Issues
3. 联系项目维护者

---

**作者**: arkSong (arksong2018@gmail.com)
**项目**: rustodon
**最后更新**: 2024年
