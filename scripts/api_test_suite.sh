#!/bin/bash

# Rustodon API 全面测试套件
# 作者: arkSong (arksong2018@gmail.com)
# 功能: 自动化测试注册、登录、发帖、关注等完整用户流程

set -e

# 配置
BASE_URL="http://127.0.0.1:3000"
API_BASE="$BASE_URL/api/v1"
TEST_USER="testuser"
TEST_EMAIL="test@example.com"
TEST_PASSWORD="testpassword123"
ADMIN_USER="admin"
ADMIN_EMAIL="admin@rustodon.example.com"

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
    log_info "Running test: $test_name"

    if eval "$test_command" > /tmp/test_output.json 2>/dev/null; then
        local status_code=$(cat /tmp/test_output.json | grep -o 'HTTP/[0-9.]* [0-9]*' | tail -1 | awk '{print $2}')
        if [ "$status_code" = "$expected_status" ]; then
            log_success "✓ $test_name passed (Status: $status_code)"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            log_error "✗ $test_name failed (Expected: $expected_status, Got: $status_code)"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        log_error "✗ $test_name failed (Command execution error)"
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
    run_test "Root Path" \
        "curl -s -w '%{http_code}' -o /dev/null $BASE_URL/" \
        "200"

    # 实例信息测试
    run_test "Instance Info" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/instance" \
        "200"

    # 状态列表测试
    run_test "Statuses List" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses" \
        "200"

    # 账户列表测试
    run_test "Accounts List" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts" \
        "200"

    # 趋势标签测试
    run_test "Trending Tags" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/trends/tags" \
        "200"

    # 趋势状态测试
    run_test "Trending Statuses" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/trends/statuses" \
        "200"
}

# 用户注册测试
test_user_registration() {
    log_info "=== 用户注册测试 ==="

    # 创建测试用户
    run_test "User Registration" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts \
        -H 'Content-Type: application/json' \
        -d '{\"username\":\"$TEST_USER\",\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\",\"agreement\":true}'" \
        "201"

    # 验证用户创建
    run_test "Verify User Created" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts/verify_credentials" \
        "200"
}

# 用户认证测试
test_user_authentication() {
    log_info "=== 用户认证测试 ==="

    # 获取访问令牌
    run_test "Get Access Token" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/oauth/token \
        -H 'Content-Type: application/x-www-form-urlencoded' \
        -d 'grant_type=password&username=$TEST_USER&password=$TEST_PASSWORD&client_id=test_client&client_secret=test_secret'" \
        "200"

    # 验证凭据
    run_test "Verify Credentials" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts/verify_credentials" \
        "200"
}

# 状态发布测试
test_status_creation() {
    log_info "=== 状态发布测试 ==="

    # 发布文本状态
    run_test "Create Text Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{\"status\":\"Hello Rustodon! This is a test status.\"}'" \
        "200"

    # 发布带可见性的状态
    run_test "Create Public Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{\"status\":\"This is a public status\",\"visibility\":\"public\"}'" \
        "200"

    # 发布私有状态
    run_test "Create Private Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{\"status\":\"This is a private status\",\"visibility\":\"private\"}'" \
        "200"

    # 发布敏感内容状态
    run_test "Create Sensitive Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{\"status\":\"Sensitive content\",\"sensitive\":true,\"spoiler_text\":\"Content warning\"}'" \
        "200"
}

# 状态交互测试
test_status_interactions() {
    log_info "=== 状态交互测试 ==="

    # 获取状态列表
    run_test "Get Statuses" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses" \
        "200"

    # 获取单个状态
    run_test "Get Single Status" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses/1" \
        "200"

    # 点赞状态
    run_test "Favourite Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/favourite" \
        "200"

    # 取消点赞
    run_test "Unfavourite Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/unfavourite" \
        "200"

    # 转发状态
    run_test "Reblog Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/reblog" \
        "200"

    # 取消转发
    run_test "Unreblog Status" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/unreblog" \
        "200"

    # 获取点赞列表
    run_test "Get Favourites" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses/1/favourited_by" \
        "200"

    # 获取转发列表
    run_test "Get Reblogs" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/statuses/1/reblogged_by" \
        "200"
}

# 用户关系测试
test_user_relationships() {
    log_info "=== 用户关系测试 ==="

    # 关注用户
    run_test "Follow User" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts/1/follow" \
        "200"

    # 取消关注
    run_test "Unfollow User" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts/1/unfollow" \
        "200"

    # 获取关注者列表
    run_test "Get Followers" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts/1/followers" \
        "200"

    # 获取关注列表
    run_test "Get Following" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/accounts/1/following" \
        "200"

    # 阻止用户
    run_test "Block User" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts/1/block" \
        "200"

    # 取消阻止
    run_test "Unblock User" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts/1/unblock" \
        "200"

    # 静音用户
    run_test "Mute User" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts/1/mute" \
        "200"

    # 取消静音
    run_test "Unmute User" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/accounts/1/unmute" \
        "200"
}

# 通知测试
test_notifications() {
    log_info "=== 通知测试 ==="

    # 获取通知列表
    run_test "Get Notifications" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/notifications" \
        "200"

    # 获取单个通知
    run_test "Get Single Notification" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/notifications/1" \
        "200"

    # 清除通知
    run_test "Clear Notifications" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/notifications/clear" \
        "200"
}

