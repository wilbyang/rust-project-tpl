# Rust Project Template

A production-ready Rust project template with essential components:
- ✅ **Command-line Arguments** (using clap)
- ✅ **Configuration Management** (using config)
- ✅ **Structured Logging** (using tracing)
- ✅ **Comprehensive Tests** (unit & integration)

## Features

### 1. Command-Line Interface (CLI)
- Built with `clap` using derive macros
- Support for subcommands
- Environment variable integration
- Automatic help generation

### 2. Configuration Management
- Multiple environment support (development, production, etc.)
- TOML-based configuration files
- Environment variable overrides
- Configuration validation

### 3. Logging System
- Structured logging with `tracing`
- Multiple output formats (JSON, text)
- Console and file output
- Daily log rotation
- Configurable log levels

### 4. Testing
- Unit tests in modules
- Integration tests
- Configuration testing
- Example test patterns

## Quick Start

### Installation
```bash
# Clone or copy this template
cargo generate --git <your-repo-url>

# Or use as template
cargo init --name myapp
# Then copy files from this template
```

### Build
```bash
cargo build --release
```

### Run
```bash
# Run with default configuration
cargo run

# Run with custom environment
cargo run -- --env production

# Run with verbose output
cargo run -- --verbose

# Show help
cargo run -- --help
```

### Test
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_config_loading
```

## Project Structure

```
project-tpl/
├── config/                 # Configuration files
│   ├── default.toml       # Default configuration
│   ├── development.toml   # Development overrides
│   └── production.toml    # Production overrides
├── logs/                  # Log output directory
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── cli.rs            # CLI argument parsing
│   ├── config.rs         # Configuration management
│   └── logging.rs        # Logging setup
├── tests/
│   └── integration_test.rs # Integration tests
└── Cargo.toml            # Dependencies
```

## Configuration

### Configuration Files
Configuration is layered:
1. `config/default.toml` - Base configuration
2. `config/{environment}.toml` - Environment-specific overrides
3. Environment variables with `APP_` prefix

### Environment Variables
```bash
# Override server port
export APP_SERVER__PORT=9090

# Override log level
export LOG_LEVEL=debug

# Set environment
export APP_ENV=production
```

## Usage Examples

### Basic Usage
```rust
use project_tpl::config::AppConfig;

fn main() -> anyhow::Result<()> {
    let config = AppConfig::new("config/default.toml", "development")?;
    println!("Server running on {}:{}", config.server.host, config.server.port);
    Ok(())
}
```

### CLI Commands
```bash
# Run with subcommand
cargo run -- serve --port 8080

# Show current configuration
cargo run -- config

# Run database migrations
cargo run -- migrate
```

### Logging
```rust
use tracing::{info, warn, error};

info!("Application started");
warn!(port = 8080, "Using custom port");
error!(error = ?err, "Failed to connect");
```

## Customization

### Adding New Configuration
Edit `src/config.rs` to add new configuration sections:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySettings {
    pub field: String,
}
```

Then update `config/default.toml`:
```toml
[my_settings]
field = "value"
```

### Adding CLI Arguments
Edit `src/cli.rs` to add new arguments:
```rust
#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub my_arg: String,
}
```

### Adding Tests
Add unit tests in the same file:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
        assert!(true);
    }
}
```

Add integration tests in `tests/` directory.

## Dependencies

- **clap** - Command-line argument parsing
- **config** - Configuration management
- **serde** - Serialization/deserialization
- **tracing** - Structured logging
- **anyhow** - Error handling
- **thiserror** - Custom error types

## Best Practices

1. **Configuration**: Always validate configuration on load
2. **Logging**: Use structured logging with context
3. **Testing**: Write tests for all public APIs
4. **Errors**: Use `anyhow::Result` for applications, `thiserror` for libraries
5. **CLI**: Provide helpful descriptions for all arguments

## License

MIT OR Apache-2.0

## Contributing

Feel free to customize this template for your needs!
