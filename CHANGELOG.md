# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release
- CLI interface for managing port mappings
- Support for single ports and port ranges
- Auto-assignment of available ports
- Two-tier configuration system:
  - `config.toml` - User custom port mappings
  - `defaults.toml` - Pre-configured defaults (30+ common services)
- Commands: `set`, `get`, `list`, `remove`, `reset-defaults`, `sync-defaults`
- **NEW: Automatic defaults** - 30+ common service ports available out-of-the-box:
  - Databases: PostgreSQL, MySQL, MongoDB, Redis, Cassandra, CouchDB, InfluxDB, Neo4j, ClickHouse
  - Message Brokers: RabbitMQ, Kafka, ZooKeeper
  - Caching: Redis, Memcached
  - Search: Elasticsearch
  - Monitoring: Prometheus, Grafana
  - Infrastructure: Docker, etcd, Consul, Vault
  - CI/CD: Jenkins, SonarQube
  - Storage: MinIO
  - Network: HTTP, HTTPS, SSH, FTP, SMTP, DNS
- **NEW: `reset-defaults` command** - Reset `defaults.toml` to built-in values
- **NEW: `sync-defaults` command** - Completely replaces defaults with latest version
- **NEW: `ignored_defaults` array** - List services in `config.toml` to exclude from defaults
- **NEW: Version tracking** - Defaults file includes version for automatic sync
- **Clean separation** - User config in `config.toml`, synced defaults in `defaults.toml`
- Optional descriptions for port mappings
- Cross-platform support (Linux, macOS, Windows)

## [0.1.0] - 2025-11-11

### Added
- Initial project setup
- Basic CLI functionality
- Configuration management
- Port finder logic
- Comprehensive test suite
- GitHub Actions CI/CD pipelines
- Security audit workflows
- Documentation and contributing guidelines

[Unreleased]: https://github.com/mgorunuch/ports-manager/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mgorunuch/ports-manager/releases/tag/v0.1.0
