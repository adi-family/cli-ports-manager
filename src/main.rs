mod config;
mod port_finder;

use clap::{Parser, Subcommand};
use config::{Config, DefaultsConfig, PortSpec};

#[derive(Parser)]
#[command(name = "ports-manager")]
#[command(about = "A simple port management tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set a port mapping
    Set {
        /// Name of the service/application
        name: String,
        /// Port number or range (e.g., 8080 or 8000-8010)
        port: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Get a port value (outputs just the port for shell capture)
    Get {
        /// Name of the service/application
        name: String,
    },
    /// Remove a port mapping
    Remove {
        /// Name of the service/application
        name: String,
    },
    /// List all port mappings
    List,
    /// Reset defaults.toml to built-in default port mappings
    ResetDefaults,
    /// Sync defaults.toml with newer versions (preserves ignored entries)
    SyncDefaults,
}

fn main() {
    let cli = Cli::parse();

    let mut config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Set {
            name,
            port,
            description,
        } => {
            let port_spec = match PortSpec::parse(&port) {
                Ok(spec) => spec,
                Err(e) => {
                    eprintln!("Error parsing port: {}", e);
                    std::process::exit(1);
                }
            };
            config.add_port(name.clone(), port_spec.clone(), description);
            if let Err(e) = config.save() {
                eprintln!("Error saving config: {}", e);
                std::process::exit(1);
            }
            eprintln!("Set port mapping: {} -> {}", name, port_spec.display());
        }
        Commands::Get { name } => {
            if let Some(mapping) = config.find_port(&name) {
                println!("{}", mapping.port.display());
            } else {
                // Port not found - auto-assign a new one
                let used_ports = config.get_used_ports();
                match port_finder::find_available_port(&used_ports) {
                    Some(port) => {
                        let port_spec = PortSpec::Single(port);
                        config.add_port(name.clone(), port_spec.clone(), None);
                        if let Err(e) = config.save() {
                            eprintln!("Error saving config: {}", e);
                            std::process::exit(1);
                        }
                        eprintln!("Auto-assigned port for '{}': {}", name, port);
                        println!("{}", port);
                    }
                    None => {
                        eprintln!("No available ports found");
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Remove { name } => {
            if config.remove_port(&name) {
                if let Err(e) = config.save() {
                    eprintln!("Error saving config: {}", e);
                    std::process::exit(1);
                }
                eprintln!("Removed port mapping: {}", name);
            } else {
                eprintln!("Port mapping not found: {}", name);
                std::process::exit(1);
            }
        }
        Commands::List => {
            let ports = config.list_ports();
            if ports.is_empty() {
                println!("No port mappings configured");
            } else {
                println!("{:<20} {:<15} Description", "Name", "Port");
                println!("{}", "-".repeat(60));
                for mapping in ports {
                    println!(
                        "{:<20} {:<15} {}",
                        mapping.name,
                        mapping.port.display(),
                        mapping.description.as_deref().unwrap_or("-")
                    );
                }
            }
        }
        Commands::ResetDefaults => {
            if let Err(e) = DefaultsConfig::reset() {
                eprintln!("Error resetting defaults: {}", e);
                std::process::exit(1);
            }
            eprintln!("Default ports reset successfully");
            eprintln!("File location: ~/.config/ports-manager/defaults.toml");
            eprintln!("You can edit this file to customize default port mappings");
        }
        Commands::SyncDefaults => match DefaultsConfig::sync() {
            Ok(diff) => {
                eprintln!("Defaults synchronized successfully");
                if diff > 0 {
                    eprintln!("  - {} new default(s) added", diff);
                } else if diff < 0 {
                    eprintln!("  - {} default(s) removed", -diff);
                } else {
                    eprintln!("  - No changes (already up to date)");
                }
                eprintln!("\nFile location: ~/.config/ports-manager/defaults.toml");
                eprintln!("\nTip: Add defaults to 'ignored_defaults' array in config.toml to exclude them");
            }
            Err(e) => {
                eprintln!("Error syncing defaults: {}", e);
                std::process::exit(1);
            }
        },
    }
}
