# MCP Server Configuration for Garnix Insights

This document describes how to configure Garnix Insights as an MCP (Model Context Protocol) server for use with AI assistants like GitHub Copilot.

## Installation

First, install Garnix Insights from crates.io:

```bash
cargo install garnix-insights
```

Or if you prefer using Nix:

```bash
nix profile install github:shift/garnix-insights
```

## Configuration for VS Code Copilot

Add this configuration to your VS Code settings or MCP configuration:

```json
{
  "mcpServers": {
    "garnix-insights": {
      "command": "garnix-insights",
      "args": ["mcp"],
      "env": {
        "GARNIX_JWT_TOKEN": "${env:GARNIX_JWT_TOKEN}"
      }
    }
  }
}
```

### Alternative: Using Nix (Development)

For development or if you prefer Nix:

```json
{
  "mcpServers": {
    "garnix-insights": {
      "command": "nix",
      "args": [
        "run",
        "github:shift/garnix-insights#mcp"
      ],
      "env": {
        "GARNIX_JWT_TOKEN": "${env:GARNIX_JWT_TOKEN}"
      }
    }
  }
}
```

## Local Development Configuration

For local development, use:

```json
{
  "mcpServers": {
    "garnix-insights-local": {
      "command": "cargo",
      "args": ["run", "--", "mcp"],
      "cwd": "/path/to/garnix-insights",
      "env": {
        "GARNIX_JWT_TOKEN": "${env:GARNIX_JWT_TOKEN}"
      }
    }
  }
}
```

## Environment Variables

Set these environment variables:

```bash
# Required: Your Garnix.io JWT token
export GARNIX_JWT_TOKEN="your-jwt-token-here"

# Optional: Custom API endpoint (defaults to api.garnix.io)
export GARNIX_API_URL="https://api.garnix.io"

# Optional: Enable debug logging
export RUST_LOG=debug
```

### Getting Your JWT Token

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

## Available MCP Tools

Once configured, the following tools will be available in your AI assistant:

### `garnix_build_status`
Get build status for a specific commit.
```json
{
  "name": "garnix_build_status",
  "description": "Fetch build status from Garnix.io for a specific commit",
  "inputSchema": {
    "type": "object",
    "properties": {
      "commit_id": {
        "type": "string",
        "description": "The Git commit SHA to check"
      },
      "format": {
        "type": "string",
        "enum": ["json", "markdown"],
        "description": "Output format (default: markdown)"
      }
    },
    "required": ["commit_id"]
  }
}
```

### `garnix_build_details`
Get detailed information about individual builds.
```json
{
  "name": "garnix_build_details", 
  "description": "Get detailed build information including logs and artifacts",
  "inputSchema": {
    "type": "object",
    "properties": {
      "commit_id": {
        "type": "string",
        "description": "The Git commit SHA"
      },
      "build_id": {
        "type": "string",
        "description": "Optional: specific build ID to focus on"
      }
    },
    "required": ["commit_id"]
  }
}
```

### `garnix_health_check`
Check if the Garnix API is accessible.
```json
{
  "name": "garnix_health_check",
  "description": "Verify connectivity to Garnix.io API",
  "inputSchema": {
    "type": "object",
    "properties": {}
  }
}
```

## Usage Examples

### In GitHub Copilot Chat

```
@copilot Check the build status for commit abc123def

@copilot Show me detailed build information for the latest commit

@copilot Is Garnix API accessible?
```

### In Claude Desktop

Configure in `claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "garnix-insights": {
      "command": "garnix-insights",
      "args": ["mcp"],
      "env": {
        "GARNIX_JWT_TOKEN": "your-token"
      }
    }
  }
}
```

## Security Considerations

- **JWT Token Security**: Never commit JWT tokens to version control
- **Environment Variables**: Use secure environment variable management
- **Network Access**: MCP server requires internet access to reach Garnix.io
- **API Rate Limits**: Respect Garnix.io API rate limits

## Troubleshooting

### Common Issues

1. **"Command not found"**: Ensure `garnix-insights` is installed via `cargo install garnix-insights`
2. **"Authentication failed"**: Check GARNIX_JWT_TOKEN environment variable
3. **"Network timeout"**: Verify internet connectivity and Garnix.io status

### Debug Mode

Enable verbose logging:
```bash
export RUST_LOG=garnix_insights=debug,info
```

### Testing MCP Connection

```bash
# Test MCP server directly
garnix-insights mcp

# Or with Nix:
nix run github:shift/garnix-insights#mcp

# Test with sample data
echo '{"method": "tools/list"}' | garnix-insights mcp
```

## Contributing

To extend MCP functionality:

1. Modify `src/mcp.rs` to add new tools
2. Update this configuration documentation
3. Add tests in `src/mcp.rs`
4. Update flake.nix if needed

## Support

- **Issues**: https://github.com/shift/garnix-insights/issues
- **Documentation**: https://shift.github.io/garnix-insights/
- **License**: AGPL-3.0 (see LICENSE-AGPL-3.0.md)
