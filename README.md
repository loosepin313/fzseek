# fzseek

A command-line tool with a fuzzy finder frontend for searching resources across multiple data sources.

## Features

- Fuzzy search functionality for finding resources quickly
- Command-line interface with version support
- Configuration file support
- Extensible architecture for multiple data sources
- Sample data implementation for demonstration

## Contributors
- Chris Harris (chris.m.harris)

## Building

To build this project, you'll need Rust installed on your system. If you don't have Rust installed, you can install it from [https://rustup.rs/](https://rustup.rs/).

Once Rust is installed, you can build the project with:

```bash
cargo build
```

To build in release mode for better performance:

```bash
cargo build --release
```

## Running

To run the application:

```bash
cargo run
```

To run with version information:

```bash
cargo run -- --version
```

To perform a fuzzy search:

```bash
cargo run -- [search_term]
```

Example:
```bash
cargo run -- database
```

## Configuration

The application will automatically create a default configuration file at `~/.config/fzseek/config.yaml` on first run. The configuration file supports:

- General settings: search mode, max results, show hidden files
- UI settings: theme, color output, animations
- Connectors: configuration for data source connectors

## Testing

To run unit tests:

```bash
cargo test
```

## Example Usage

```bash
# Show version information
cargo run -- --version

# Run the application
cargo run

# Perform a fuzzy search
cargo run -- database

# Search with different query terms
cargo run -- web
cargo run -- load
```

## Implementation Status

The current implementation includes:
- Fuzzy search functionality using the fuzzy-matcher crate
- Command-line interface with clap
- Configuration file support with serde_yaml
- Sample data for demonstration (will be extended with real connectors)
- Basic structure for extensible data source connectors

Note: While the architecture supports multiple data sources, the current implementation uses sample data for demonstration purposes.
``` 
