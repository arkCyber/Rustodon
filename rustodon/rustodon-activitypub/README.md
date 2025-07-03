# Rustodon ActivityPub Implementation

This crate provides a complete implementation of the ActivityPub protocol for the Rustodon project, enabling federated social networking capabilities.

## Features

### ✅ Implemented

- **Core ActivityPub Types**: Complete implementation of ActivityPub vocabulary including Activities, Objects, and Actors
- **Activity Types**: Support for all major activity types (Create, Follow, Like, Announce, Accept, Reject, Undo, Delete, Update, Block)
- **Object Types**: Support for various object types (Note, Article, Video, Audio, Image, etc.)
- **Actor Types**: Support for Person, Organization, Service, Application, and Group actors
- **JSON-LD Serialization**: Proper ActivityPub JSON-LD format with correct context and field naming
- **Activity Processing**: Comprehensive inbox activity processing with type-specific handlers
- **Actor Discovery**: Remote actor discovery and caching
- **Convenience Methods**: Easy-to-use constructors for common ActivityPub objects
- **Comprehensive Testing**: Unit tests covering all major functionality

### 🚧 TODO (Database Integration)

The current implementation has placeholder methods for database operations. The following need to be implemented:

- **Database Schema**: Create SQL migrations for ActivityPub tables
- **Actor Caching**: Implement database storage and retrieval for remote actors
- **Activity Storage**: Store incoming and outgoing activities
- **Object Storage**: Store and manage ActivityPub objects (posts, media, etc.)
- **Relationship Management**: Follow/follower relationships, likes, announces, blocks
- **HTTP Signature Verification**: Cryptographic verification of incoming activities
- **Activity Delivery**: Outgoing activity delivery to remote servers

## Usage

```rust
use rustodon_activitypub::{ActivityPubService, Activity, ActivityType, Actor, Object};
use sqlx::PgPool;

// Create the service
let pool = PgPool::connect("postgres://localhost/rustodon").await?;
let mut service = ActivityPubService::new(pool, "https://your-instance.com".to_string());

// Set up cryptographic keys (for signing outgoing activities)
// let private_key = generate_rsa_key(); // You need to implement this
// service.set_private_key(private_key);

// Process incoming activities
let activity = Activity::create(
    "https://remote.example/users/alice".to_string(),
    serde_json::json!({
        "type": "Note",
        "content": "Hello, ActivityPub world!",
        "attributedTo": "https://remote.example/users/alice"
    })
);

service.process_inbox_activity(activity).await?;

// Discover remote actors
let actor = service.discover_actor("https://remote.example/users/alice").await?;
println!("Discovered actor: {}", actor.preferred_username.unwrap_or_default());
```

## Creating ActivityPub Objects

### Activities

```rust
use rustodon_activitypub::{Activity, ActivityType};

// Create a Follow activity
let follow = Activity::follow(
    "https://your-instance.com/users/bob".to_string(),
    "https://remote.example/users/alice".to_string()
);

// Create a Like activity
let like = Activity::like(
    "https://your-instance.com/users/bob".to_string(),
    "https://remote.example/objects/123".to_string()
);
```

### Actors

```rust
use rustodon_activitypub::{Actor, ActorType};

// Create a Person actor
let actor = Actor::person(
    "https://your-instance.com/users/bob".to_string(),
    "bob".to_string(),
    Some("Bob Smith".to_string())
);
```

### Objects

```rust
use rustodon_activitypub::{Object, ObjectType};

// Create a Note object
let note = Object::note(
    "https://your-instance.com/objects/456".to_string(),
    "Hello, world!".to_string(),
    "https://your-instance.com/users/bob".to_string()
);
```

## ActivityPub Compliance

This implementation follows the [W3C ActivityPub specification](https://www.w3.org/TR/activitypub/) and is designed to be compatible with other ActivityPub implementations including:

- Mastodon
- Pleroma
- PeerTube
- Pixelfed
- And other ActivityPub-compliant servers

## Architecture

The implementation is structured around several key components:

- **ActivityPubService**: Main service for handling federation operations
- **Activity/Actor/Object**: Core data structures representing ActivityPub entities
- **Error Handling**: Comprehensive error types for all federation scenarios
- **Type Safety**: Rust enums for ActivityPub vocabulary ensuring type safety
- **Async Support**: Full async/await support for network and database operations

## Testing

Run the test suite:

```bash
cargo test
```

The tests cover:
- ActivityPub type serialization/deserialization
- Activity creation and validation
- Actor and Object construction
- Type display implementations
- JSON-LD format compliance

## Next Steps

1. **Database Implementation**: Implement the database layer with proper SQL migrations
2. **HTTP Signatures**: Add cryptographic signature verification
3. **Activity Delivery**: Implement outgoing activity delivery to remote servers
4. **Webfinger Support**: Add Webfinger protocol support for actor discovery
5. **Performance Optimization**: Add caching and connection pooling
6. **Integration**: Integrate with the main Rustodon API endpoints

## Contributing

When contributing to this crate:

1. Ensure all tests pass: `cargo test`
2. Follow ActivityPub specification requirements
3. Add tests for new functionality
4. Update documentation for API changes
5. Maintain compatibility with existing ActivityPub implementations

## License

This project is part of the Rustodon project and follows the same licensing terms.
