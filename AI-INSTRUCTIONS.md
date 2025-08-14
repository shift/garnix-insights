# AI Assistant Instructions for Garnix Insights Integration

## Overview
This document provides instructions for AI assistants (GitHub Copilot, Claude, ChatGPT, etc.) on how to effectively use the `garnix-insights` tool for CI/CD build status monitoring and analysis.

## Tool Description
`garnix-insights` is a CI/CD insights tool for Garnix.io that provides:
- Real-time build status checking
- Detailed failure analysis with logs
- Build trend monitoring
- Multi-format output (JSON, human-readable, plain text)
- MCP (Model Context Protocol) server integration

## When to Use Garnix Insights

### Trigger Scenarios
Use `garnix-insights` when users:
- Ask about build status or CI failures
- Mention commit SHAs and want to know if builds passed
- Request build logs or failure details
- Want to debug CI/CD issues
- Ask about Garnix.io specific builds
- Need to verify deployment readiness
- Troubleshoot nix/flake build problems

### Key Phrases That Should Trigger Usage
- "check build status"
- "CI failing" or "build failing"
- "garnix build"
- "commit [SHA] status"
- "build logs"
- "why did this fail"
- "is this ready to deploy"
- "nix build errors"

## Available Commands

### 1. Check Build Status
```bash
garnix-insights fetch --commit-id <COMMIT_SHA>
```
**Use for**: Quick status overview of all builds for a commit

### 2. Get Detailed Build Information with Logs
```bash
garnix-insights logs --commit-id <COMMIT_SHA>
```
**Use for**: Debugging failures, getting error details, analyzing build problems

### 3. Validate API Token
```bash
garnix-insights validate-token
```
**Use for**: Troubleshooting authentication issues

### 4. Start MCP Server (for integrated AI assistants)
```bash
garnix-insights mcp
```
**Use for**: Continuous integration with AI tools

## Integration Patterns

### For GitHub Copilot
Include in your `copilot-instructions.md`:
```markdown
## Garnix CI/CD Integration
- Use `garnix-insights fetch --commit-id <SHA>` to check build status
- Use `garnix-insights logs --commit-id <SHA>` for failure analysis  
- Always check builds before suggesting deployments
- Parse JSON output for programmatic analysis
```

### For Claude Desktop/API
```markdown
## Garnix Build Monitoring
When users mention CI failures or commit status:
1. Run `garnix-insights fetch --commit-id <COMMIT>` first
2. If builds failed, get details with `garnix-insights logs --commit-id <COMMIT>`
3. Analyze error messages and provide specific solutions
4. Suggest fixes based on Nix/Garnix error patterns
```

### For ChatGPT/Custom GPTs
```markdown
## Build Status Analysis Protocol
1. **Status Check**: Always verify current build status first
2. **Failure Analysis**: Get detailed logs for failed builds
3. **Solution Guidance**: Provide actionable fixes based on error types
4. **Deployment Readiness**: Confirm all builds pass before deployment advice
```

## Output Format Examples

### Successful Build Status
```
✅ Build Status for commit abc123def

Summary: 3/3 builds passed
├─ ✅ x86_64-linux.default (2m 34s)
├─ ✅ x86_64-linux.checks (1m 45s) 
└─ ✅ aarch64-darwin.default (3m 12s)

🎉 All builds successful - ready for deployment!
```

### Failed Build with Analysis
```
❌ Build Status for commit abc123def

Summary: 1/3 builds failed
├─ ❌ x86_64-linux.default (failed after 1m 23s)
├─ ✅ x86_64-linux.checks (2m 45s)
└─ ✅ aarch64-darwin.default (3m 12s)

🔍 Failure Analysis:
Error: derivation '/nix/store/...' failed with exit code 2
Cause: missing dependency 'openssl-dev'
Fix: Add openssl to buildInputs in flake.nix
```

## Error Pattern Recognition

### Common Garnix/Nix Error Types
1. **Missing Dependencies**: Look for "command not found", "No such file"
2. **Build Tool Issues**: Check for "cargo", "npm", "rustc" errors  
3. **Nix Store Problems**: Watch for "/nix/store/" path issues
4. **Environment Issues**: Look for missing env vars, PATH problems
5. **Resource Limits**: Memory/disk space exhaustion

### Suggested Responses by Error Type
- **Dependency Missing**: "Add [package] to buildInputs in your flake.nix"
- **Compilation Error**: "Check your source code for syntax errors"
- **Test Failure**: "Review failing tests and fix the underlying issues"
- **Timeout**: "Consider optimizing build steps or increasing timeouts"

## Best Practices for AI Assistants

### 1. Always Check Status First
Before providing build advice, always run a status check to understand the current state.

### 2. Provide Context-Aware Solutions
Parse error messages and provide specific, actionable fixes rather than generic advice.

### 3. Use Appropriate Format
- Use `--format json` for programmatic analysis
- Use `--format human` for user-facing reports
- Use `--format plain` for logs and scripting

### 4. Handle Authentication Gracefully
If authentication fails, guide users to set up their JWT token properly.

### 5. Respect Rate Limits
Don't make excessive API calls - cache results when appropriate.

## Environment Setup

### Required Environment Variables
```bash
export GARNIX_JWT_TOKEN="your-jwt-token-here"
```

### Optional Configuration
```bash
export GARNIX_API_URL="https://api.garnix.io"  # Custom endpoint
export RUST_LOG="info"  # Logging level
```

## MCP Integration

### For VS Code Copilot
Add to your MCP configuration:
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

### Available MCP Tools
- `garnix_build_status`: Get build status for commits
- `garnix_build_details`: Get detailed build information and logs
- `garnix_health_check`: Verify API connectivity

## Example AI Conversations

### User: "Why is my build failing?"
**AI Response**:
```
Let me check your build status first. I'll need your commit SHA.

[After getting commit]: 
*runs: garnix-insights logs --commit-id abc123def*

I can see your build failed due to a missing OpenSSL dependency. Here's how to fix it:

1. Add `openssl` to your `buildInputs` in `flake.nix`
2. Add `pkg-config` to `nativeBuildInputs` 
3. Rebuild with `nix build`

The specific error shows: "ld: library not found for -lssl"
```

### User: "Is commit abc123 ready to deploy?"
**AI Response**:
```
*runs: garnix-insights fetch --commit-id abc123*

✅ Yes! Commit abc123 is ready to deploy:
- All 3 builds passed successfully
- Tests completed without issues  
- Build time was reasonable (under 5 minutes total)
- No security vulnerabilities detected

You're good to proceed with deployment.
```

## Troubleshooting Guide

### Common Issues
1. **Authentication Error**: Check GARNIX_JWT_TOKEN is set correctly
2. **Network Timeout**: Verify internet connection and Garnix.io status
3. **Command Not Found**: Ensure garnix-insights is installed via cargo or nix
4. **Invalid Commit**: Verify the commit SHA exists in your repository

### Debug Mode
Enable verbose logging:
```bash
export RUST_LOG=garnix_insights=debug
garnix-insights fetch --commit-id <SHA> --verbose
```

## Updates and Maintenance

- **Version**: Check `garnix-insights --version` for current version
- **Updates**: Update via `cargo install garnix-insights` or `nix profile upgrade`
- **Documentation**: Visit https://github.com/shift/garnix-insights
- **Issues**: Report bugs at https://github.com/shift/garnix-insights/issues

---

**Remember**: Always verify build status before advising on deployments, and provide specific, actionable solutions based on actual error messages from the build logs.
