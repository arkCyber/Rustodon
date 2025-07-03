#!/bin/bash

#
# Rustodon API 测试脚本
#
# 这个脚本测试 Rustodon 服务器的基本 API 功能，包括：
# - 健康检查
# - 用户注册和登录
# - OAuth 应用注册
# - 状态创建和获取
# - 时间线查看
# - 状态点赞/取消点赞
#
# 使用方法：
# 1. 启动 Rustodon 服务器：cargo run -p rustodon-server
# 2. 运行测试脚本：./test_api.sh
#
# 作者：arkSong (arksong2018@gmail.com)
#

set -e  # 遇到错误时退出

# 配置
BASE_URL="http://localhost:3000"
API_BASE="$BASE_URL/api/v1"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查服务器是否运行
check_server() {
    log_info "检查服务器状态..."
    if curl -s "$API_BASE/health" > /dev/null; then
        log_success "服务器正在运行"
    else
        log_error "服务器未运行，请先启动 Rustodon 服务器"
        log_info "运行命令：cargo run -p rustodon-server"
        exit 1
    fi
}

# 测试健康检查
test_health() {
    log_info "测试健康检查端点..."
    response=$(curl -s -w "%{http_code}" "$API_BASE/health")
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "健康检查通过 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "健康检查失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试用户注册
test_register() {
    log_info "测试用户注册..."
    
    # 生成随机用户名避免冲突
    RANDOM_USER="testuser_$(date +%s)"
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE/auth/register" \
        -H "Content-Type: application/json" \
        -d "{
            \"username\": \"$RANDOM_USER\",
            \"email\": \"$RANDOM_USER@example.com\",
            \"password\": \"testpassword123\"
        }")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "201" ]; then
        log_success "用户注册成功 (HTTP $http_code)"
        echo "响应: $body"
        
        # 提取 token
        TOKEN=$(echo "$body" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
        if [ -n "$TOKEN" ]; then
            log_success "获取到认证令牌: ${TOKEN:0:20}..."
            export AUTH_TOKEN="$TOKEN"
            export TEST_USERNAME="$RANDOM_USER"
        else
            log_warning "未能提取认证令牌"
        fi
    else
        log_error "用户注册失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试用户登录
test_login() {
    log_info "测试用户登录..."
    
    if [ -z "$TEST_USERNAME" ]; then
        log_warning "跳过登录测试 - 没有测试用户"
        return
    fi
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE/auth/login" \
        -H "Content-Type: application/json" \
        -d "{
            \"username_or_email\": \"$TEST_USERNAME\",
            \"password\": \"testpassword123\"
        }")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "用户登录成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "用户登录失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试 OAuth 应用注册
test_app_registration() {
    log_info "测试 OAuth 应用注册..."
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE/apps" \
        -H "Content-Type: application/json" \
        -d '{
            "client_name": "Rustodon Test App",
            "redirect_uris": "http://localhost:3000/oauth/callback",
            "scopes": "read write follow",
            "website": "https://github.com/arkCyber/Rustodon"
        }')
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "201" ]; then
        log_success "OAuth 应用注册成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "OAuth 应用注册失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试验证凭据
test_verify_credentials() {
    log_info "测试验证凭据..."
    
    if [ -z "$AUTH_TOKEN" ]; then
        log_warning "跳过凭据验证测试 - 没有认证令牌"
        return
    fi
    
    response=$(curl -s -w "%{http_code}" -X GET "$API_BASE/accounts/verify_credentials" \
        -H "Authorization: Bearer $AUTH_TOKEN")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "凭据验证成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "凭据验证失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试创建状态
test_create_status() {
    log_info "测试创建状态..."
    
    if [ -z "$AUTH_TOKEN" ]; then
        log_warning "跳过状态创建测试 - 没有认证令牌"
        return
    fi
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE/statuses" \
        -H "Authorization: Bearer $AUTH_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{
            "status": "Hello from Rustodon! 🦀 This is a test status created via API.",
            "visibility": "public"
        }')
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "201" ]; then
        log_success "状态创建成功 (HTTP $http_code)"
        echo "响应: $body"
        
        # 提取状态 ID
        STATUS_ID=$(echo "$body" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
        if [ -n "$STATUS_ID" ]; then
            log_success "获取到状态 ID: $STATUS_ID"
            export TEST_STATUS_ID="$STATUS_ID"
        fi
    else
        log_error "状态创建失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试获取状态
test_get_status() {
    log_info "测试获取状态..."
    
    if [ -z "$TEST_STATUS_ID" ]; then
        log_warning "跳过状态获取测试 - 没有测试状态 ID"
        return
    fi
    
    response=$(curl -s -w "%{http_code}" -X GET "$API_BASE/statuses/$TEST_STATUS_ID")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "状态获取成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "状态获取失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试公共时间线
test_public_timeline() {
    log_info "测试公共时间线..."
    
    response=$(curl -s -w "%{http_code}" -X GET "$API_BASE/timelines/public")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "公共时间线获取成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "公共时间线获取失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试主页时间线
test_home_timeline() {
    log_info "测试主页时间线..."
    
    if [ -z "$AUTH_TOKEN" ]; then
        log_warning "跳过主页时间线测试 - 没有认证令牌"
        return
    fi
    
    response=$(curl -s -w "%{http_code}" -X GET "$API_BASE/timelines/home" \
        -H "Authorization: Bearer $AUTH_TOKEN")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "主页时间线获取成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "主页时间线获取失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试点赞状态
test_favourite_status() {
    log_info "测试点赞状态..."
    
    if [ -z "$AUTH_TOKEN" ] || [ -z "$TEST_STATUS_ID" ]; then
        log_warning "跳过点赞测试 - 缺少认证令牌或状态 ID"
        return
    fi
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE/statuses/$TEST_STATUS_ID/favourite" \
        -H "Authorization: Bearer $AUTH_TOKEN")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "状态点赞成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "状态点赞失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 测试取消点赞状态
test_unfavourite_status() {
    log_info "测试取消点赞状态..."
    
    if [ -z "$AUTH_TOKEN" ] || [ -z "$TEST_STATUS_ID" ]; then
        log_warning "跳过取消点赞测试 - 缺少认证令牌或状态 ID"
        return
    fi
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE/statuses/$TEST_STATUS_ID/unfavourite" \
        -H "Authorization: Bearer $AUTH_TOKEN")
    
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        log_success "取消点赞成功 (HTTP $http_code)"
        echo "响应: $body"
    else
        log_error "取消点赞失败 (HTTP $http_code)"
        echo "响应: $body"
    fi
    echo
}

# 主函数
main() {
    echo "=========================================="
    echo "         Rustodon API 测试脚本"
    echo "=========================================="
    echo
    
    # 检查服务器状态
    check_server
    
    # 运行所有测试
    test_health
    test_register
    test_login
    test_app_registration
    test_verify_credentials
    test_create_status
    test_get_status
    test_public_timeline
    test_home_timeline
    test_favourite_status
    test_unfavourite_status
    
    echo "=========================================="
    log_info "所有测试完成！"
    echo "=========================================="
}

# 运行主函数
main "$@"
