# Ports Manager

A simple Rust CLI tool for managing port mappings. Configuration is stored in dotfiles at `~/.config/ports-manager/config.toml`.

## Installation

```bash
cargo build --release
cargo install --path .
```

## Usage

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

Configuration is automatically stored in `~/.config/ports-manager/config.toml` in TOML format:

```toml
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

## Features

- Simple CLI interface using clap
- Configuration stored in standard dotfiles location
- Set, get, remove, and list port mappings
- Support for single ports and port ranges (e.g., 8000-8010)
- Auto-assignment: `get` automatically finds and assigns available ports if not configured
- Smart port allocation: avoids ports already in use by other services in the config
- Shell-friendly output for easy variable capture
- Optional descriptions for each port
- Automatic config file creation on first use