# 搜索测试
test_search() {
    log_info "=== 搜索测试 ==="

    # 搜索账户
    run_test "Search Accounts" \
        "curl -s -w '%{http_code}' -o /dev/null '$API_BASE/accounts/search?q=admin'" \
        "200"

    # 搜索状态
    run_test "Search Statuses" \
        "curl -s -w '%{http_code}' -o /dev/null '$API_BASE/search?q=test'" \
        "200"

    # 搜索标签
    run_test "Search Tags" \
        "curl -s -w '%{http_code}' -o /dev/null '$API_BASE/tags/search?q=test'" \
        "200"
}

# 标签测试
test_tags() {
    log_info "=== 标签测试 ==="

    # 获取标签信息
    run_test "Get Tag Info" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/tags/test" \
        "200"

    # 关注标签
    run_test "Follow Tag" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/tags/test/follow" \
        "200"

    # 取消关注标签
    run_test "Unfollow Tag" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/tags/test/unfollow" \
        "200"
}

# 列表测试
test_lists() {
    log_info "=== 列表测试 ==="

    # 创建列表
    run_test "Create List" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/lists \
        -H 'Content-Type: application/json' \
        -d '{\"title\":\"Test List\"}'" \
        "200"

    # 获取列表
    run_test "Get Lists" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/lists" \
        "200"

    # 获取列表账户
    run_test "Get List Accounts" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/lists/1/accounts" \
        "200"

    # 添加账户到列表
    run_test "Add Account to List" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/lists/1/accounts \
        -H 'Content-Type: application/json' \
        -d '{\"account_ids\":[\"1\"]}'" \
        "200"

    # 从列表移除账户
    run_test "Remove Account from List" \
        "curl -s -w '%{http_code}' -X DELETE $API_BASE/lists/1/accounts \
        -H 'Content-Type: application/json' \
        -d '{\"account_ids\":[\"1\"]}'" \
        "200"
}

# 书签测试
test_bookmarks() {
    log_info "=== 书签测试 ==="

    # 添加书签
    run_test "Add Bookmark" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/bookmark" \
        "200"

    # 获取书签列表
    run_test "Get Bookmarks" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/bookmarks" \
        "200"

    # 移除书签
    run_test "Remove Bookmark" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses/1/unbookmark" \
        "200"
}

# 轮询测试
test_polls() {
    log_info "=== 轮询测试 ==="

    # 创建轮询
    run_test "Create Poll" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{\"status\":\"What is your favorite color?\",\"poll\":{\"options\":[\"Red\",\"Blue\",\"Green\"],\"expires_in\":86400}}'" \
        "200"

    # 投票
    run_test "Vote in Poll" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/polls/1/votes \
        -H 'Content-Type: application/json' \
        -d '{\"choices\":[0]}'" \
        "200"

    # 获取轮询结果
    run_test "Get Poll Results" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/polls/1" \
        "200"
}

# 媒体上传测试
test_media_upload() {
    log_info "=== 媒体上传测试 ==="

    # 创建测试图片文件
    echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==" | base64 -d > /tmp/test.png

    # 上传媒体
    run_test "Upload Media" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/media \
        -F 'file=@/tmp/test.png' \
        -F 'description=Test image'" \
        "200"

    # 清理测试文件
    rm -f /tmp/test.png
}

# 错误处理测试
test_error_handling() {
    log_info "=== 错误处理测试 ==="

    # 404 错误
    run_test "404 Error" \
        "curl -s -w '%{http_code}' -o /dev/null $API_BASE/nonexistent" \
        "404"

    # 无效的 JSON
    run_test "Invalid JSON" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d 'invalid json'" \
        "400"

    # 缺少必需字段
    run_test "Missing Required Fields" \
        "curl -s -w '%{http_code}' -X POST $API_BASE/statuses \
        -H 'Content-Type: application/json' \
        -d '{}'" \
        "400"
}

# 性能测试
test_performance() {
    log_info "=== 性能测试 ==="

    # 并发请求测试
    log_info "执行并发请求测试..."
    for i in {1..10}; do
        curl -s -o /dev/null "$API_BASE/instance" &
    done
    wait

    # 响应时间测试
    log_info "测试响应时间..."
    start_time=$(date +%s%N)
    curl -s -o /dev/null "$API_BASE/instance"
    end_time=$(date +%s%N)
    response_time=$(( (end_time - start_time) / 1000000 ))
    log_info "响应时间: ${response_time}ms"
}

# 主测试函数
main() {
    log_info "开始 Rustodon API 全面测试..."
    log_info "服务器地址: $BASE_URL"

    # 检查服务器状态
    if ! check_server; then
        log_error "服务器未运行，请先启动服务器"
        exit 1
    fi

    # 运行所有测试
    test_basic_apis
    test_user_registration
    test_user_authentication
    test_status_creation
    test_status_interactions
    test_user_relationships
    test_notifications
    test_search
    test_tags
    test_lists
    test_bookmarks
    test_polls
    test_media_upload
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
        log_error "部分测试失败，请检查服务器日志"
        exit 1
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
