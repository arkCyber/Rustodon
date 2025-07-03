#!/bin/sh

API_URL="http://localhost:3000"
USER_COUNT=3
IMG_PATH="/Users/arksong/mastodon@rustodon_副本/.storybook/images.jpg"
USER_IDS="4 5 6"

# 注册和登录
echo "=== 批量注册/登录 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  EMAIL="apitestuser$i@example.com"
  PASS="apitestpass"
  echo "注册 $USER ..."
  RESP=$(curl -s -X POST "$API_URL/api/v1/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"username\":\"$USER\",\"email\":\"$EMAIL\",\"password\":\"$PASS\",\"agreement\":true,\"locale\":\"en\"}")
  echo "注册响应: $RESP"
  TOKEN=$(echo "$RESP" | sed -n 's/.*"token"[ :]\{0,1\}["'\'' ]*\([^"'\'' ,}]*\).*/\1/p' | head -n1)
  if [ -z "$TOKEN" ]; then
    # 注册失败，尝试登录
    echo "$RESP" | grep -q 'User already exists'
    if [ $? -eq 0 ]; then
      echo "$USER 已存在，尝试登录..."
      LOGIN_RESP=$(curl -s -X POST "$API_URL/api/v1/auth/login" \
        -H "Content-Type: application/json" \
        -d "{\"username_or_email\":\"$USER\",\"password\":\"$PASS\"}")
      echo "登录响应: $LOGIN_RESP"
      TOKEN=$(echo "$LOGIN_RESP" | sed -n 's/.*"token"[ :]\{0,1\}["'\'' ]*\([^"'\'' ,}]*\).*/\1/p' | head -n1)
      if [ -z "$TOKEN" ]; then
        TOKEN=$(echo "$LOGIN_RESP" | grep -o '"token":"[^"]*' | head -n1 | cut -d':' -f2 | tr -d '"')
      fi
    fi
  fi
  if [ -z "$TOKEN" ]; then
    echo "[FATAL] $USER 无法获取 token，终止测试。"
    exit 1
  fi
  eval TOKEN$i="$TOKEN"
  eval echo "  -> token: \$TOKEN$i"
done

echo
echo "=== 每个用户发一条动态 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  echo "用户 $USER 发动态 ..."
  STATUS_RESP=$(curl -s -X POST "$API_URL/api/v1/statuses" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"status":"Hello from '$USER' #rustodon"}')
  echo "发动态响应: $STATUS_RESP"
  STATUS_ID=$(echo "$STATUS_RESP" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
  eval STATUS_ID$i="$STATUS_ID"
  eval echo "  -> status_id: \$STATUS_ID$i"
done

echo
echo "=== 每个用户上传图片 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  echo "用户 $USER 上传图片 ..."
  MEDIA_RESP=$(curl -s -X POST "$API_URL/api/v1/media" \
    -H "Authorization: Bearer $TOKEN" \
    -F "file=@$IMG_PATH")
  echo "上传图片响应: $MEDIA_RESP"
  MEDIA_ID=$(echo "$MEDIA_RESP" | grep -o '"id":"[0-9]*' | head -n1 | cut -d'"' -f4)
  eval MEDIA_ID$i="$MEDIA_ID"
  eval echo "  -> media_id: \$MEDIA_ID$i"
done

echo
echo "=== 点赞（favourite） ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  eval STATUS_ID=\$STATUS_ID$i
  echo "用户 $USER 点赞自己的动态 $STATUS_ID ..."
  curl -s -X POST "$API_URL/api/v1/statuses/$STATUS_ID/favourite" \
    -H "Authorization: Bearer $TOKEN" > /dev/null
  echo "  -> done"
done

echo
echo "=== 转发（reblog） ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  eval STATUS_ID=\$STATUS_ID$i
  echo "用户 $USER 转发自己的动态 $STATUS_ID ..."
  curl -s -X POST "$API_URL/api/v1/statuses/$STATUS_ID/reblog" \
    -H "Authorization: Bearer $TOKEN" > /dev/null
  echo "  -> done"
done

echo
echo "=== 关注/取关 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  TARGET_ID=$(( (i - 3) % 3 + 4 ))
  echo "用户 $USER 关注用户 $TARGET_ID ..."
  curl -s -X POST "$API_URL/api/v1/accounts/$TARGET_ID/follow" \
    -H "Authorization: Bearer $TOKEN" > /dev/null
  echo "  -> done"
  echo "用户 $USER 取关用户 $TARGET_ID ..."
  curl -s -X POST "$API_URL/api/v1/accounts/$TARGET_ID/unfollow" \
    -H "Authorization: Bearer $TOKEN" > /dev/null
  echo "  -> done"
done

echo
echo "=== 评论（回复） ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  eval STATUS_ID=\$STATUS_ID$i
  echo "用户 $USER 回复自己的动态 $STATUS_ID ..."
  curl -s -X POST "$API_URL/api/v1/statuses" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"status\":\"Reply from $USER\",\"in_reply_to_id\":\"$STATUS_ID\"}" > /dev/null
  echo "  -> done"
done

echo
echo "=== 拉黑/解除拉黑 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  TARGET_ID=$(( (i - 3) % 3 + 4 ))
  echo "用户 $USER 拉黑用户 $TARGET_ID ..."
  curl -s -X POST "$API_URL/api/v1/accounts/$TARGET_ID/block" \
    -H "Authorization: Bearer $TOKEN" > /dev/null
  echo "  -> done"
  echo "用户 $USER 解除拉黑用户 $TARGET_ID ..."
  curl -s -X POST "$API_URL/api/v1/accounts/$TARGET_ID/unblock" \
    -H "Authorization: Bearer $TOKEN" > /dev/null
  echo "  -> done"
done

echo
echo "=== 获取通知 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  echo "用户 $USER 获取通知 ..."
  curl -s -X GET "$API_URL/api/v1/notifications" \
    -H "Authorization: Bearer $TOKEN" | head -c 200
  echo " ..."
done

echo
echo "=== 每个用户互发私信（Direct Message） ==="
for i in 4 5 6; do
  FROM_USER="apitestuser$i"
  eval FROM_TOKEN=\$TOKEN$i
  for j in 4 5 6; do
    if [ "$i" != "$j" ]; then
      TO_USER="apitestuser$j"
      echo "$FROM_USER -> $TO_USER 发送私信 ..."
      DM_RESP=$(curl -s -X POST "$API_URL/api/v1/conversations" \
        -H "Authorization: Bearer $FROM_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"recipient":"'$TO_USER'","text":"Hi '$TO_USER'! This is a DM from '$FROM_USER'"}')
      echo "私信响应: $DM_RESP"
    fi
  done
done

echo
echo "=== 每个用户拉取私信历史 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  echo "$USER 拉取私信历史 ..."
  DM_LIST=$(curl -s -X GET "$API_URL/api/v1/conversations" \
    -H "Authorization: Bearer $TOKEN")
  echo "私信历史响应: $DM_LIST"
done

echo
echo "=== 每个用户创建列表 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  echo "$USER 创建列表 ..."
  LIST_RESP=$(curl -s -X POST "$API_URL/api/v1/lists" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"title":"Test List '$USER'"}')
  echo "创建列表响应: $LIST_RESP"
  LIST_ID=$(echo "$LIST_RESP" | grep -o '"id":[0-9]*' | head -n1 | cut -d':' -f2)
  eval LIST_ID$i="$LIST_ID"
  eval echo "  -> list_id: \$LIST_ID$i"
done

echo
echo "=== 每个用户给列表添加成员 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  eval LIST_ID=\$LIST_ID$i
  for j in 4 5 6; do
    if [ "$i" != "$j" ]; then
      echo "$USER 向列表 $LIST_ID 添加 apitestuser$j ..."
      ADD_RESP=$(curl -s -X POST "$API_URL/api/v1/lists/$LIST_ID/accounts" \
        -H "Authorization: Bearer $TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"account_ids":['$j']}' )
      echo "添加成员响应: $ADD_RESP"
    fi
  done
done

echo
echo "=== 每个用户拉取列表内容 ==="
for i in 4 5 6; do
  USER="apitestuser$i"
  eval TOKEN=\$TOKEN$i
  eval LIST_ID=\$LIST_ID$i
  echo "$USER 拉取列表 $LIST_ID 内容 ..."
  LIST_CONTENT=$(curl -s -X GET "$API_URL/api/v1/lists/$LIST_ID/accounts" \
    -H "Authorization: Bearer $TOKEN")
  echo "列表内容响应: $LIST_CONTENT"
done

echo "=== 测试完成 ==="
