# fzseek

A kind of cmdb cli with a fuzzy finder front end with many connectors to many datasources 

## Features

- Fuzzy search functionality for finding resources quickly
- Command-line interface with version support
- Configuration file support
- Extensible architecture for multiple data sources

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
``` 
