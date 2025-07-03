#!/usr/bin/env python3
"""
Simple Test Server for Rustodon API Testing
Author: arkSong (arksong2018@gmail.com)
Project: rustodon

This is a simple HTTP server that responds to basic API endpoints
for testing purposes.
"""

import http.server
import socketserver
import json
import os
from datetime import datetime

class TestHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()

        if self.path == "/health":
            response = {"status": "ok", "message": "Health check passed", "timestamp": datetime.now().isoformat()}
        elif self.path == "/api/v1/instance":
            response = {"version": "1.0.0", "name": "Rustodon Test Server", "description": "Test server for API validation"}
        elif self.path == "/api/v1/timelines/public":
            response = {"statuses": [], "next": None, "prev": None}
        elif self.path.startswith("/api/v1/accounts/"):
            response = {"id": "1", "username": "testuser", "display_name": "Test User", "created_at": "2025-01-01T00:00:00Z"}
        elif self.path.startswith("/api/v1/search"):
            response = {"accounts": [], "statuses": [], "hashtags": []}
        else:
            response = {
                "message": "Welcome to Rustodon Test Server",
                "endpoints": [
                    "/health",
                    "/api/v1/instance",
                    "/api/v1/timelines/public",
                    "/api/v1/accounts/1",
                    "/api/v1/search?q=test"
                ]
            }

        self.wfile.write(json.dumps(response, indent=2).encode())

    def do_POST(self):
        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()

        if self.path == "/api/v1/auth/register":
            response = {"token": "test_token_123", "user_id": "1", "message": "Registration successful"}
        elif self.path == "/api/v1/auth/login":
            response = {"token": "test_token_123", "user_id": "1", "message": "Login successful"}
        elif self.path == "/api/v1/statuses":
            response = {"id": "1", "content": "Test status", "created_at": datetime.now().isoformat()}
        elif self.path == "/api/v1/media":
            response = {"id": "1", "type": "image", "url": "https://example.com/test.jpg"}
        elif self.path == "/api/v1/notifications":
            response = {"notifications": []}
        elif self.path.endswith("/follow"):
            response = {"message": "Follow successful"}
        elif self.path.endswith("/unfollow"):
            response = {"message": "Unfollow successful"}
        elif self.path.endswith("/favourite"):
            response = {"message": "Favourite successful"}
        elif self.path.endswith("/unfavourite"):
            response = {"message": "Unfavourite successful"}
        elif self.path.endswith("/reblog"):
            response = {"message": "Reblog successful"}
        elif self.path.endswith("/unreblog"):
            response = {"message": "Unreblog successful"}
        elif self.path == "/api/v1/lists":
            response = {"lists": []}
        elif self.path == "/api/v1/conversations":
            response = {"conversations": []}
        elif self.path == "/api/v1/bookmarks":
            response = {"bookmarks": []}
        elif self.path == "/api/v1/mutes":
            response = {"mutes": []}
        elif self.path == "/api/v1/blocks":
            response = {"blocks": []}
        elif self.path == "/api/v1/reports":
            response = {"reports": []}
        elif self.path == "/api/v1/filters":
            response = {"filters": []}
        else:
            response = {"error": "Endpoint not implemented", "path": self.path}

        self.wfile.write(json.dumps(response, indent=2).encode())

    def log_message(self, format, *args):
        print(f"[{self.log_date_time_string()}] {format % args}")

if __name__ == "__main__":
    PORT = 3000
    with socketserver.TCPServer(("", PORT), TestHandler) as httpd:
        print(f"Server running on port {PORT}")
        print(f"Test endpoints available at http://localhost:{PORT}")
        httpd.serve_forever()
