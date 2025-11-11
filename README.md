# Ports Manager

[![CI](https://github.com/mgorunuch/ports-manager/workflows/CI/badge.svg)](https://github.com/mgorunuch/ports-manager/actions/workflows/ci.yml)
[![Security Audit](https://github.com/mgorunuch/ports-manager/workflows/Security%20Audit/badge.svg)](https://github.com/mgorunuch/ports-manager/actions/workflows/security.yml)
[![Crates.io](https://img.shields.io/crates/v/ports-manager.svg)](https://crates.io/crates/ports-manager)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

A simple, fast, and reliable Rust CLI tool for managing port mappings. Configuration is stored in dotfiles at `~/.config/ports-manager/config.toml`.

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap adi-family/ports-manager
brew install ports-manager
```

Or install directly:

```bash
brew install adi-family/ports-manager/ports-manager
```

### From Crates.io

```bash
cargo install ports-manager
```

### From Source

```bash
git clone https://github.com/mgorunuch/ports-manager.git
cd ports-manager
cargo install --path .
```

### From GitHub Releases

Download pre-built binaries for your platform from the [releases page](https://github.com/mgorunuch/ports-manager/releases).

## Usage

### Default Ports

Ports Manager comes with built-in default ports for 30+ common services that are automatically available. These are stored in `~/.config/ports-manager/defaults.toml` and are checked automatically when you use the `get` command.

**Available defaults include:**
- **Databases**: PostgreSQL (5432), MySQL (3306), MongoDB (27017), Redis (6379), Cassandra (9042), etc.
- **Message Brokers**: RabbitMQ (5672), Kafka (9092)
- **Monitoring**: Prometheus (9090), Grafana (3000)
- **Infrastructure**: Docker (2375), etcd (2379), Consul (8500), Vault (8200)
- **Search**: Elasticsearch (9200)
- **And many more...**

```bash
# Get a default service port (no setup needed!)
ports-manager get postgres
# Outputs: 5432

# Reset defaults to built-in values (if you've modified them)
ports-manager reset-defaults

# Sync defaults with newer versions (preserves ignored entries)
ports-manager sync-defaults

# Customize defaults by editing the file directly
vim ~/.config/ports-manager/defaults.toml
```

**Ignoring Specific Defaults:**

Add services to the `ignored_defaults` array in your `config.toml` to exclude them from defaults:

```toml
# config.toml
ignored_defaults = ["postgres", "mysql"]  # Won't use defaults for these

[[ports]]
name = "myapp"
port = 8080
description = "My custom app"
```

When a service is in `ignored_defaults`:
- ✅ Won't use the default port from `defaults.toml`
- ✅ Will auto-assign a new port if not in your custom config
- ✅ Useful for services you always want to customize

### Set a port mapping
```bash
# Single port
ports-manager set myapp 8080 --description "My application server"

# Port range
ports-manager set webservices 8000-8010 --description "Web services port range"
```

### Get a port value (shell-friendly)
```bash
# Get existing port
ENV_VAR=$(ports-manager get myapp)
echo $ENV_VAR  # Outputs: 8080

# Auto-assign if not configured
PORT=$(ports-manager get mynewapp)
# Automatically finds an available port, saves it, and returns it
# Outputs: 8001 (or next available port)

# Subsequent calls return the same port
PORT=$(ports-manager get mynewapp)
echo $PORT  # Outputs: 8001 (same port)

# Works with ranges too
PORT=$(ports-manager get webservices)
echo "Starting on port $PORT"  # Outputs: 8000-8010

```

### List all port mappings
```bash
ports-manager list
```

### Remove a port mapping
```bash
ports-manager remove myapp
```

## Configuration

Ports Manager uses two configuration files:

1. **`~/.config/ports-manager/config.toml`** - Your custom port mappings
2. **`~/.config/ports-manager/defaults.toml`** - Default ports for common services (automatically created)

When you run `get`, it:
1. Checks your custom config (`config.toml`)
2. Checks if the service is in `ignored_defaults`
3. Falls back to `defaults.toml` (if not ignored)
4. Auto-assigns a new port (if not found anywhere)

### config.toml format:

```toml
# Defaults to ignore (won't use defaults.toml for these)
ignored_defaults = ["postgres", "redis"]

# Single port
[[ports]]
name = "myapp"
port = 8080
description = "My application server"

# Port range
[[ports]]
name = "webservices"
description = "Web services port range"

[ports.port]
start = 8000
end = 8010
```

### defaults.toml format:

The defaults file is a clean mapping that syncs automatically. It's replaced during `sync-defaults`:

```toml
version = 1  # Version tracking

[[ports]]
name = "postgres"
port = 5432
description = "PostgreSQL database"

[[ports]]
name = "mysql"
port = 3306
description = "MySQL database"

# ... 28 more services
```

**Note:** This file is completely replaced during `sync-defaults`. To prevent using specific defaults, add them to `ignored_defaults` in `config.toml` instead of editing this file.

## Features

- **Pre-configured defaults**: 30+ common service ports automatically available (PostgreSQL, MySQL, Redis, Kafka, etc.)
- **Auto-sync with version updates**: Run `sync-defaults` to get new defaults from updates
- **Ignore specific defaults**: Use `ignored_defaults` array in config to exclude services
- **Two-tier configuration**: Custom ports in `config.toml` override defaults in `defaults.toml`
- **Clean sync**: `defaults.toml` is completely replaced - user customizations go in `config.toml`
- **Version tracking**: Defaults file tracks version for automatic sync
- Simple CLI interface using clap
- Configuration stored in standard dotfiles location
- Set, get, remove, and list port mappings
- Support for single ports and port ranges (e.g., 8000-8010)
- Auto-assignment: `get` automatically finds and assigns available ports if not configured
- Smart port allocation: avoids ports already in use by other services
- Shell-friendly output for easy variable capture
- Optional descriptions for each port
- Automatic config file creation on first use
- Cross-platform support (Linux, macOS, Windows)
- Fast and lightweight with minimal dependencies

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m '✨ Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure to:
- Update tests as appropriate
- Run `cargo fmt` and `cargo clippy` before committing
- Follow the existing code style

## Development

### Running Tests

```bash
cargo test
```

### Running Lints

```bash
cargo clippy --all-targets --all-features
cargo fmt -- --check
```

### Building for Release

```bash
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/mgorunuch/ports-manager/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mgorunuch/ports-manager/discussions)

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [toml](https://github.com/toml-rs/toml) - TOML parser
