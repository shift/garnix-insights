# Contributing to Garnix Insights

Thank you for considering contributing to Garnix Insights! This project helps developers gain insights into their Garnix.io CI/CD builds through multiple interfaces (CLI, HTTP server, and MCP server).

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Project Structure](#project-structure)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Nix package manager with flakes enabled
- Git
- A Garnix.io account (for testing)

### Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/shift/garnix-insights.git
   cd garnix-insights
   ```

2. **Enter the development environment:**
   ```bash
   nix develop
   ```

   This provides all necessary tools including Rust toolchain, formatters, linters, and testing utilities.

3. **Verify the setup:**
   ```bash
   nix flake check  # Run all checks
   cargo build      # Build the project
   cargo test       # Run tests
   ```
## Making Changes

### Development Workflow

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes:**
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes:**
   ```bash
   cargo test           # Run unit tests
   cargo clippy         # Run linter
   cargo fmt --check    # Check formatting
   nix flake check      # Run all checks
   ```

4. **Test crates.io integration (if applicable):**
   ```bash
   test-cratesio        # Test published version
   check-cratesio-api   # Verify API availability
   ```

### Commit Message Guidelines

We follow [Conventional Commits](https://conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Test additions/modifications
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes

**Examples:**
```
feat: add JSON export functionality to CLI
fix: handle network timeouts in Garnix API calls
docs: update MCP server configuration examples
test: add integration tests for HTTP server endpoints
```

**Important:** Use only ASCII characters in commit messages (no emojis) due to release automation requirements.
## Testing

### Test Categories

1. **Unit Tests:** Test individual functions and modules
   ```bash
   cargo test --lib
   ```

2. **Integration Tests:** Test HTTP server and CLI functionality
   ```bash
   cargo test --test '*'
   ```

3. **Documentation Tests:** Test code examples in documentation
   ```bash
   cargo test --doc
   ```

4. **End-to-End Tests:** Test against real Garnix API (requires JWT)
   ```bash
   export GARNIX_JWT_TOKEN="your-token"
   cargo test --test integration
   ```

### Code Coverage

Generate coverage reports:
```bash
cargo tarpaulin --out Html --output-dir coverage
```

View coverage: Open `coverage/tarpaulin-report.html` in your browser.

### Performance Testing

For performance-critical changes:
```bash
cargo bench  # Run benchmarks (if implemented)
```

## Code Style

### Rust Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` with default settings: `cargo fmt`
- Address all `clippy` warnings: `cargo clippy`
- Add documentation for public APIs
- Use `anyhow::Error` for error handling
- Prefer `tokio` for async operations

### Documentation

- Document all public functions, structs, and modules
- Include examples in documentation when helpful
- Update README.md for user-facing changes
- Update GUIDE.md for API/server changes
- Update MCP-CONFIGURATION.md for MCP-related changes

### Nix Guidelines

- Test changes with `nix flake check`
- Ensure reproducible builds
- Update flake inputs responsibly
- Document any new development tools
## Project Structure

```
garnix-insights/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library root
│   ├── client.rs        # Garnix API client
│   ├── server.rs        # HTTP server implementation
│   ├── mcp.rs           # MCP server implementation
│   ├── cli.rs           # CLI argument parsing and logic
│   ├── types.rs         # Data structures and types
│   └── error.rs         # Error types and handling
├── tests/               # Integration tests
├── docs/                # Additional documentation
├── .github/             # GitHub workflows and templates
├── flake.nix           # Nix development environment
├── Cargo.toml          # Rust project configuration
├── README.md           # Project overview
├── GUIDE.md            # API and server guide
└── MCP-CONFIGURATION.md # MCP server setup
```

## Submitting Changes

### Pull Request Process

1. **Update documentation** for any user-facing changes
2. **Add tests** that cover your changes
3. **Ensure all checks pass:**
   ```bash
   nix flake check
   ```
4. **Update CHANGELOG** if your changes are user-facing
5. **Submit the pull request** with:
   - Clear title following conventional commit format
   - Description of what changed and why
   - Links to related issues
   - Screenshots/examples if applicable

### Review Process

- At least one maintainer review is required
- All CI checks must pass
- Code coverage should not decrease significantly
- Documentation must be updated for user-facing changes

### Release Process

Releases are automated via [release-please](https://github.com/googleapis/release-please):
- Semantic version bumps based on conventional commits
- Automatic changelog generation
- Automatic crates.io publishing
- GitHub release creation

## Getting Help

- **Questions:** Open a [Discussion](https://github.com/shift/garnix-insights/discussions)
- **Bug Reports:** Open an [Issue](https://github.com/shift/garnix-insights/issues)
- **Feature Requests:** Open an [Issue](https://github.com/shift/garnix-insights/issues)
- **Security Issues:** Email shift@someone.section.me directly

## Recognition

Contributors are automatically added to the repository's contributor list. Significant contributions may be recognized in release notes and project documentation.

Thank you for contributing to Garnix Insights! 🚀
