use anyhow::{Context, Result};
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

use crate::config::LoggingSettings;

/// Initialize the logging system based on configuration
pub fn init(config: &LoggingSettings) -> Result<()> {
    // Validate log level
    let _log_level = parse_log_level(&config.level)?;
    
    // Create environment filter
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&config.level))
        .context("Failed to create environment filter")?;

    let mut layers = Vec::new();

    // Console output layer
    if config.console_output {
        let console_layer = match config.format.as_str() {
            "json" => fmt::layer()
                .json()
                .with_span_events(FmtSpan::CLOSE)
                .with_filter(env_filter.clone())
                .boxed(),
            _ => fmt::layer()
                .with_span_events(FmtSpan::CLOSE)
                .with_filter(env_filter.clone())
                .boxed(),
        };
        layers.push(console_layer);
    }

    // File output layer
    if config.file_output {
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("app")
            .filename_suffix("log")
            .build("logs")
            .context("Failed to create file appender")?;

        let file_layer = match config.format.as_str() {
            "json" => fmt::layer()
                .json()
                .with_writer(file_appender)
                .with_span_events(FmtSpan::CLOSE)
                .with_ansi(false)
                .with_filter(env_filter.clone())
                .boxed(),
            _ => fmt::layer()
                .with_writer(file_appender)
                .with_span_events(FmtSpan::CLOSE)
                .with_ansi(false)
                .with_filter(env_filter.clone())
                .boxed(),
        };
        layers.push(file_layer);
    }

    // Initialize the subscriber
    tracing_subscriber::registry()
        .with(layers)
        .try_init()
        .context("Failed to initialize tracing subscriber")?;

    Ok(())
}

/// Parse log level from string
fn parse_log_level(level: &str) -> Result<Level> {
    match level.to_lowercase().as_str() {
        "trace" => Ok(Level::TRACE),
        "debug" => Ok(Level::DEBUG),
        "info" => Ok(Level::INFO),
        "warn" => Ok(Level::WARN),
        "error" => Ok(Level::ERROR),
        _ => anyhow::bail!("Invalid log level: {}", level),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_level() {
        assert!(matches!(parse_log_level("trace"), Ok(Level::TRACE)));
        assert!(matches!(parse_log_level("debug"), Ok(Level::DEBUG)));
        assert!(matches!(parse_log_level("info"), Ok(Level::INFO)));
        assert!(matches!(parse_log_level("warn"), Ok(Level::WARN)));
        assert!(matches!(parse_log_level("error"), Ok(Level::ERROR)));
        assert!(matches!(parse_log_level("DEBUG"), Ok(Level::DEBUG)));
        assert!(parse_log_level("invalid").is_err());
    }

    #[test]
    fn test_logging_config() {
        let config = LoggingSettings {
            level: "info".to_string(),
            format: "text".to_string(),
            file_output: false,
            console_output: true,
        };

        // Test that configuration is valid
        assert_eq!(config.level, "info");
        assert_eq!(config.format, "text");
    }
}
