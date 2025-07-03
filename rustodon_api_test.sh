#!/bin/bash

API_URL="http://localhost:3000"
USER_COUNT=3

# 注册和登录
declare -A TOKENS

echo "=== 批量注册 ==="
for i in $(seq 1 $USER_COUNT); do
  USER="apitestuser$i"
  EMAIL="apitestuser$i@example.com"
  PASS="apitestpass"
  echo "注册 $USER ..."
  RESP=$(curl -s -X POST "$API_URL/api/v1/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"username\":\"$USER\",\"email\":\"$EMAIL\",\"password\":\"$PASS\",\"agreement\":true,\"locale\":\"en\"}")
  TOKEN=$(echo "$RESP" | grep -o '"token":"[^"]*' | cut -d':' -f2 | tr -d '"')
  TOKENS[$USER]=$TOKEN
  echo "  -> token: $TOKEN"
done

echo
echo "=== 每个用户发一条动态 ==="
for i in $(seq 1 $USER_COUNT); do
  USER="apitestuser$i"
  TOKEN="${TOKENS[$USER]}"
  echo "用户 $USER 发动态 ..."
  curl -s -X POST "$API_URL/api/v1/statuses" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"status\":\"Hello from $USER! #rustodon\"}" | grep -E 'id|content'
done

echo
echo "=== 拉取公共 timeline ==="
curl -s "$API_URL/api/v1/timelines/public" | head -c 500
echo

echo "=== 拉取每个用户的个人 timeline ==="
for i in $(seq 1 $USER_COUNT); do
  USER="apitestuser$i"
  TOKEN="${TOKENS[$USER]}"
  echo "用户 $USER 的 home timeline:"
  curl -s "$API_URL/api/v1/timelines/home" -H "Authorization: Bearer $TOKEN" | head -c 500
  echo
done
