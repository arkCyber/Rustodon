#!/bin/bash

# Add sqlx dependencies to crates that need them
echo "Adding sqlx dependencies to crates..."

# List of crates that need sqlx dependency
CRATES=(
    "crates/auth/rustodon-webauthn-credentials"
    "crates/auth/rustodon-sessions"
    "crates/features/rustodon-account-suggestions"
    "crates/features/rustodon-account-conversations"
    "crates/features/rustodon-analytics"
    "crates/features/rustodon-groups"
    "crates/features/rustodon-mentions"
    "crates/features/rustodon-account-aliases"
    "crates/features/rustodon-email-domain-blocks"
    "crates/features/rustodon-access-grants"
    "crates/features/rustodon-instances"
    "crates/features/rustodon-devices"
    "crates/features/rustodon-reports"
    "crates/features/rustodon-preview-cards"
    "crates/features/rustodon-severed-relationships"
    "crates/features/rustodon-bulk-imports"
    "crates/features/rustodon-scheduled-statuses"
    "crates/features/rustodon-account-moderation-notes"
    "crates/features/rustodon-custom-emojis"
    "crates/features/rustodon-conversations"
    "crates/features/rustodon-bookmarks"
    "crates/features/rustodon-encrypted-messages"
    "crates/features/rustodon-applications"
    "crates/features/rustodon-account-warnings"
    "crates/features/rustodon-status-pins"
    "crates/features/rustodon-tag-follows"
    "crates/features/rustodon-domains"
    "crates/features/rustodon-terms-of-service"
    "crates/features/rustodon-reblogs"
    "crates/features/rustodon-statuses"
    "crates/features/rustodon-account-deletion-requests"
    "crates/features/rustodon-tags"
    "crates/features/rustodon-trends"
    "crates/features/rustodon-follow-requests"
    "crates/features/rustodon-follow-recommendation-suppressions"
    "crates/features/rustodon-mutes"
    "crates/features/rustodon-account-notes"
    "crates/features/rustodon-accounts"
    "crates/features/rustodon-user-settings"
    "crates/features/rustodon-favourites"
    "crates/features/rustodon-software-updates"
    "crates/features/rustodon-polls"
    "crates/features/rustodon-access-tokens"
    "crates/admin/rustodon-admin"
    "crates/server/rustodon-web"
    "crates/server/rustodon-server"
    "crates/utils/rustodon-workers"
    "crates/utils/rustodon-mailer"
    "crates/utils/rustodon-queue"
    "crates/utils/rustodon-logging"
    "crates/utils/rustodon-config"
    "crates/utils/rustodon-scheduler"
    "crates/utils/rustodon-webhooks"
    "crates/utils/rustodon-cache"
    "crates/utils/rustodon-metrics"
    "crates/utils/rustodon-search"
    "crates/cli/rustodon-cli"
    "crates/federation/rustodon-federation"
    "crates/api/rustodon-activitypub"
    "crates/api/rustodon-streaming"
    "crates/api/rustodon-api"
    "crates/media/rustodon-storage"
)

for crate in "${CRATES[@]}"; do
    if [ -f "$crate/Cargo.toml" ]; then
        echo "Adding sqlx to $crate"
        cd "$crate"
        cargo add sqlx --features postgres,chrono,runtime-tokio-rustls
        cd - > /dev/null
    fi
done

# Add cidr dependency to rustodon-ip-blocks
echo "Adding cidr dependency to rustodon-ip-blocks"
cd "crates/features/rustodon-ip-blocks"
cargo add cidr
cd - > /dev/null

echo "Dependencies added successfully!"
