# Type Tester

A powerful Rust CLI tool for automatically generating comprehensive test suites for Rust types. It analyzes Rust type definitions and generates tests for traits, serialization, memory layout, and more.

## Features

- **Trait Implementation Tests**
  - Verifies `Debug`, `Clone`, `PartialEq` implementations
  - Tests `Serialize`/`Deserialize` trait functionality
  - Validates custom trait implementations

- **Memory Layout Tests**
  - Size and alignment checks
  - Instance size validation
  - `Option<T>` size comparison
  - Memory layout optimization hints

- **Serialization Tests**
  - JSON roundtrip validation
  - Pretty-print format testing
  - JSON schema validation
  - Error handling verification

- **Field Analysis**
  - Field accessibility tests
  - Type safety verification
  - Attribute validation
  - Field-level serialization tests

## Installation & Usage

There are two ways to use this tool:

### Method 1: Run directly with Cargo

```bash
# Clone the repository
git clone https://github.com/yourusername/type-tester.git
cd type-tester

# Run with cargo run (development)
cargo run -- path/to/types.rs

# Run with cargo run (release mode for better performance)
cargo run --release -- path/to/types.rs

# Examples:
cargo run -- path/to/types.rs -o custom/test/dir  # Custom output directory
cargo run -- path/to/types.rs -v                  # Verbose output
```

### Method 2: Install globally (optional)

```bash
# Install globally (this will add the binary to your PATH)
cargo install --path .

# After installation, you can run it directly:
type-tester path/to/types.rs
type-tester path/to/types.rs -o custom/test/dir
type-tester path/to/types.rs -v
```

### Quick Start

1. Create a Rust file with your types (e.g., `types.rs`):

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u32,
}
```

2. Run the tool:
```bash
# Using cargo run
cargo run -- types.rs

# OR if installed globally
type-tester types.rs
```

3. Find your generated tests in the `tests` directory (or your specified output directory)

### Command Line Options

```bash
USAGE:
    type-tester [OPTIONS] <TYPES_FILE>

ARGS:
    <TYPES_FILE>    Path to the Rust types file

OPTIONS:
    -o, --output-dir <DIR>    Output directory for generated tests [default: tests]
    -v, --verbose            Enable verbose output
    -h, --help              Print help information
    -V, --version           Print version information
```

## Project Structure

```
src/
├── analyzer/           # Type analysis
│   ├── mod.rs         # Main analyzer module
│   ├── parser.rs      # Rust file parsing
│   └── visitor.rs     # AST visitor
├── generator/         # Test generation
│   ├── mod.rs        # Test suite coordination
│   ├── derive.rs     # Trait tests
│   ├── serialization.rs # Serialization tests
│   ├── size.rs       # Memory layout tests
│   └── field.rs      # Field tests
├── cli/              # CLI interface
│   └── mod.rs       # Command line handling
├── error/           # Error handling
│   └── mod.rs      # Custom error types
└── main.rs         # Entry point
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

## Extending

### Adding New Test Generators

1. Create a new file in `src/generator/`
2. Implement the `TestGenerator` trait:

```rust
impl TestGenerator for MyNewGenerator {
    fn is_applicable(&self, type_info: &TypeInfo) -> bool {
        // Define when this generator should be used
    }

    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        // Generate test code
    }
}
```

3. Add the generator to `TestSuite::default()`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [syn](https://crates.io/crates/syn) for Rust code parsing
- [quote](https://crates.io/crates/quote) for code generation
- [serde](https://crates.io/crates/serde) for serialization support 