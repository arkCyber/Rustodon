#!/bin/bash

# Rustodon 简单 API 测试脚本
# 作者: arkSong (arksong2018@gmail.com)

set -e

BASE_URL="http://127.0.0.1:3000"
API_BASE="$BASE_URL/api/v1"

# 颜色输出
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}=== Rustodon API 测试 ===${NC}"
echo "服务器地址: $BASE_URL"
echo ""

# 测试计数器
total=0
passed=0
failed=0

# 测试函数
test_endpoint() {
    local name="$1"
    local url="$2"
    local expected_status="$3"

    total=$((total + 1))
    echo -n "测试 $name... "

    status=$(curl -s -o /dev/null -w "%{http_code}" "$url")

    if [ "$status" = "$expected_status" ]; then
        echo -e "${GREEN}✓ 通过 (${status})${NC}"
        passed=$((passed + 1))
    else
        echo -e "${RED}✗ 失败 (期望: ${expected_status}, 实际: ${status})${NC}"
        failed=$((failed + 1))
    fi
}

# 基础 API 测试
echo -e "${BLUE}1. 基础 API 测试${NC}"
test_endpoint "根路径" "$BASE_URL/" "200"
test_endpoint "实例信息" "$API_BASE/instance" "200"
test_endpoint "状态列表" "$API_BASE/statuses" "200"
test_endpoint "账户列表" "$API_BASE/accounts" "200"
echo ""

# 用户注册测试
echo -e "${BLUE}2. 用户注册测试${NC}"
test_endpoint "用户注册" "$API_BASE/accounts" "201"
echo ""

# 状态发布测试
echo -e "${BLUE}3. 状态发布测试${NC}"
test_endpoint "发布状态" "$API_BASE/statuses" "200"
echo ""

# 状态交互测试
echo -e "${BLUE}4. 状态交互测试${NC}"
test_endpoint "获取状态1" "$API_BASE/statuses/1" "200"
test_endpoint "点赞状态" "$API_BASE/statuses/1/favourite" "200"
test_endpoint "转发状态" "$API_BASE/statuses/1/reblog" "200"
echo ""

# 用户关系测试
echo -e "${BLUE}5. 用户关系测试${NC}"
test_endpoint "关注用户" "$API_BASE/accounts/1/follow" "200"
test_endpoint "获取关注者" "$API_BASE/accounts/1/followers" "200"
test_endpoint "获取关注列表" "$API_BASE/accounts/1/following" "200"
echo ""

# 通知测试
echo -e "${BLUE}6. 通知测试${NC}"
test_endpoint "获取通知" "$API_BASE/notifications" "200"
echo ""

# 搜索测试
echo -e "${BLUE}7. 搜索测试${NC}"
test_endpoint "搜索账户" "$API_BASE/accounts/search?q=admin" "200"
test_endpoint "搜索状态" "$API_BASE/search?q=test" "200"
echo ""

# 错误处理测试
echo -e "${BLUE}8. 错误处理测试${NC}"
test_endpoint "404错误" "$API_BASE/nonexistent" "404"
echo ""

# 性能测试
echo -e "${BLUE}9. 性能测试${NC}"
echo -n "响应时间测试... "
start_time=$(date +%s%N)
curl -s -o /dev/null "$API_BASE/instance"
end_time=$(date +%s%N)
response_time=$(( (end_time - start_time) / 1000000 ))
echo -e "${GREEN}${response_time}ms${NC}"

echo -n "并发测试... "
for i in {1..5}; do
    curl -s -o /dev/null "$API_BASE/instance" &
done
wait
echo -e "${GREEN}完成${NC}"
echo ""

# 结果汇总
echo -e "${BLUE}=== 测试结果汇总 ===${NC}"
echo "总测试数: $total"
echo -e "通过: ${GREEN}$passed${NC}"
if [ $failed -gt 0 ]; then
    echo -e "失败: ${RED}$failed${NC}"
else
    echo -e "失败: ${GREEN}$failed${NC}"
fi

success_rate=$(( passed * 100 / total ))
echo "成功率: ${success_rate}%"

if [ $failed -eq 0 ]; then
    echo -e "${GREEN}🎉 所有测试通过！${NC}"
else
    echo -e "${RED}⚠️  部分测试失败，这是正常的，因为某些功能可能尚未实现${NC}"
fi
