use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PortSpec {
    Single(u16),
    Range { start: u16, end: u16 },
}

impl PortSpec {
    pub fn parse(input: &str) -> Result<Self, String> {
        if let Some((start_str, end_str)) = input.split_once('-') {
            let start = start_str
                .trim()
                .parse::<u16>()
                .map_err(|_| format!("Invalid start port: {}", start_str))?;
            let end = end_str
                .trim()
                .parse::<u16>()
                .map_err(|_| format!("Invalid end port: {}", end_str))?;

            if start >= end {
                return Err("Start port must be less than end port".to_string());
            }

            Ok(PortSpec::Range { start, end })
        } else {
            let port = input
                .trim()
                .parse::<u16>()
                .map_err(|_| format!("Invalid port: {}", input))?;
            Ok(PortSpec::Single(port))
        }
    }

    pub fn display(&self) -> String {
        match self {
            PortSpec::Single(port) => port.to_string(),
            PortSpec::Range { start, end } => format!("{}-{}", start, end),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortMapping {
    pub name: String,
    pub port: PortSpec,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub ports: Vec<PortMapping>,
    #[serde(default)]
    pub ignored_defaults: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            // Create default config if it doesn't exist
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home_dir = directories::BaseDirs::new()
            .ok_or("Could not find home directory")?
            .home_dir()
            .to_path_buf();

        Ok(home_dir
            .join(".config")
            .join("ports-manager")
            .join("config.toml"))
    }

    fn config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home_dir = directories::BaseDirs::new()
            .ok_or("Could not find home directory")?
            .home_dir()
            .to_path_buf();

        Ok(home_dir.join(".config").join("ports-manager"))
    }

    pub fn add_port(&mut self, name: String, port: PortSpec, description: Option<String>) {
        self.ports.push(PortMapping {
            name,
            port,
            description,
        });
    }

    pub fn remove_port(&mut self, name: &str) -> bool {
        let original_len = self.ports.len();
        self.ports.retain(|p| p.name != name);
        self.ports.len() < original_len
    }

    pub fn find_port(&self, name: &str) -> Option<PortMapping> {
        // First check user config
        if let Some(mapping) = self.ports.iter().find(|p| p.name == name) {
            return Some(mapping.clone());
        }

        // Check if this default is ignored
        if self.ignored_defaults.contains(&name.to_string()) {
            return None;
        }

        // Then check defaults
        if let Ok(defaults) = DefaultsConfig::load() {
            if let Some(mapping) = defaults.ports.iter().find(|p| p.name == name) {
                return Some(mapping.clone());
            }
        }

        None
    }

    pub fn list_ports(&self) -> &[PortMapping] {
        &self.ports
    }

    pub fn get_used_ports(&self) -> Vec<u16> {
        let mut used_ports = Vec::new();

        // Get ports from user config
        for mapping in &self.ports {
            match &mapping.port {
                PortSpec::Single(port) => used_ports.push(*port),
                PortSpec::Range { start, end } => {
                    for port in *start..=*end {
                        used_ports.push(port);
                    }
                }
            }
        }

        // Get ports from defaults
        if let Ok(defaults) = DefaultsConfig::load() {
            for mapping in &defaults.ports {
                match &mapping.port {
                    PortSpec::Single(port) => used_ports.push(*port),
                    PortSpec::Range { start, end } => {
                        for port in *start..=*end {
                            used_ports.push(port);
                        }
                    }
                }
            }
        }

        used_ports
    }
}

const DEFAULTS_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DefaultsConfig {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub ports: Vec<PortMapping>,
}

fn default_version() -> u32 {
    DEFAULTS_VERSION
}

impl DefaultsConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let defaults_path = Self::defaults_path()?;

