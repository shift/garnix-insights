# Garnix Insights

A professional multi-mode Rust application for comprehensive CI/CD insights from Garnix.io. Available as a CLI tool, HTTP server, and MCP (Model Context Protocol) server for AI assistant integration.

## Features

- **Multiple Operating Modes**: CLI, HTTP server, and MCP server
- **Comprehensive Build Analysis**: Real-time status, detailed logs, and failure analysis
- **Professional Output Formats**: Human-readable, JSON, and plain text
- **AI Assistant Integration**: Full MCP support for GitHub Copilot, Claude, and other AI tools
- **Robust Error Handling**: Graceful handling of API failures and network issues
- **Security Focused**: AGPL-3.0 licensed with comprehensive dependency auditing

## Installation

### From Cargo (Recommended)

```bash
cargo install garnix-insights
```

### From Nix Flake

```bash
# Install to profile
nix profile install github:shift/garnix-insights

# Or run directly
nix run github:shift/garnix-insights -- --help
```

### From Source

```bash
git clone https://github.com/shift/garnix-insights.git
cd garnix-insights
nix build
./result/bin/garnix-insights --help
```

## Usage

### CLI Mode

Set your JWT token as an environment variable:

```bash
export GARNIX_JWT_TOKEN="your_jwt_token_here"
```

#### Getting Your JWT Token

**Currently, the JWT token is only available through browser developer tools:**

1. **Login to Garnix.io**: Go to https://garnix.io and log in to your account
2. **Open Developer Tools**: Press F12 or right-click → "Inspect Element"
3. **Go to Network Tab**: Click on the "Network" tab in developer tools
4. **Make an API Request**: Navigate around the Garnix.io site to trigger API calls
5. **Find API Request**: Look for requests to `api.garnix.io` in the network list
6. **Copy JWT from Cookie**: 
   - Click on any API request
   - Go to "Request Headers" section
   - Find the `Cookie` header
   - Copy the JWT value from the cookie

**Note**: This method is not ideal - I've reached out to the Garnix.io maintainers about providing a better solution for API access. Thanks to @lassulus for helping connect me with the team!

#### Commands

Check build status for a commit:

```bash
garnix-insights fetch --commit-id 3402d0072ce57370ed58ce28fe879c32a3501392
```

Get detailed build logs:

```bash
garnix-insights logs --commit-id 3402d0072ce57370ed58ce28fe879c32a3501392
```

**Output Formats:**

```bash
garnix-insights fetch --commit-id <COMMIT> --format json    # JSON output
garnix-insights fetch --commit-id <COMMIT> --format human   # Human-readable (default)
garnix-insights fetch --commit-id <COMMIT> --format plain   # Plain text
```

### HTTP Server Mode

```bash
export GARNIX_JWT_TOKEN="your_jwt_token_here"
garnix-insights server
```

Access the API at `http://127.0.0.1:8080/build-status/{commit_id}`

### MCP Server Mode

For AI assistant integration:

```bash
export GARNIX_JWT_TOKEN="your_jwt_token_here"
garnix-insights mcp
```

## AI Assistant Integration

### GitHub Copilot

Add this to your `.github/copilot-instructions.md`:

```markdown
## Garnix CI/CD Integration
- Use `garnix-insights fetch --commit-id <SHA>` to check build status
- Use `garnix-insights logs --commit-id <SHA>` for failure analysis  
- Always check builds before suggesting deployments
- Parse JSON output for programmatic analysis
```

### Claude Desktop

Add to your configuration file:

```json
{
  "mcpServers": {
    "garnix-insights": {
      "command": "garnix-insights",
      "args": ["mcp"],
      "env": {
        "GARNIX_JWT_TOKEN": "your-token-here"
      }
    }
  }
}
```

For comprehensive setup instructions, see [MCP-CONFIGURATION.md](MCP-CONFIGURATION.md).

## Development

### Prerequisites

- Nix with flakes enabled
- Rust toolchain (via Nix shell)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/shift/garnix-insights.git
cd garnix-insights

# Enter development environment
nix develop

# Run tests
cargo test

# Build for development  
cargo build

# Run comprehensive CI checks
nix flake check
```

### Publishing

See [PUBLISHING.md](PUBLISHING.md) for detailed instructions on publishing to crates.io.

## License

This project is licensed under **AGPL-3.0** with commercial approval requirements.

- **Open Source Use**: Free for open source projects under AGPL-3.0 terms
- **Commercial Use**: Requires separate commercial license approval
- See [LICENSE-AGPL-3.0.md](LICENSE-AGPL-3.0.md) for full license terms
- See [LICENSE-ANALYSIS.md](LICENSE-ANALYSIS.md) for dependency analysis

## Support

- **Issues**: https://github.com/shift/garnix-insights/issues
- **Documentation**: Complete guides in [AI-INSTRUCTIONS.md](AI-INSTRUCTIONS.md) and [MCP-CONFIGURATION.md](MCP-CONFIGURATION.md)
- **Contributing**: Follow standard Rust/Nix contribution practices
