use std::io::Write;

/// Integration tests for the application
/// 
/// These tests verify that all components work together correctly

#[test]
fn test_config_loading() {
    // Test that configuration can be loaded from default file
    use project_tpl::config::AppConfig;
    
    let config = AppConfig::new("config/default.toml", "development");
    assert!(config.is_ok(), "Should load configuration successfully");
    
    let config = config.unwrap();
    assert_eq!(config.app.name, "project-tpl");
    assert!(config.server.port > 0);
}

#[test]
fn test_custom_config_file() {
    use project_tpl::config::AppConfig;
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
version = "2.0.0"
environment = "test"

[server]
host = "0.0.0.0"
port = 3000
timeout = 60

[database]
url = "postgres://localhost/testdb"
max_connections = 20
min_connections = 5

[logging]
level = "trace"
format = "json"
file_output = true
console_output = false
        "#
    ).unwrap();

    let config = AppConfig::from_file(temp_file.path()).unwrap();
    assert_eq!(config.app.name, "test-app");
    assert_eq!(config.app.version, "2.0.0");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.max_connections, 20);
    assert_eq!(config.logging.level, "trace");
}

#[test]
fn test_config_validation() {
    use project_tpl::config::{AppConfig, AppSettings, ServerSettings, DatabaseSettings, LoggingSettings};
    
    let valid_config = AppConfig {
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
    
    assert!(valid_config.validate().is_ok());
}

#[test]
fn test_invalid_config_validation() {
    use project_tpl::config::{AppConfig, AppSettings, ServerSettings, DatabaseSettings, LoggingSettings};
    
    let invalid_config = AppConfig {
        app: AppSettings {
            name: "test".to_string(),
            version: "1.0".to_string(),
            environment: "test".to_string(),
        },
        server: ServerSettings {
            host: "localhost".to_string(),
            port: 0, // Invalid port
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
    
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_environment_override() {
    use project_tpl::config::AppConfig;
    
    // Test that environment-specific config overrides default
    let config = AppConfig::new("config/default.toml", "production");
    
    if config.is_ok() {
        let config = config.unwrap();
        // Production should override some settings
        assert_eq!(config.app.environment, "production");
    }
}
