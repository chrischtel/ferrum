# Contributing to Ferrum

Thank you for your interest in contributing to Ferrum! This document provides guidelines for contributing to the project.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find that one already exists. When you create a bug report, please include as many details as possible:

- Use the bug report template
- Include your environment details (OS, Rust version, Zig version)
- Provide a minimal code example that reproduces the issue
- Include the full error output

### Suggesting Features

Feature suggestions are welcome! Please use the feature request template and include:

- A clear description of the problem the feature would solve
- Examples of how the feature would be used
- Consider if this should be an RFC for major language changes

### Contributing Code

1. **Fork the repository** and create your branch from `main`
2. **Install dependencies**:
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install Zig (if not already installed)
   # See https://ziglang.org/download/
   
   # Clone and build
   git clone https://github.com/your-username/ferrum.git
   cd ferrum
   cargo build
   ```

3. **Make your changes**:
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation if needed

4. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   zig fmt --check .
   ```

5. **Create a pull request** using the PR template

## Development Setup

### Prerequisites

- Rust 1.85+ with Cargo
- Zig 0.14.1+
- Git

### Project Structure

```
ferrum/
├── frontend/          # Rust compiler frontend
│   ├── src/lexer/    # Tokenization
│   ├── src/parser/   # AST generation  
│   ├── src/analyzer/ # Semantic analysis
│   └── src/codegen/  # Code generation interface
├── backend/          # Zig backend
│   └── src/          # LLVM code generation
├── core/             # Core runtime
│   └── src/          # Memory management, built-ins
├── tests/            # Integration tests
└── examples/         # Example programs
```

### Building

```bash
# Build everything
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --open
```

## Style Guidelines

### Rust Code

- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` and fix all warnings
- Prefer explicit types in public APIs
- Document public functions and modules
- Use descriptive variable names

### Zig Code

- Follow Zig standard formatting (`zig fmt`)
- Use camelCase for functions, snake_case for variables
- Prefer explicit error handling
- Document complex algorithms

### Git Commits

- Use conventional commit format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- Scopes: `frontend`, `backend`, `core`, `ci`, `docs`
- Examples:
  - `feat(frontend): add support for string literals`
  - `fix(backend): resolve LLVM memory leak`
  - `docs: update installation instructions`

## Testing

### Unit Tests

```bash
# Run Rust tests
cargo test

# Run Zig tests  
zig build test
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration
```

### Adding Tests

- Add unit tests in the same file as the code being tested
- Add integration tests in the `tests/` directory
- Include both positive and negative test cases
- Test error conditions and edge cases

## Documentation

- Update `README.md` for user-facing changes
- Update code comments for implementation changes
- Add examples for new features
- Update the language specification for syntax changes

## Release Process

Releases are handled automatically:

1. **Nightly builds**: Automatic daily builds from `main`
2. **Stable releases**: Tagged releases (e.g., `v0.1.0`)
3. **Pre-releases**: Beta versions for testing

## Getting Help

- **Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Discord**: [Join our community](https://discord.gg/ferrum) (if available)

## Recognition

Contributors will be recognized in:
- Release notes
- `CONTRIBUTORS.md` file
- Project documentation

Thank you for contributing to Ferrum! 🦀⚡
