# Why I Built Garnix Insights

## The Problem

### Copy-Paste Hell is Shit

Getting build status to AI agents was doing my head in:

- **Constant switching**: Need to check if a commit's ready? Copy the SHA, tab to browser, paste it, wait, then manually tell the agent what happened. Bollocks to that.
- **Context switching nightmare**: Terminal -> browser -> chat -> repeat. My ADHD brain was having none of it.
- **No API access**: Garnix has an API but no decent tooling. Had to build something that actually works.
- **Manual errors**: Copy-paste introduces mistakes and I kept missing details.

This wasn't just annoying—it was **properly shit** and killing productivity.

### GitHub Actions Can't See Garnix

GitHub Actions has a blind spot with Garnix:

- **No direct access**: GitHub Actions can't query Garnix build status
- **Crap log visibility**: Even when CI can access logs, they don't show useful stuff
- **No status updates**: Can't automatically tell GitHub "this commit passed Nix builds"  
- **Can't decide deployments**: No way to programmatically check if a commit's ready

So I built something that fixes this.

### The Garnix Problem  

Garnix.io gives you sweet CI builds for Nix projects - proper fast, distributed, cached builds that actually work. But there's one issue: the only way to check your build status is through their web interface. That's fine for humans, but useless for:

- **Scripts and automation**
- **AI agents** (MCP servers, Claude Desktop, etc.)  
- **CI/CD systems** that need to check external build status
- **Command line workflows**

You end up with this stupid workflow:
1. Push your code
2. Open browser 
3. Navigate to Garnix
4. Find your commit
5. Check if it built
6. Maybe copy some logs if it failed

That's bollocks when you're trying to automate things or when an AI agent needs to check build status.

## What I Built

Garnix Insights is a dead simple CLI tool and API server that bridges this gap. Give it a JWT token and a commit SHA, and it'll fetch the build status, logs, and system details from Garnix.io.

**CLI Mode:**
```bash
garnix-insights fetch --commit-id abc123def --token your_jwt --format human
```

**API Server Mode:**
```bash
garnix-insights serve --port 3000
curl "http://localhost:3000/status/abc123def" -H "Authorization: Bearer your_jwt"
```

**MCP Server Mode:** (for AI agents)
```bash
garnix-insights mcp
```

It handles all the Garnix API bollocks for you - authentication, pagination, error handling, different response formats. Just get the data you need.

### Three Ways to Get Build Data

Built three interfaces because different situations need different approaches:

1. **CLI**: For quick checks and scripts
2. **HTTP Server**: For web apps and services  
3. **MCP Server**: For AI agents (fully working)

### What You Can Actually Do

#### Agent Integration
```bash
# Agent queries directly instead of you copying/pasting
garnix-insights fetch --commit-id abc123 --format json
```

Before: "Can you check if this commit passed? Let me go copy the link..."
After: Agent just gets the data automatically.

#### Deployment Automation  
```bash
# Script decides if it's safe to deploy
if garnix-insights fetch --commit-id $COMMIT --format json | jq '.success_rate == 100'; then
  echo "All builds passed - deploying"
else  
  echo "Builds failed - not deploying"
fi
```

#### GitHub Actions
```yaml
- name: Check Garnix Build Status
  run: |
    garnix-insights fetch --commit-id ${{ github.sha }} --format json > build-status.json
    cat build-status.json | jq '.summary'
```

#### Quick PR Checks
```bash
# Before creating a PR
garnix-insights fetch --commit-id HEAD --format human
```

### What This Actually Fixes

#### For Developers
- **Quick build status**: No more browser tabs
- **Build details**: Get logs, failures, system info
- **Works in scripts**: Integrate into any automation
- **Multiple formats**: Human readable, JSON, plain text

#### For AI Agents  
- **Direct access**: Agents query without human help
- **Structured data**: JSON responses for making decisions
- **MCP support**: Works with agent frameworks  
- **Build logs**: Agents can analyze failures

#### For CI/CD
- **External validation**: Check Nix builds in other CI
- **Deployment gates**: Block bad deployments automatically
- **Status updates**: Tell GitHub about build results
- **Log collection**: Pull logs into other systems

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

- **Good error handling**: Clear error messages for authentication, network, and API issues
- **Multiple output formats**: Choose the right format for each use case
- **Solid authentication**: JWT token support with environment variable fallback
- **Proper logging**: Debug mode provides detailed request/response information

## Impact and Benefits

### Before Garnix Insights
- Manual copy-paste workflows 
- No programmatic access to build data 
- GitHub Actions can't check Garnix status 
- Agents can't access build information 
- Deployment decisions require manual verification 

### After Garnix Insights
- Fully automated build status queries 
- Rich programmatic API access 
- GitHub Actions integration possible 
- Native AI agent support 
- Automated deployment gating

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

Garnix Insights transforms Garnix.io from a web-only service into a **proper API** that can participate in modern DevOps workflows, AI agent interactions, and automated decision-making processes.

The result? **Less manual work, better decisions, and more reliable deployments.**
