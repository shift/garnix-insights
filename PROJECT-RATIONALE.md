# Project Rationale: Why Garnix Insights Exists

## The Problem

### Manual Copy-Paste Hell

Before Garnix Insights, getting build status information to AI agents or automation tools was a nightmare of manual processes:

- **Copy-paste dependency**: Need to check if a commit is ready? Copy the commit SHA, navigate to Garnix.io, paste it in, wait for results, then manually copy-paste the status back to wherever you needed it.
- **Context switching**: Constantly switching between terminals, browsers, and chat interfaces just to get basic build information.
- **No programmatic access**: Garnix.io has an API, but no tooling existed to make it easily accessible to agents, scripts, or automation workflows.
- **Manual error propagation**: Human copy-paste introduces errors, missed details, and inconsistent reporting.

This wasn't just inconvenient—it was **genuinely shitty** and a massive productivity drain.

### GitHub Actions Blind Spot

GitHub Actions and CI systems have a fundamental limitation when it comes to Garnix.io integration:

- **No native Garnix access**: GitHub Actions can't directly query Garnix.io build status or logs
- **Limited log visibility**: Even when CI systems can access logs, they often don't surface the meaningful details that developers need
- **Status reporting gap**: No way for GitHub Actions to automatically report "this commit passed all Nix builds on Garnix" without manual intervention
- **Deployment decision paralysis**: Can't programmatically determine if a commit is ready for deployment based on Garnix build results

## The Solution: Garnix Insights

### Programmatic Access to Build Intelligence

Garnix Insights bridges the gap by providing **three complementary interfaces** for accessing Garnix.io build data:

1. **CLI Tool**: Direct command-line access for scripts and manual queries
2. **HTTP Server**: RESTful API for web applications and services
3. **MCP Server**: Model Context Protocol integration for AI agents

### Real Use Cases This Enables

#### AI Agent Integration
```bash
# Agent can now directly query build status
garnix-insights fetch --commit-id abc123... --format json
```

Before: *"Can you check if commit abc123 passed its builds? Let me copy-paste the Garnix link..."*

After: *Agent automatically queries build status and responds with complete analysis*

#### Automated Deployment Decisions
```bash
# Script can programmatically decide if deployment is safe
if garnix-insights fetch --commit-id $COMMIT --format json | jq '.success_rate == 100'; then
  echo "✅ All builds passed - safe to deploy"
else
  echo "❌ Builds failed - blocking deployment"
fi
```

#### GitHub Actions Integration
```yaml
- name: Check Garnix Build Status
  run: |
    garnix-insights fetch --commit-id ${{ github.sha }} --format json > build-status.json
    cat build-status.json | jq '.summary'
```

#### Development Workflow Enhancement
```bash
# Quick status check before creating a PR
garnix-insights fetch --commit-id HEAD --format human
```

### What This Actually Solves

#### For Developers
- **Instant build status**: No more navigating to web interfaces
- **Rich contextual information**: Get build logs, failure details, and system-specific results
- **Scriptable workflows**: Integrate build status into any automation
- **Multiple output formats**: Human-readable, JSON for scripts, plain text for logs

#### For AI Agents
- **Direct API access**: Agents can query build status without human intervention
- **Structured data**: JSON responses enable agents to make intelligent decisions
- **MCP protocol support**: Native integration with agent frameworks
- **Build log access**: Agents can analyse failure details and suggest fixes

#### For CI/CD Pipelines
- **External build validation**: Verify Nix builds as part of broader CI workflows
- **Deployment gating**: Automatically block deployments for failed builds
- **Status reporting**: Update GitHub commit status based on Garnix results
- **Log aggregation**: Pull build logs into centralised logging systems

## Technical Architecture

### Why Three Interfaces?

**CLI**: Perfect for scripts, manual queries, and simple automation
```bash
garnix-insights fetch --commit-id abc123 --format human
```

**HTTP Server**: Enables web applications and services to query build status
```http
GET /api/v1/build-status/abc123?token=jwt-token
```

**MCP Server**: Native AI agent integration via Model Context Protocol
```python
# Agent framework automatically has access to Garnix build data
agent.query_build_status("abc123")
```

### Built for Reliability

- **Comprehensive error handling**: Clear error messages for authentication, network, and API issues
- **Multiple output formats**: Choose the right format for each use case
- **Robust authentication**: JWT token support with environment variable fallback
- **Extensive logging**: Debug mode provides detailed request/response information

## Impact and Benefits

### Before Garnix Insights
- Manual copy-paste workflows ❌
- No programmatic access to build data ❌
- GitHub Actions can't check Garnix status ❌
- Agents can't access build information ❌
- Deployment decisions require manual verification ❌

### After Garnix Insights
- Fully automated build status queries ✅
- Rich programmatic API access ✅
- GitHub Actions integration possible ✅
- Native AI agent support ✅
- Automated deployment gating ✅

### Real-World Workflow Transformation

**Old workflow:**
1. Want to check if commit is ready
2. Copy commit SHA from terminal
3. Open Garnix.io in browser  
4. Paste commit SHA
5. Wait for page to load
6. Manually read build status
7. Copy-paste status to agent/chat
8. Make deployment decision based on manual assessment

**New workflow:**
```bash
garnix-insights fetch --commit-id $(git rev-parse HEAD) --format human
```
**Done.** Agent gets structured data, automation makes decisions, deployment proceeds safely.

## Why This Matters

This isn't just about convenience—it's about **removing friction from the development workflow**. When tools integrate seamlessly, developers can focus on building instead of wrestling with manual processes.

Garnix Insights transforms Garnix.io from a web-only service into a **first-class API citizen** that can participate in modern DevOps workflows, AI agent interactions, and automated decision-making processes.

The result? **Less manual work, better decisions, and more reliable deployments.**
