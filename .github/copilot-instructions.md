# Copilot Instructions for Garnix Insights

## Project Overview
- This is a Rust application and API server for fetching and exposing Garnix.io CI build status for specific commits.
- The codebase is designed for use within a NixOS environment, leveraging Nix flakes for reproducible builds and development shells.
- **Project was renamed from "garnix-fetcher" to "garnix-insights"** - all references should use the new name.
- **Licensed under AGPL-3.0** (changed from dual MIT/Apache-2.0 licensing).

## Architecture & Key Files
- Main logic is in `src/main.rs`.
- API server requirements and conventions are described in `GUIDE.md`.
- Command-line usage and build instructions are in `README.md`.
- Nix build and environment setup is managed by `flake.nix`.
- **Release automation** is handled by `.github/workflows/release-please.yml` using Google's release-please.

## Developer Workflow
- **Always use Nix shells for Rust commands:**
  - Build, run, and test with: `nix develop -c -- cargo <cmd>` (e.g., `cargo build`, `cargo run`, `cargo test`).
- **After changing Rust dependencies:**
  - Run `nix develop -c -- cargo update` to update `Cargo.lock`.
- **Before building with Nix flakes:**
  - Stage and commit all changes to files referenced by `flake.nix` (including `Cargo.toml`, `Cargo.lock`, `src/main.rs`).
- **To build the CLI:**
  - `nix build .#default` (output in `result/bin/garnix-insights`).
- **To run directly:**
  - CLI: `nix run .#default -- <JWT_TOKEN> <COMMIT_ID>`
  - Server: `nix run .#server`
- **Development verification:**
  - Use `nix develop -c -- cargo check` for quick compilation check.
  - Use `nix develop -c -- cargo build` for full build in development mode.

## Release & Publishing Workflow
- **Automated releases**: Uses Google's release-please for semantic versioning and automated publishing.
- **Commit message format**: Must follow conventional commits (feat:, fix:, docs:, chore:, ci:, etc.)
- **Unicode/Emoji restrictions**: Commit messages must contain only ASCII characters - emojis cause ByteString conversion failures in release-please.
- **GitHub secrets required**:
  - `CARGO_REGISTRY_TOKEN`: For publishing to crates.io
  - `GH_TOKEN`: GitHub token with repo permissions for release creation
- **Git history management**: Use `git filter-branch` for Unicode cleanup if needed, not `git-filter-repo` (not available in nix environment).
- **To trigger releases**: Push semantic commits to main branch, release-please will create PRs and releases automatically.

## Rust Conventions
- Use `tokio` for async runtime; main entrypoint should use `block_on` for async logic.
- Error handling: prefer `Result<T, E>`, `Option<T>`, and `anyhow::Error` for fallible operations. Use `?` for propagation.
- API responses should mirror the `GarnixResponse` struct, including nested summaries and logs.
- Use emojis (✅/❌) for build status in outputs.

## Nix Build Specifics
- `flake.nix` uses `crane` for Rust builds; ensure `openssl` and `pkg-config` are in `buildInputs`.
- If OpenSSL errors occur, set `PKG_CONFIG_PATH`, `OPENSSL_DIR`, etc. to correct Nix store paths.
- **Python3 dependency**: Required for `git-filter-repo` tool in development shell.
- **Additional development tools**: `gh` (GitHub CLI) for secret management and workflow interaction.

## External Integration
- Communicates with Garnix.io API using JWT authentication.
- All build status and logs are fetched via HTTP requests.

## Patterns & Examples
- See `src/main.rs` for CLI and API logic.
- See `GUIDE.md` for API server structure and error handling patterns.

## Additional Notes
- Adhere to project-specific shell and build commands; do not use plain `cargo` or system binaries outside Nix shell.
- Reference `README.md` and `GUIDE.md` for up-to-date workflow and conventions.
- **Python files**: `test_mcp.py` and `convert-commits.py` are legacy scripts from development and can be removed if unused.
- **Release automation troubleshooting**: If release-please fails with ByteString errors, check for Unicode characters in commit messages using `git log --format='%B' --all | grep -P '[^\x00-\x7F]'`.

---

If any section is unclear or missing, please request clarification or provide feedback for improvement.
