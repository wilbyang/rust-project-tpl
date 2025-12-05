use clap::Parser;

/// Project Template - A Rust application template with CLI, config, logging, and tests
#[derive(Parser, Debug)]
#[command(name = "project-tpl")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "Rust project template with essential components", long_about = None)]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config/default.toml")]
    pub config: String,

    /// Environment (development, production, test)
    #[arg(short, long, env = "APP_ENV", default_value = "development")]
    pub env: String,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Log level override (trace, debug, info, warn, error)
    #[arg(short, long, env = "LOG_LEVEL")]
    pub log_level: Option<String>,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Run the application in server mode
    Serve {
        /// Port to bind to
        #[arg(short, long)]
        port: Option<u16>,
    },
    
    /// Show current configuration
    Config,
    
    /// Run database migrations
    Migrate {
        /// Rollback migrations
        #[arg(short, long)]
        rollback: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test default values
        let cli = Cli::parse_from(&["project-tpl"]);
        assert_eq!(cli.config, "config/default.toml");
        assert_eq!(cli.env, "development");
        assert!(!cli.verbose);
    }

    #[test]
    fn test_cli_with_args() {
        let cli = Cli::parse_from(&[
            "project-tpl",
            "--config", "custom.toml",
            "--env", "production",
            "--verbose",
        ]);
        assert_eq!(cli.config, "custom.toml");
        assert_eq!(cli.env, "production");
        assert!(cli.verbose);
    }
}
