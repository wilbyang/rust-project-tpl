use anyhow::Result;
use clap::Parser;
use tracing::info;

mod cli;
mod config;
mod logging;

use cli::Cli;
use config::AppConfig;

fn main() -> Result<()> {
    // 1. Parse command-line arguments
    let cli = Cli::parse();
    
    // 2. Load configuration
    let config = AppConfig::new(&cli.config, &cli.env)?;
    
    // 3. Initialize logging
    logging::init(&config.logging)?;
    
    // Log application startup
    info!(
        app_name = config.app.name,
        version = config.app.version,
        environment = config.app.environment,
        "Application started"
    );
    
    // 4. Run application logic
    run_application(&cli, &config)?;
    
    info!("Application finished successfully");
    Ok(())
}

fn run_application(cli: &Cli, config: &AppConfig) -> Result<()> {
    // Example application logic
    if cli.verbose {
        info!("Verbose mode enabled");
        info!("Configuration: {:?}", config);
    }
    
    info!(
        host = config.server.host,
        port = config.server.port,
        "Server configuration loaded"
    );
    
    // Add your application logic here
    info!("Running main application logic...");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_application_runs() {
        // Test basic application setup
        assert!(true);
    }
}
