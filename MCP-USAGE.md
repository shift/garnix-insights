# MCP Server Usage Examples

## Basic Setup

1. **Install Garnix Insights:**
```bash
cargo install garnix-insights
```

2. **Configure Claude Desktop** (add to claude_desktop_config.json):
```json
{
  "mcpServers": {
    "garnix-insights": {
      "command": "garnix-insights",
      "args": ["mcp"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

3. **Start Claude Desktop** - Garnix tools will be available automatically

## Available Tools

### 1. get_build_status
Get comprehensive build status for any commit:
```
Check the build status for commit abc123def with my JWT token
```

### 2. check_commit_ready  
Quick deployment readiness check:
```
Is commit abc123def ready for deployment?
```

### 3. get_build_logs
Get detailed build information:
```
Show me the build details for commit abc123def
```

## Example Conversation

**You:** "Check if commit a1b2c3d4 is ready for deployment using token xyz..."

**Claude (via MCP):** 
```
[OK] Commit a1b2c3d4 is ready for deployment! All 3 builds passed.

Build Status Details:
- nix-build-x86_64-linux: Success (2m 34s)
- nix-build-aarch64-darwin: Success (3m 12s) 
- nix-flake-check: Success (45s)

All systems green - safe to deploy! 🚀
```

## Direct Command Line Usage

You can also use the MCP server directly via stdio:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"check_commit_ready","arguments":{"commit_id":"abc123","token":"your-jwt"}}}' | garnix-insights mcp
```

## Integration Benefits

✅ **No more copy-paste** between terminal and browser  
✅ **Instant build status** in your AI conversations  
✅ **Smart deployment decisions** based on actual build results  
✅ **Automated CI/CD integration** with AI agents  
✅ **Rich build details** with error diagnostics  

The MCP server transforms Garnix from a web-only service into a **proper API** that AI agents can use directly.
