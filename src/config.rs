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
            let start = start_str.trim().parse::<u16>()
                .map_err(|_| format!("Invalid start port: {}", start_str))?;
            let end = end_str.trim().parse::<u16>()
                .map_err(|_| format!("Invalid end port: {}", end_str))?;

            if start >= end {
                return Err("Start port must be less than end port".to_string());
            }

            Ok(PortSpec::Range { start, end })
        } else {
            let port = input.trim().parse::<u16>()
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
    pub ports: Vec<PortMapping>,
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

        Ok(home_dir.join(".config").join("ports-manager").join("config.toml"))
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

    pub fn find_port(&self, name: &str) -> Option<&PortMapping> {
        self.ports.iter().find(|p| p.name == name)
    }

    pub fn list_ports(&self) -> &[PortMapping] {
        &self.ports
    }

    pub fn get_used_ports(&self) -> Vec<u16> {
        let mut used_ports = Vec::new();
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
        used_ports
    }
}