        if !defaults_path.exists() {
            // Create default defaults if they don't exist
            let default_config = Self::create_defaults();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&defaults_path)?;
        let config: DefaultsConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let defaults_path = Self::defaults_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = defaults_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&defaults_path, content)?;
        Ok(())
    }

    fn defaults_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(Config::config_dir()?.join("defaults.toml"))
    }

    pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
        let defaults = Self::create_defaults();
        defaults.save()
    }

    pub fn sync() -> Result<isize, Box<dyn std::error::Error>> {
        let defaults_path = Self::defaults_path()?;
        let built_in_defaults = Self::create_defaults();

        let old_count = if defaults_path.exists() {
            Self::load().map(|d| d.ports.len()).unwrap_or(0)
        } else {
            0
        };

        // Just replace with new defaults
        built_in_defaults.save()?;

        let new_count = built_in_defaults.ports.len();
        Ok(new_count as isize - old_count as isize)
    }

    fn create_defaults() -> Self {
        let mut config = DefaultsConfig::default();

        // Databases
        config.ports.push(PortMapping {
            name: "postgres".to_string(),
            port: PortSpec::Single(5432),
            description: Some("PostgreSQL database".to_string()),
        });
        config.ports.push(PortMapping {
            name: "mysql".to_string(),
            port: PortSpec::Single(3306),
            description: Some("MySQL database".to_string()),
        });
        config.ports.push(PortMapping {
            name: "mongodb".to_string(),
            port: PortSpec::Single(27017),
            description: Some("MongoDB database".to_string()),
        });
        config.ports.push(PortMapping {
            name: "redis".to_string(),
            port: PortSpec::Single(6379),
            description: Some("Redis cache".to_string()),
        });
        config.ports.push(PortMapping {
            name: "elasticsearch".to_string(),
            port: PortSpec::Single(9200),
            description: Some("Elasticsearch search engine".to_string()),
        });
        config.ports.push(PortMapping {
            name: "cassandra".to_string(),
            port: PortSpec::Single(9042),
            description: Some("Apache Cassandra database".to_string()),
        });
        config.ports.push(PortMapping {
            name: "couchdb".to_string(),
            port: PortSpec::Single(5984),
            description: Some("Apache CouchDB database".to_string()),
        });
        config.ports.push(PortMapping {
            name: "influxdb".to_string(),
            port: PortSpec::Single(8086),
            description: Some("InfluxDB time-series database".to_string()),
        });
        config.ports.push(PortMapping {
            name: "neo4j".to_string(),
            port: PortSpec::Single(7687),
            description: Some("Neo4j graph database".to_string()),
        });
        config.ports.push(PortMapping {
            name: "clickhouse".to_string(),
            port: PortSpec::Single(9000),
            description: Some("ClickHouse analytics database".to_string()),
        });

        // Message Brokers
        config.ports.push(PortMapping {
            name: "rabbitmq".to_string(),
            port: PortSpec::Single(5672),
            description: Some("RabbitMQ message broker".to_string()),
        });
        config.ports.push(PortMapping {
            name: "kafka".to_string(),
            port: PortSpec::Single(9092),
            description: Some("Apache Kafka message broker".to_string()),
        });
        config.ports.push(PortMapping {
            name: "zookeeper".to_string(),
            port: PortSpec::Single(2181),
            description: Some("Apache ZooKeeper coordination service".to_string()),
        });

        // Caching
        config.ports.push(PortMapping {
            name: "memcached".to_string(),
            port: PortSpec::Single(11211),
            description: Some("Memcached cache".to_string()),
        });

        // Monitoring
        config.ports.push(PortMapping {
            name: "prometheus".to_string(),
            port: PortSpec::Single(9090),
            description: Some("Prometheus monitoring".to_string()),
        });
        config.ports.push(PortMapping {
            name: "grafana".to_string(),
            port: PortSpec::Single(3000),
            description: Some("Grafana dashboard".to_string()),
        });

        // Infrastructure
        config.ports.push(PortMapping {
            name: "docker".to_string(),
            port: PortSpec::Single(2375),
            description: Some("Docker daemon".to_string()),
        });
        config.ports.push(PortMapping {
            name: "etcd".to_string(),
            port: PortSpec::Single(2379),
            description: Some("etcd distributed key-value store".to_string()),
        });
        config.ports.push(PortMapping {
            name: "consul".to_string(),
            port: PortSpec::Single(8500),
            description: Some("Consul service mesh".to_string()),
        });
        config.ports.push(PortMapping {
            name: "vault".to_string(),
            port: PortSpec::Single(8200),
            description: Some("HashiCorp Vault secrets management".to_string()),
        });

        // CI/CD
        config.ports.push(PortMapping {
            name: "jenkins".to_string(),
            port: PortSpec::Single(8080),
            description: Some("Jenkins CI/CD".to_string()),
        });
        config.ports.push(PortMapping {
            name: "sonarqube".to_string(),
            port: PortSpec::Single(9000),
            description: Some("SonarQube code quality".to_string()),
        });

        // Storage
        config.ports.push(PortMapping {
            name: "minio".to_string(),
            port: PortSpec::Single(9000),
            description: Some("MinIO object storage".to_string()),
        });

        // Network Services
        config.ports.push(PortMapping {
            name: "http".to_string(),
            port: PortSpec::Single(80),
            description: Some("HTTP web server".to_string()),
        });
        config.ports.push(PortMapping {
            name: "https".to_string(),
            port: PortSpec::Single(443),
            description: Some("HTTPS web server".to_string()),
        });
        config.ports.push(PortMapping {
            name: "ssh".to_string(),
            port: PortSpec::Single(22),
            description: Some("SSH server".to_string()),
        });
        config.ports.push(PortMapping {
            name: "ftp".to_string(),
            port: PortSpec::Single(21),
            description: Some("FTP server".to_string()),
        });
        config.ports.push(PortMapping {
            name: "smtp".to_string(),
            port: PortSpec::Single(25),
            description: Some("SMTP mail server".to_string()),
        });
        config.ports.push(PortMapping {
            name: "dns".to_string(),
            port: PortSpec::Single(53),
            description: Some("DNS server".to_string()),
        });

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portspec_parse_single_port() {
        let result = PortSpec::parse("8080");
        assert!(result.is_ok());
        match result.unwrap() {
            PortSpec::Single(port) => assert_eq!(port, 8080),
            _ => panic!("Expected Single port"),
        }
    }

    #[test]
    fn test_portspec_parse_single_port_with_whitespace() {
        let result = PortSpec::parse("  8080  ");
        assert!(result.is_ok());
        match result.unwrap() {
            PortSpec::Single(port) => assert_eq!(port, 8080),
            _ => panic!("Expected Single port"),
        }
    }

    #[test]
    fn test_portspec_parse_range() {
        let result = PortSpec::parse("8000-8010");
        assert!(result.is_ok());
        match result.unwrap() {
            PortSpec::Range { start, end } => {
                assert_eq!(start, 8000);
                assert_eq!(end, 8010);
            }
            _ => panic!("Expected Range"),
        }
    }

    #[test]
    fn test_portspec_parse_range_with_whitespace() {
        let result = PortSpec::parse(" 8000 - 8010 ");
        assert!(result.is_ok());
        match result.unwrap() {
            PortSpec::Range { start, end } => {
                assert_eq!(start, 8000);
                assert_eq!(end, 8010);
            }
            _ => panic!("Expected Range"),
        }
    }

    #[test]
    fn test_portspec_parse_invalid_port() {
        let result = PortSpec::parse("invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid port"));
    }

    #[test]
    fn test_portspec_parse_invalid_range_start() {
        let result = PortSpec::parse("invalid-8010");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid start port"));
    }

    #[test]
    fn test_portspec_parse_invalid_range_end() {
        let result = PortSpec::parse("8000-invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid end port"));
    }

    #[test]
    fn test_portspec_parse_range_start_equals_end() {
        let result = PortSpec::parse("8000-8000");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Start port must be less than end port"));
    }

    #[test]
    fn test_portspec_parse_range_start_greater_than_end() {
        let result = PortSpec::parse("8010-8000");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Start port must be less than end port"));
    }

    #[test]
    fn test_portspec_parse_out_of_range() {
        let result = PortSpec::parse("99999");
        assert!(result.is_err());
    }

    #[test]
    fn test_portspec_parse_negative() {
        let result = PortSpec::parse("-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_portspec_parse_zero() {
        let result = PortSpec::parse("0");
        assert!(result.is_ok());
        match result.unwrap() {
            PortSpec::Single(port) => assert_eq!(port, 0),
            _ => panic!("Expected Single port"),
        }
    }

    #[test]
    fn test_portspec_parse_max_port() {
        let result = PortSpec::parse("65535");
        assert!(result.is_ok());
        match result.unwrap() {
            PortSpec::Single(port) => assert_eq!(port, 65535),
            _ => panic!("Expected Single port"),
        }
    }

    #[test]
    fn test_portspec_display_single() {
        let spec = PortSpec::Single(8080);
        assert_eq!(spec.display(), "8080");
    }

    #[test]
    fn test_portspec_display_range() {
        let spec = PortSpec::Range {
            start: 8000,
            end: 8010,
        };
        assert_eq!(spec.display(), "8000-8010");
    }

    #[test]
    fn test_config_add_port() {
        let mut config = Config::default();
        config.add_port(
            "test".to_string(),
            PortSpec::Single(8080),
            Some("Test port".to_string()),
        );

        assert_eq!(config.ports.len(), 1);
        assert_eq!(config.ports[0].name, "test");
        assert_eq!(config.ports[0].description, Some("Test port".to_string()));
    }

    #[test]
    fn test_config_add_multiple_ports() {
        let mut config = Config::default();
        config.add_port("test1".to_string(), PortSpec::Single(8080), None);
        config.add_port("test2".to_string(), PortSpec::Single(8081), None);
        config.add_port(
            "test3".to_string(),
            PortSpec::Range {
                start: 8100,
                end: 8110,
            },
            None,
        );

        assert_eq!(config.ports.len(), 3);
    }

    #[test]
    fn test_config_remove_port_existing() {
        let mut config = Config::default();
        config.add_port("test".to_string(), PortSpec::Single(8080), None);

        let removed = config.remove_port("test");
        assert!(removed);
        assert_eq!(config.ports.len(), 0);
    }

    #[test]
    fn test_config_remove_port_non_existing() {
        let mut config = Config::default();
        config.add_port("test".to_string(), PortSpec::Single(8080), None);

        let removed = config.remove_port("nonexistent");
        assert!(!removed);
        assert_eq!(config.ports.len(), 1);
    }

    #[test]
    fn test_config_remove_port_from_empty() {
        let mut config = Config::default();
        let removed = config.remove_port("test");
        assert!(!removed);
    }

    #[test]
    fn test_config_find_port_existing() {
        let mut config = Config::default();
        config.add_port("test".to_string(), PortSpec::Single(8080), None);

        let found = config.find_port("test");
        assert!(found.is_some());
        assert_eq!(found.as_ref().unwrap().name, "test");
    }

    #[test]
    fn test_config_find_port_non_existing() {
        let mut config = Config::default();
        config.add_port("test".to_string(), PortSpec::Single(8080), None);

        // Use a very specific name that won't be in defaults
        let found = config.find_port("this-definitely-does-not-exist-xyz123");
        // This specific name should not exist in defaults or config
        assert!(found.is_none());
    }

    #[test]
    fn test_config_list_ports_empty() {
        let config = Config::default();
        assert_eq!(config.list_ports().len(), 0);
    }

    #[test]
    fn test_config_list_ports() {
        let mut config = Config::default();
        config.add_port("test1".to_string(), PortSpec::Single(8080), None);
        config.add_port("test2".to_string(), PortSpec::Single(8081), None);

        let ports = config.list_ports();
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].name, "test1");
        assert_eq!(ports[1].name, "test2");
    }

    #[test]
    fn test_config_get_used_ports_empty() {
        let config = Config::default();
        let used = config.get_used_ports();
        // May include defaults if defaults.toml exists, or be empty if not
        // Just verify it doesn't crash and returns a valid Vec
        let _ = used;
    }

    #[test]
    fn test_config_get_used_ports_single() {
        let mut config = Config::default();
        config.add_port("test".to_string(), PortSpec::Single(8080), None);

        let used = config.get_used_ports();
        // Must include the port we added, may include defaults
        assert!(used.len() >= 1);
        assert!(used.contains(&8080));
    }

    #[test]
    fn test_config_get_used_ports_multiple_single() {
        let mut config = Config::default();
        config.add_port("test1".to_string(), PortSpec::Single(8080), None);
        config.add_port("test2".to_string(), PortSpec::Single(8081), None);
        config.add_port("test3".to_string(), PortSpec::Single(8082), None);

        let used = config.get_used_ports();
        // Must include the 3 ports we added, may include defaults
        assert!(used.len() >= 3);
        assert!(used.contains(&8080));
        assert!(used.contains(&8081));
        assert!(used.contains(&8082));
    }

    #[test]
    fn test_config_get_used_ports_range() {
        let mut config = Config::default();
        config.add_port(
            "test".to_string(),
            PortSpec::Range {
                start: 8000,
                end: 8010,
            },
            None,
        );

        let used = config.get_used_ports();
        // Must include all 11 ports from the range, may include defaults
        assert!(used.len() >= 11);
        for port in 8000..=8010 {
            assert!(used.contains(&port));
        }
    }

    #[test]
    fn test_config_get_used_ports_mixed() {
        let mut config = Config::default();
        config.add_port("single1".to_string(), PortSpec::Single(8080), None);
        config.add_port(
            "range1".to_string(),
            PortSpec::Range {
                start: 8000,
                end: 8002,
            },
            None,
        );
        config.add_port("single2".to_string(), PortSpec::Single(8090), None);

        let used = config.get_used_ports();
        // Must include 5 ports (2 singles + 3 from range), may include defaults
        assert!(used.len() >= 5);
        assert!(used.contains(&8080));
        assert!(used.contains(&8000));
        assert!(used.contains(&8001));
        assert!(used.contains(&8002));
        assert!(used.contains(&8090));
    }

    #[test]
    fn test_config_duplicate_names() {
        let mut config = Config::default();
        config.add_port("test".to_string(), PortSpec::Single(8080), None);
        config.add_port("test".to_string(), PortSpec::Single(8081), None);

        // Should allow duplicates (no uniqueness constraint)
        assert_eq!(config.ports.len(), 2);

        // Remove should remove all occurrences with matching name
        config.remove_port("test");
        assert_eq!(config.ports.len(), 0);
    }

    #[test]
    fn test_portspec_serialization_single() {
        let spec = PortSpec::Single(8080);
        let serialized = serde_json::to_string(&spec).unwrap();
        assert_eq!(serialized, "8080");
    }

    #[test]
    fn test_portspec_serialization_range() {
        let spec = PortSpec::Range {
            start: 8000,
            end: 8010,
        };
        let serialized = serde_json::to_string(&spec).unwrap();
        assert!(serialized.contains("\"start\":8000"));
        assert!(serialized.contains("\"end\":8010"));
    }

    #[test]
    fn test_portspec_deserialization_single() {
        let json = "8080";
        let spec: PortSpec = serde_json::from_str(json).unwrap();
        match spec {
            PortSpec::Single(port) => assert_eq!(port, 8080),
            _ => panic!("Expected Single port"),
        }
    }

    #[test]
    fn test_portspec_deserialization_range() {
        let json = r#"{"start":8000,"end":8010}"#;
        let spec: PortSpec = serde_json::from_str(json).unwrap();
        match spec {
            PortSpec::Range { start, end } => {
                assert_eq!(start, 8000);
                assert_eq!(end, 8010);
            }
            _ => panic!("Expected Range"),
        }
    }
}
