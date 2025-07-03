-- Create index for OAuth applications
CREATE INDEX IF NOT EXISTS idx_oauth_apps_client_id ON oauth_applications(client_id);
