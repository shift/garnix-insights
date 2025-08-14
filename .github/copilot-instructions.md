# Copilot Instructions for Garnix Fetcher

## Project Overview
- This is a Rust application and API server for fetching and exposing Garnix.io CI build status for specific commits.
- The codebase is designed for use within a NixOS environment, leveraging Nix flakes for reproducible builds and development shells.

## Architecture & Key Files
- Main logic is in `src/main.rs`.
- API server requirements and conventions are described in `GUIDE.md`.
- Command-line usage and build instructions are in `README.md`.
- Nix build and environment setup is managed by `flake.nix`.

## Developer Workflow
- **Always use Nix shells for Rust commands:**
  - Build, run, and test with: `nix develop -c -- cargo <cmd>` (e.g., `cargo build`, `cargo run`, `cargo test`).
- **After changing Rust dependencies:**
  - Run `nix develop -c -- cargo update` to update `Cargo.lock`.
- **Before building with Nix flakes:**
  - Stage and commit all changes to files referenced by `flake.nix` (including `Cargo.toml`, `Cargo.lock`, `src/main.rs`).
- **To build the CLI:**
  - `nix build .#default` (output in `result/bin/garnix-fetcher`).
- **To run the server (if implemented):**
  - `nix run .#server`

## Rust Conventions
- Use `tokio` for async runtime; main entrypoint should use `block_on` for async logic.
- Error handling: prefer `Result<T, E>`, `Option<T>`, and `anyhow::Error` for fallible operations. Use `?` for propagation.
- API responses should mirror the `GarnixResponse` struct, including nested summaries and logs.
- Use emojis (✅/❌) for build status in outputs.

## Nix Build Specifics
- `flake.nix` uses `crane` for Rust builds; ensure `openssl` and `pkg-config` are in `buildInputs`.
- If OpenSSL errors occur, set `PKG_CONFIG_PATH`, `OPENSSL_DIR`, etc. to correct Nix store paths.

## External Integration
- Communicates with Garnix.io API using JWT authentication.
- All build status and logs are fetched via HTTP requests.

## Patterns & Examples
- See `src/main.rs` for CLI and API logic.
- See `GUIDE.md` for API server structure and error handling patterns.

## Additional Notes
- Adhere to project-specific shell and build commands; do not use plain `cargo` or system binaries outside Nix shell.
- Reference `README.md` and `GUIDE.md` for up-to-date workflow and conventions.

---

If any section is unclear or missing, please request clarification or provide feedback for improvement.
