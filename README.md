# Garnix Fetcher

A multi-mode Rust application to fetch and parse build status information from the Garnix.io API. Available as a CLI tool, HTTP server, or MCP (Model Context Protocol) server.

## Features

- **Multiple Operating Modes**: CLI, HTTP server, and MCP server
- Retrieve a summary of a commit's builds from Garnix.io
- Display a detailed table of all packages with their pass/fail status (using emojis)
- Fetch and display logs for failed builds
- JSON output support for programmatic usage
- MCP integration for AI assistants

## Installation

This project is built using Nix flakes, ensuring a reproducible development environment.

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/garnix-fetcher.git
    cd garnix-fetcher
    ```

2.  **Build the application using Nix:**

    ```bash
    nix build .#default
    ```

    This will build the `garnix-fetcher` executable and place it in `result/bin/garnix-fetcher`.

## Usage

### CLI Mode

The default mode for direct command-line usage:

```bash
./result/bin/garnix-fetcher <JWT_TOKEN> <COMMIT_ID> [--json-output]
```

**Example:**

```bash
./result/bin/garnix-fetcher "your_jwt_token_here" "3402d0072ce57370ed58ce28fe879c32a3501392"
```

**With JSON output:**

```bash
./result/bin/garnix-fetcher "your_jwt_token_here" "3402d0072ce57370ed58ce28fe879c32a3501392" --json-output
```

### HTTP Server Mode

Run as an HTTP server on port 8080:

```bash
./result/bin/garnix-fetcher --server
```

The server provides an endpoint at `http://127.0.0.1:8080/build-status/{commit_id}` that requires the JWT token to be set via the `JWT_TOKEN` environment variable.

### MCP Server Mode

Run as a Model Context Protocol server for AI assistants:

```bash
./result/bin/garnix-fetcher --mcp
```

The MCP server provides a `garnix_build_status` tool that can be used by AI assistants to fetch Garnix build information. The tool accepts:
- `jwt`: JWT token for Garnix.io authentication
- `commit_id`: Git commit ID to fetch build status for

## MCP Integration

### Configure with Claude Desktop

Add to your Claude Desktop configuration file:

```json
{
  "mcpServers": {
    "garnix-fetcher": {
      "command": "/path/to/garnix-fetcher/result/bin/garnix-fetcher",
      "args": ["--mcp"]
    },
    "nixos-packages": {
      "command": "npx",
      "args": ["-y", "@anysphere/mcp-server-nixos@latest"]
    }
  }
}
```

### NixOS MCP Server

For comprehensive NixOS package management and search capabilities, also consider adding the NixOS MCP server from https://mcp-nixos.io/. This provides tools for:
- Searching NixOS packages
- Getting package information
- Finding configuration options
- Package version management

Install via npm:
```bash
npx @anysphere/mcp-server-nixos@latest
```

## Development

Enter the development environment:

```bash
nix develop
```

Build for development:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

## Licensing

This project is dual-licensed under the MIT License and the Apache License 2.0.

- See [LICENSE-MIT](LICENSE-MIT) for details on the MIT License.
- See [LICENSE-APACHE](LICENSE-APACHE) for details on the Apache License 2.0.

## Contributing

Contributions are welcome! Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project. (Note: This file is not yet created, but will be added soon.)
