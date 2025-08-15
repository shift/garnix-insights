# GitHub Secrets Setup for Automated Publishing

This guide shows you how to securely add your Cargo API key and other secrets to GitHub for automated publishing.

## Required GitHub Secrets

You need to add these secrets to your GitHub repository:

### 1. CARGO_REGISTRY_TOKEN (Required)
- **Value**: `<your-cargo-api-token>`
- **Purpose**: Allows GitHub Actions to publish to crates.io
- **Used in**: `publish.yml` workflow

### 2. CACHIX_AUTH_TOKEN (Optional but Recommended)  
- **Value**: Get from https://app.cachix.org/personal-auth-tokens
- **Purpose**: Speeds up Nix builds by using binary cache
- **Used in**: Both `ci.yml` and `publish.yml` workflows

## How to Add GitHub Secrets

### Step 1: Navigate to Repository Settings
1. Go to your GitHub repository: `https://github.com/shift/garnix-insights`
2. Click on **Settings** tab (top right)
3. In the left sidebar, click **Secrets and variables** > **Actions**

### Step 2: Add CARGO_REGISTRY_TOKEN
1. Click **New repository secret**
2. **Name**: `CARGO_REGISTRY_TOKEN`
3. **Secret**: `<your-cargo-api-token>`
4. Click **Add secret**

### Step 3: Add CACHIX_AUTH_TOKEN (Optional)
1. First, get your token from https://app.cachix.org/personal-auth-tokens
2. Click **New repository secret**  
3. **Name**: `CACHIX_AUTH_TOKEN`
4. **Secret**: `<your-cachix-token>`
5. Click **Add secret**

## Automated Publishing Workflow

Once secrets are set up, publishing happens automatically:

### Trigger Publishing
```bash
# Bump version and create tag
./scripts/version-bump.sh 0.1.1

# Push to GitHub (this triggers publishing)
git push origin main --tags
```

### What Happens Automatically
1. **Tag Detection**: GitHub detects the new `v0.1.1` tag
2. **CI Pipeline**: Runs full test suite and flake checks
3. **Package Validation**: Verifies package contents and metadata
4. **Dry Run**: Tests publishing without actually publishing
5. **Publish**: Publishes to crates.io using your API token
6. **GitHub Release**: Creates a GitHub release with install instructions

## Manual Publishing (Alternative)

If you prefer manual control:

### Local Publishing
```bash
# Set your API token locally (one-time setup)
cargo login YOUR_CRATES_IO_API_TOKEN

# Use the automated script
./scripts/publish.sh
```

### Environment Variable Method
```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export CARGO_REGISTRY_TOKEN="YOUR_CRATES_IO_API_TOKEN"

# Then publish without login
nix develop -c -- cargo publish
```

## Security Best Practices

### ✅ Do This
- Store API tokens in GitHub Secrets (encrypted)
- Use environment variables for local development
- Rotate API tokens regularly
- Use separate tokens for different projects if possible

### ❌ Don't Do This
- Never commit API tokens to git
- Don't store tokens in plain text files
- Don't share tokens in chat/email
- Don't use production tokens for testing

## Workflow Files Overview

### `.github/workflows/publish.yml`
- **Triggers**: On version tags (`v*`) or manual dispatch
- **Purpose**: Publishes to crates.io and creates GitHub releases
- **Secrets Used**: `CARGO_REGISTRY_TOKEN`, `CACHIX_AUTH_TOKEN`

### `.github/workflows/ci.yml`  
- **Triggers**: On pushes to main/develop, pull requests
- **Purpose**: Runs tests, checks, and security audits
- **Secrets Used**: `CACHIX_AUTH_TOKEN` (optional)

## Troubleshooting

### Common Issues

1. **"authentication failed"**
   - Check that `CARGO_REGISTRY_TOKEN` secret is set correctly
   - Verify the token hasn't expired
   - Make sure you have publish permissions for the crate name

2. **"workflow not triggering"**
   - Ensure tag format is correct (`v1.0.0`, not just `1.0.0`)
   - Check that workflows are enabled in repository settings
   - Verify you pushed the tags: `git push --tags`

3. **"cachix authentication failed"**  
   - This is optional - workflow will still work without it
   - Get a new token from https://app.cachix.org/personal-auth-tokens
   - Update the `CACHIX_AUTH_TOKEN` secret

### Debug Commands
```bash
# Check if secrets are available (locally)
echo $CARGO_REGISTRY_TOKEN

# Verify token works
cargo login $CARGO_REGISTRY_TOKEN
cargo owner --list garnix-insights

# Test workflow locally (after installing act)
act -s CARGO_REGISTRY_TOKEN=YOUR_CRATES_IO_API_TOKEN
```

## Next Steps

1. **Set up the repository**: Create `https://github.com/shift/garnix-insights`
2. **Add the secrets**: Follow the steps above to add `CARGO_REGISTRY_TOKEN`
3. **Push your code**: `git push origin main`
4. **Test the workflow**: Create a test tag and see if it publishes
5. **Monitor**: Check GitHub Actions tab for workflow results

---

**Your API token is now secure and ready for automated publishing!** 🔒
