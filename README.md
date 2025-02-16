# Tygen

A powerful Rust CLI tool for automatically generating comprehensive test suites for Rust types.

## Features

- Trait implementation testing (Debug, Clone, PartialEq)
- Serialization testing
- Memory layout analysis
- Field validation
- JSON schema validation
- Default implementation testing
- Size and alignment checks
- Option wrapping analysis
- Attribute validation

## Installation

```bash
# Clone the repository
git clone https://github.com/deltartificial/tygen.git
cd tygen

# Install dependencies and build
cargo build --release
```

## Usage

```bash
# Basic usage
cargo run -- path/to/types.rs

# Example with provided test files
cargo run -- ./example_type.rs

# With custom output directory
cargo run -- path/to/types.rs -o custom/test/dir

# With verbose output
cargo run -- path/to/types.rs -v
```

## Configuration

The test generation can be configured through the `TestConfig` struct:

```rust
let config = TestConfig {
    check_derives: true,
    check_serialization: true,
    check_size: true,
    check_fields: true,
};
```

## Project Structure

```
src/
├── analyzer/     # Type analysis and parsing
├── generator/    # Test generation logic
├── error/       # Error handling
├── cli.rs       # CLI interface
├── lib.rs       # Library interface
└── main.rs      # Entry point
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 