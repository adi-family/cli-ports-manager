# Contributing to Ports Manager

Thank you for your interest in contributing to Ports Manager! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report, include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment (OS, Rust version, ports-manager version)
- Any relevant configuration files or logs

Use the bug report template when creating issues.

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- A clear and descriptive title
- Detailed description of the proposed feature
- Example usage or API design
- Explanation of why this enhancement would be useful

Use the feature request template when creating issues.

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes**:
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed
3. **Ensure tests pass**:
   ```bash
   cargo test
   cargo clippy --all-targets --all-features
   cargo fmt --check
   ```
4. **Commit your changes** with clear, descriptive commit messages
5. **Push to your fork** and submit a pull request

### Commit Message Guidelines

We use conventional commits with emojis:

- ✨ `:sparkles:` - New features
- 🐛 `:bug:` - Bug fixes
- 📝 `:memo:` - Documentation
- ♻️ `:recycle:` - Refactoring
- ✅ `:white_check_mark:` - Tests
- 🚀 `:rocket:` - Performance improvements
- 🔧 `:wrench:` - Configuration changes
- 🔒 `:lock:` - Security fixes

Example: `✨ Add support for IPv6 port ranges`

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Setting Up Your Development Environment

1. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ports-manager.git
   cd ports-manager
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Run the CLI**:
   ```bash
   cargo run -- --help
   ```

### Project Structure

```
ports-manager/
├── src/
│   ├── main.rs          # CLI entry point and command handling
│   ├── config.rs        # Configuration management
│   └── port_finder.rs   # Port allocation logic
├── tests/               # Integration tests
├── .github/             # GitHub Actions workflows
└── Cargo.toml          # Project dependencies
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Style

This project uses:
- `rustfmt` for code formatting
- `clippy` for linting

Run before committing:
```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation

- Add doc comments to all public items
- Include examples in doc comments where helpful
- Update README.md if adding user-facing features

## Review Process

1. A maintainer will review your pull request
2. Changes may be requested
3. Once approved, your PR will be merged

## Questions?

Feel free to:
- Open an issue with the question label
- Start a discussion in GitHub Discussions
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
