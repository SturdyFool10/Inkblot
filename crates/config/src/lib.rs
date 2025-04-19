use serde::{Deserialize, Serialize};

/// Represents the main configuration structure for the application.
///
/// # Fields
/// * `network` - Contains the network interface configuration.
/// * `database_path` - The path to the database file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub network: InterfaceConfig,
    pub database_path: String,
}

/// Represents the configuration for a network interface.
///
/// # Fields
/// * `interface` - the ip address of the network interface. this can be:
/// `<gateway ip>`: only allows devices on the same network to connect.
/// `<specific ip>`: only allows that specific ip to connect.
/// `127.0.0.1 or localhost`: only allows the local machine to connect.
/// * `port` - The port number to be used for the network interface. can be any unsigned 16-bit integer as per the ipv4 or ipv6 standard.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterfaceConfig {
    pub interface: String,
    pub port: u16,
}

/// Provides a default implementation for the `InterfaceConfig` struct.
///
/// # Default Values
/// * `interface` - Defaults to `"0.0.0.0"`, which allows connections from any IP address.
/// * `port` - Defaults to `3250`, which is the default port for the application.
impl Default for InterfaceConfig {
    fn default() -> Self {
        Self {
            interface: "0.0.0.0".to_string(),
            port: 3250,
        }
    }
}

/// Provides a default implementation for the `Config` struct.
///
/// # Default Values
/// * `network` - Defaults to the `InterfaceConfig` default implementation.
/// * `database_path` - Defaults to `"database.db"`, which is the default database file path.
impl Default for Config {
    fn default() -> Self {
        Self {
            network: InterfaceConfig::default(),
            database_path: "database.db".to_string(),
        }
    }
}

use std::fs;
use std::path::Path;

/// Loads a configuration from a JSON file at the specified path or creates a default configuration
/// file if it does not exist.
///
/// # Arguments
/// * `path` - The path to the configuration file as a string.
///
/// # Returns
/// * `Config` - The loaded or newly created configuration.
///
/// # Behavior
/// * If the file at the given path does not exist:
///   - A default configuration is created.
///   - The necessary directory structure is created if it does not exist.
///   - The default configuration is serialized to JSON and written to the file.
/// * If the file exists:
///   - The file is read and its contents are parsed as JSON into a `Config` struct.
///
/// # Panics
/// This function will panic if:
/// * The directory structure cannot be created.
/// * The default configuration cannot be serialized to JSON.
/// * The configuration file cannot be written.
/// * The configuration file cannot be read.
/// * The file contents cannot be parsed as valid JSON.
///
/// # Example
/// ```
/// use config::{load_config};
///
/// let config = load_config("config.json");
/// println!("Loaded configuration: {:?}", config);
/// ```
pub fn load_config(path: &str) -> Config {
    let path = Path::new(path);

    // Create default config
    let config = if !path.exists() {
        let default_config = Config::default();

        // Create directory structure if needed
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap_or_else(|err| {
                    panic!("Failed to create directory for configuration file: {}", err)
                });
            }
        }

        // Write the default config to the file
        let json = serde_jsonrc::to_string_pretty(&default_config)
            .unwrap_or_else(|err| panic!("Failed to serialize default configuration: {}", err));

        fs::write(path, json)
            .unwrap_or_else(|err| panic!("Failed to write default configuration file: {}", err));

        default_config
    } else {
        // Read the existing file
        let content = fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("Failed to read configuration file: {}", err));

        // Parse the content as JSON
        // Parse the content as JSON
        serde_jsonrc::from_str(&content).unwrap_or_else(|err| {
            let error_msg = format!(
                "Configuration file contains invalid JSON: {}\n\
                     Please check the syntax of your configuration file. Common issues include:\n\
                     - Missing or extra commas\n\
                     - Unquoted string values\n\
                     - Missing closing brackets or braces",
                err
            );
            panic!("{}", error_msg)
        })
    };

    config
}
