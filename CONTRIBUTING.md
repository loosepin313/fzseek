# Contributing to fzseek

Thank you for your interest in contributing to fzseek! This document provides guidelines and best practices for contributing to the project.

## How to Contribute

### Reporting Issues
Before submitting an issue, please:
1. Check if the issue has already been reported
2. Create a minimal reproduction case
3. Include relevant information like:
   - Rust version
   - Operating system
   - Steps to reproduce the issue

### Suggesting Features
When suggesting new features, please:
1. Explain the use case
2. Describe how it would benefit users
3. Consider the impact on existing functionality

### Submitting Pull Requests
When submitting a pull request, please:
1. Ensure your code follows the existing code style
2. Add or update documentation as needed
3. Include tests for new functionality
4. Provide a clear description of changes
5. Reference any related issues

## Development Setup

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Formatting
```bash
cargo fmt
```

## Code Style Guidelines

### Rust Conventions
- Follow Rust best practices and idioms
- Use descriptive variable and function names
- Include documentation comments for public APIs
- Handle errors gracefully with proper error types

### Documentation
- Update documentation when changing functionality
- Keep examples in documentation up-to-date
- Document configuration options clearly

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run tests to ensure nothing is broken
6. Submit a pull request with a clear description

## License

By contributing to fzseek, you agree that your contributions will be licensed under the MIT License.