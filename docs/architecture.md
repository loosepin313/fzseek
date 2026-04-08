# fzseek Architecture Documentation

## Overview

fzseek is a command-line tool that provides a fuzzy finder interface for searching resources across multiple data sources. The tool is built with Rust and uses a modular architecture that supports extensible connectors.

## Core Components

### 1. Command-Line Interface
Built with clap for robust argument parsing and help system.

### 2. Fuzzy Search Engine
Utilizes the fuzzy-matcher crate for efficient fuzzy matching algorithms.

### 3. Configuration System
Supports YAML configuration files with serde for serialization and deserialization.

### 4. Data Connectors
Extensible architecture designed to support multiple data sources (currently using sample data).

## Project Structure

```
fzseek/
├── src/
│   ├── main.rs          # Main application entry point
│   └── ...              # Additional modules
├── Cargo.toml           # Project configuration and dependencies
├── README.md            # Project overview and usage
├── things2do.md         # Development progress tracking
├── default_config.yaml  # Default configuration template
└── docs/                # Documentation directory
    └── architecture.md  # This file
```

## Configuration File Structure

The configuration file (`~/.config/fzseek/config.yaml`) supports three main sections:

### General Settings
- `search_mode`: "fuzzy", "exact", or "regex"
- `max_results`: Maximum number of results to display
- `show_hidden`: Whether to show hidden files in searches

### UI Settings
- `theme`: UI theme name
- `color_enabled`: Whether to enable color output
- `animations_enabled`: Whether to enable animations

### Connectors
- Placeholder for data source connector configurations

## Implementation Details

### Main Application Flow
1. Parse command-line arguments
2. Load configuration from file or create default
3. If version flag is set, display version and exit
4. If search query is provided, perform fuzzy search on data
5. Display results or show usage information

### Fuzzy Search Functionality
- Uses SkimMatcherV2 from fuzzy-matcher crate
- Results are scored and sorted by relevance
- Supports case-insensitive matching

### Data Handling
- Sample data is provided for demonstration
- Architecture designed to support real data sources through connectors
- Search results are returned as SearchResult structs with ID, name, and score

### Error Handling
- All file operations are wrapped in Result types
- Configuration parsing errors are logged to stderr
- Graceful degradation when connectors fail

## Extending the Application

### Adding New Data Sources
To add support for new data sources:
1. Create a new connector module
2. Implement the connector trait
3. Add configuration options to ConnectorsConfig struct
4. Update the main function to use the new connector

### Adding New Features
The modular design allows for adding new features without disrupting existing functionality.

## Testing Strategy
The project uses Rust's built-in testing framework with unit tests covering core functionality:
- Fuzzy search algorithm behavior
- Configuration file parsing
- Command-line argument parsing

## Contributing

Contributions are welcome! Please ensure new code follows the existing patterns and includes appropriate tests.

## Future Enhancements
- Implement data caching mechanism
- Add progress indicators for long operations
- Add support for additional data source connectors
- Improve configuration file schema validation