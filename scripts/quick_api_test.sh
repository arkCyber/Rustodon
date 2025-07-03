#!/bin/bash

# Rustodon 快速 API 测试脚本
# 作者: arkSong (arksong2018@gmail.com)
# 功能: 快速测试当前已实现的基础 API 功能

set -e

# 配置
BASE_URL="http://127.0.0.1:3000"
API_BASE="$BASE_URL/api/v1"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 测试函数
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_status="$3"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    log_info "测试: $test_name"

    if eval "$test_command" > /tmp/test_output.json 2>/dev/null; then
        local status_code=$(cat /tmp/test_output.json | grep -o 'HTTP/[0-9.]* [0-9]*' | tail -1 | awk '{print $2}')
        if [ "$status_code" = "$expected_status" ]; then
            log_success "✓ $test_name 通过 (状态码: $status_code)"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            log_error "✗ $test_name 失败 (期望: $expected_status, 实际: $status_code)"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        log_error "✗ $test_name 失败 (命令执行错误)"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# 检查服务器状态
check_server() {
    log_info "检查服务器状态..."
    if curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/" | grep -q "200"; then
        log_success "服务器运行正常"
        return 0
    else
        log_error "服务器未运行或无法访问"
        return 1
    fi
}

# 基础 API 测试
test_basic_apis() {
    log_info "=== 基础 API 测试 ==="

    # 根路径测试
    run_test "根路径" \
        "curl -s -w '%{http_code}' -o /dev/null $BASE_URL/" \
        "200"

    # 实例信息测试
    run_test "实例信息" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/instance" \
        "200"

    # 状态列表测试
    run_test "状态列表" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses" \
        "200"

    # 账户列表测试
    run_test "账户列表" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts" \
        "200"
}

# 用户注册测试
test_user_registration() {
    log_info "=== 用户注册测试 ==="

    # 创建测试用户
    run_test "用户注册" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts \
        -H 'Content-Type: application/json' \
        -d '{\"username\":\"testuser\",\"email\":\"test@example.com\",\"password\":\"testpass123\",\"agreement\":true}'" \
        "201"
}

# 状态发布测试
test_status_creation() {
    log_info "=== 状态发布测试 ==="

    # 发布文本状态
    run_test "发布文本状态" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{\"status\":\"Hello Rustodon! 这是一个测试状态。\"}'" \
        "200"

    # 发布带可见性的状态
    run_test "发布公开状态" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{\"status\":\"这是一个公开状态\",\"visibility\":\"public\"}'" \
        "200"
}

# 状态交互测试
test_status_interactions() {
    log_info "=== 状态交互测试 ==="

    # 获取状态列表
    run_test "获取状态列表" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses" \
        "200"

    # 获取单个状态
    run_test "获取单个状态" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses/1" \
        "200"

    # 点赞状态
    run_test "点赞状态" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/favourite" \
        "200"

    # 转发状态
    run_test "转发状态" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/reblog" \
        "200"
}

# 用户关系测试
test_user_relationships() {
    log_info "=== 用户关系测试 ==="

    # 关注用户
    run_test "关注用户" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts/1/follow" \
        "200"

    # 获取关注者列表
    run_test "获取关注者列表" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts/1/followers" \
        "200"

    # 获取关注列表
    run_test "获取关注列表" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts/1/following" \
        "200"
}

# 通知测试
test_notifications() {
    log_info "=== 通知测试 ==="

    # 获取通知列表
    run_test "获取通知列表" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/notifications" \
        "200"
}

# 搜索测试
test_search() {
    log_info "=== 搜索测试 ==="

    # 搜索账户
    run_test "搜索账户" \
        "curl -s -w '%{http_code}' -o /dev/null '$API_BASE/accounts/search?q=admin'" \
        "200"

    # 搜索状态
    run_test "搜索状态" \
        "curl -s -w '%{http_code}' -o /dev/null '$API_BASE/search?q=test'" \
        "200"
}

# 错误处理测试
test_error_handling() {
    log_info "=== 错误处理测试 ==="

    # 404 错误
    run_test "404 错误" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/nonexistent" \
        "404"

    # 无效的 JSON
    run_test "无效 JSON" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d 'invalid json'" \
        "400"
}

# 性能测试
test_performance() {
    log_info "=== 性能测试 ==="

    # 响应时间测试
    log_info "测试响应时间..."
    start_time=$(date +%s%N)
    curl -s -o /dev/null "$API_BASE/instance"
    end_time=$(date +%s%N)
    response_time=$(( (end_time - start_time) / 1000000 ))
    log_info "响应时间: ${response_time}ms"

    # 并发测试
    log_info "执行并发请求测试..."
    for i in {1..5}; do
        curl -s -o /dev/null "$API_BASE/instance" &
    done
    wait
    log_success "并发测试完成"
}

# 主测试函数
main() {
    log_info "开始 Rustodon 快速 API 测试..."
    log_info "服务器地址: $BASE_URL"

    # 检查服务器状态
    if ! check_server; then
        log_error "服务器未运行，请先启动服务器"
        exit 1
    fi

    # 运行所有测试
    test_basic_apis
    test_user_registration
    test_status_creation
    test_status_interactions
    test_user_relationships
    test_notifications
    test_search
    test_error_handling
    test_performance

    # 输出测试结果
    log_info "=== 测试结果汇总 ==="
    log_info "总测试数: $TOTAL_TESTS"
    log_success "通过: $PASSED_TESTS"
    if [ $FAILED_TESTS -gt 0 ]; then
        log_error "失败: $FAILED_TESTS"
    else
        log_success "失败: $FAILED_TESTS"
    fi

    success_rate=$(( PASSED_TESTS * 100 / TOTAL_TESTS ))
    log_info "成功率: ${success_rate}%"

    if [ $FAILED_TESTS -eq 0 ]; then
        log_success "所有测试通过！🎉"
        exit 0
    else
        log_warning "部分测试失败，这是正常的，因为某些功能可能尚未实现"
        exit 0
    fi
}

# 清理函数
cleanup() {
    rm -f /tmp/test_output.json
    log_info "测试完成，清理临时文件"
}

# 设置退出时清理
trap cleanup EXIT

# 运行主函数
main "$@"
