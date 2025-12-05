use anyhow::{Context, Result};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String,
    pub file_output: bool,
    pub console_output: bool,
}

impl AppConfig {
    /// Create a new configuration from files
    /// 
    /// # Arguments
    /// * `config_path` - Path to the base configuration file
    /// * `environment` - Environment name (development, production, etc.)
    pub fn new(config_path: &str, environment: &str) -> Result<Self> {
        let config = Config::builder()
            // Start with default configuration
            .add_source(File::with_name(config_path).required(true))
            // Layer environment-specific configuration
            .add_source(
                File::with_name(&format!("config/{}", environment))
                    .required(false)
            )
            // Add environment variables with APP_ prefix
            // e.g., APP_SERVER__PORT=8080 will set server.port
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
            .context("Failed to build configuration")?;

        config
            .try_deserialize()
            .context("Failed to deserialize configuration")
    }

    /// Load configuration from a specific file
    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = Config::builder()
            .add_source(File::from(path.as_ref()))
            .build()
            .context("Failed to build configuration from file")?;

        config
            .try_deserialize()
            .context("Failed to deserialize configuration")
    }

    /// Validate configuration
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<()> {
        // Add custom validation logic here
        if self.server.port == 0 {
            anyhow::bail!("Server port must be greater than 0");
        }

        if self.database.max_connections < self.database.min_connections {
            anyhow::bail!("Max connections must be >= min connections");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_from_file() {
        use tempfile::Builder;
        
        let mut temp_file = Builder::new()
            .suffix(".toml")
            .tempfile()
            .unwrap();
        writeln!(
            temp_file,
            r#"
[app]
name = "test-app"
version = "1.0.0"
environment = "test"

[server]
host = "localhost"
port = 9000
timeout = 30

[database]
url = "postgres://localhost/test"
max_connections = 5
min_connections = 1

[logging]
level = "debug"
format = "text"
file_output = false
console_output = true
            "#
        )
        .unwrap();

        let config = AppConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.app.name, "test-app");
        assert_eq!(config.server.port, 9000);
        assert_eq!(config.database.max_connections, 5);
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig {
            app: AppSettings {
                name: "test".to_string(),
                version: "1.0".to_string(),
                environment: "test".to_string(),
            },
            server: ServerSettings {
                host: "localhost".to_string(),
                port: 8080,
                timeout: 30,
            },
            database: DatabaseSettings {
                url: "postgres://localhost/db".to_string(),
                max_connections: 10,
                min_connections: 2,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                format: "json".to_string(),
                file_output: true,
                console_output: true,
            },
        };

        assert!(config.validate().is_ok());

        // Test invalid configuration
        config.database.max_connections = 1;
        config.database.min_connections = 5;
        assert!(config.validate().is_err());
    }
}
