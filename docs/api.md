# Rustodon API Documentation

Rustodon provides a RESTful API that is 100% compatible with the Mastodon API. This document covers the currently implemented endpoints.

## Base URL

```
http://localhost:3000/api/v1
```

## Authentication

Most endpoints require authentication using Bearer tokens. Include the token in the Authorization header:

```
Authorization: Bearer YOUR_TOKEN_HERE
```

## Response Format

All responses follow this format:

```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

Error responses:

```json
{
  "success": false,
  "data": null,
  "error": "Error message"
}
```

## Endpoints

### Health Check

Check if the server is running.

**GET** `/health`

**Response:**
```json
{
  "success": true,
  "data": "OK",
  "error": null
}
```

### Authentication

#### Register User

Create a new user account.

**POST** `/auth/register`

**Request Body:**
```json
{
  "username": "testuser",
  "email": "test@example.com",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "user_id": 1,
    "token": "base64-encoded-token"
  },
  "error": null
}
```

#### Login User

Authenticate an existing user.

**POST** `/auth/login`

**Request Body:**
```json
{
  "username": "testuser",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "user_id": 1,
    "token": "base64-encoded-token"
  },
  "error": null
}
```

### OAuth Applications

#### Register OAuth Application

Register a new OAuth application.

**POST** `/apps`

**Request Body:**
```json
{
  "client_name": "My App",
  "redirect_uris": "http://localhost:3000/oauth/callback",
  "scopes": "read write follow",
  "website": "https://myapp.com"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "client_uuid",
    "client_id": "client_uuid",
    "client_secret": "secret_uuid",
    "name": "My App",
    "redirect_uri": "http://localhost:3000/oauth/callback",
    "scopes": "read write follow",
    "website": "https://myapp.com"
  },
  "error": null
}
```

### Account Management

#### Verify Credentials

Get information about the authenticated user.

**GET** `/accounts/verify_credentials`

**Headers:**
```
Authorization: Bearer YOUR_TOKEN
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "1",
    "username": "testuser",
    "display_name": "Test User",
    "note": "User bio",
    "avatar": "avatar_url",
    "header": "header_url",
    "locked": false,
    "bot": false,
    "discoverable": true,
    "group": false,
    "created_at": "2023-12-01T10:00:00Z",
    "last_status_at": null,
    "statuses_count": 0,
    "followers_count": 0,
    "following_count": 0
  },
  "error": null
}
```

### Status Management

#### Create Status

Post a new status.

**POST** `/statuses`

**Headers:**
```
Authorization: Bearer YOUR_TOKEN
```

**Request Body:**
```json
{
  "status": "Hello, world! 🌍",
  "visibility": "public",
  "sensitive": false,
  "spoiler_text": ""
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "1",
    "uri": "https://example.com/statuses/uuid",
    "account": { ... },
    "content": "Hello, world! 🌍",
    "created_at": "2023-12-01T10:00:00Z",
    "replies_count": 0,
    "reblogs_count": 0,
    "favourites_count": 0,
    "favourited": false,
    "reblogged": false,
    "muted": false,
    "bookmarked": false,
    "pinned": false,
    "sensitive": false,
    "spoiler_text": "",
    "visibility": "public",
    "media_attachments": [],
    "mentions": [],
    "tags": [],
    "application": null,
    "language": null,
    "reblog": null,
    "in_reply_to_id": null,
    "in_reply_to_account_id": null
  },
  "error": null
}
```

#### Get Status

Retrieve a specific status.

**GET** `/statuses/{id}`

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "1",
    "uri": "https://example.com/statuses/uuid",
    "account": { ... },
    "content": "Hello, world! 🌍",
    "created_at": "2023-12-01T10:00:00Z",
    // ... other status fields
  },
  "error": null
}
```

### Timelines

#### Public Timeline

Get the public timeline.

**GET** `/timelines/public`

**Query Parameters:**
- `limit` (optional): Maximum number of statuses (default: 20, max: 40)
- `max_id` (optional): Return statuses older than this ID
- `since_id` (optional): Return statuses newer than this ID

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "1",
      "uri": "https://example.com/statuses/uuid",
      "account": { ... },
      "content": "Public status content",
      "created_at": "2023-12-01T10:00:00Z",
      // ... other status fields
    }
  ],
  "error": null
}
```

#### Home Timeline

Get the authenticated user's home timeline.

**GET** `/timelines/home`

**Headers:**
```
Authorization: Bearer YOUR_TOKEN
```

**Query Parameters:**
- `limit` (optional): Maximum number of statuses (default: 20, max: 40)
- `max_id` (optional): Return statuses older than this ID
- `since_id` (optional): Return statuses newer than this ID

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "1",
      "uri": "https://example.com/statuses/uuid",
      "account": { ... },
      "content": "Home timeline status",
      "created_at": "2023-12-01T10:00:00Z",
      // ... other status fields
    }
  ],
  "error": null
}
```

### Status Interactions

#### Favourite Status

Add a status to favourites.

**POST** `/statuses/{id}/favourite`

**Headers:**
```
Authorization: Bearer YOUR_TOKEN
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "1",
    // ... status fields with favourited: true and favourites_count incremented
  },
  "error": null
}
```

#### Unfavourite Status

Remove a status from favourites.

**POST** `/statuses/{id}/unfavourite`

**Headers:**
```
Authorization: Bearer YOUR_TOKEN
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "1",
    // ... status fields with favourited: false and favourites_count decremented
  },
  "error": null
}
```

## Error Codes

- `400 Bad Request`: Invalid request parameters
- `401 Unauthorized`: Missing or invalid authentication token
- `404 Not Found`: Resource not found
- `422 Unprocessable Entity`: Validation errors
- `500 Internal Server Error`: Server error

## Rate Limiting

API endpoints are rate limited to prevent abuse:
- Default: 60 requests per minute per IP
- Burst: 10 requests

Rate limit headers are included in responses:
```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 59
X-RateLimit-Reset: 1638360000
```

## Testing

Use the provided test script to validate API functionality:

```bash
chmod +x test_api.sh
./test_api.sh
```

## Future Endpoints

The following endpoints are planned for future releases:

- Media attachments (`/media`)
- Notifications (`/notifications`)
- Following/followers (`/accounts/{id}/follow`, `/accounts/{id}/followers`)
- Search (`/search`)
- Lists (`/lists`)
- Filters (`/filters`)
- Streaming API (`/streaming`)

## Support

For API questions or issues:
- **GitHub Issues**: [Report bugs](https://github.com/arkCyber/Rustodon/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/arkCyber/Rustodon/discussions)
- **Email**: arksong2018@gmail.com
