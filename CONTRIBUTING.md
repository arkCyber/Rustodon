# Contributing to Rustodon

Thank you for your interest in contributing to Rustodon! This document provides guidelines for contributors.

## 🤝 How to Contribute

### Reporting Issues

Before creating bug reports, please check existing issues. Include:
- Clear and descriptive title
- Detailed description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment information

### Code Contributions

#### Prerequisites
- Rust 1.70 or higher
- PostgreSQL 13 or higher
- Redis 6.2 or higher

#### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/Rustodon.git
   cd Rustodon
   ```

2. **Set up environment**
   ```bash
   cp .env.example .env
   cargo build
   ```

3. **Create feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

#### Coding Standards

- Follow Rust Style Guide
- Use `cargo fmt` and `cargo clippy`
- Write comprehensive tests
- Add documentation for public APIs

#### Commit Guidelines

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(auth): add OAuth2 authentication
fix(api): resolve status creation issue
docs(readme): update installation guide
```

#### Pull Request Process

1. Ensure code follows standards
2. Update documentation if needed
3. Create descriptive pull request
4. Address review comments

## 📋 Development Commands

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Run API tests (server must be running)
chmod +x test_api.sh
./test_api.sh

# Format code
cargo fmt

# Lint code
cargo clippy

# Build release
cargo build --release

# Run server for testing
SQLX_OFFLINE=true cargo run -p rustodon-server
```

## 🧪 Testing

### Running Tests

```bash
# Run all unit tests
cargo test

# Run tests for specific crate
cargo test -p rustodon-api

# Run with logging
RUST_LOG=debug cargo test

# Run API integration tests
./test_api.sh
```

### Test Coverage

We aim for high test coverage. When adding new features:
- Write unit tests for all functions
- Add integration tests for API endpoints
- Update the API test script if needed
- Test error conditions and edge cases

## 📞 Getting Help

- GitHub Issues: For bug reports
- GitHub Discussions: For questions
- Email: arksong2018@gmail.com

## 📄 License

By contributing, you agree your contributions will be licensed under AGPL-3.0.

---

Thank you for contributing to Rustodon! 🦀
