# GUIDE.md: Garnix API Server Development Guide

This document outlines the requirements for the Garnix API Server and best practices for its Rust implementation, especially considering the Nix build environment.

## 1. Project Goal

To create a Rust-based AI Multi-Cloud Platform (MCP) server that allows other AI agents (e.g., Gemini, Copilot, Claude) to programmatically access Garnix CI build status for specific commits. The server should expose build errors and logs to aid in debugging by these AI agents.

## 2. API Specification

### Endpoint: `/build-status`

- **Method:** `POST`
- **Request Body (JSON):**
  ```json
  {
    "jwt_token": "<YOUR_GARNIX_JWT_TOKEN>",
    "commit_id": "<GIT_COMMIT_ID>"
  }
  ```
- **Response Body (JSON):**
  The server should return a structured JSON response containing:
  - A summary of the commit's builds (e.g., total succeeded, failed, pending).
  - A list of all packages involved in the build, each with:
    - Package name
    - Build status (Pass/Fail, ideally using emojis like ✅/❌)
    - Detailed failure logs for broken builds.
  
  The structure should ideally mirror the `GarnixResponse` struct from the `garnix-fetcher` application, including nested `Summary`, `Build`, and `LogEntry` structures.

## 3. Rust Best Practices

### 3.1. Error Handling

- Use Rust's `Result<T, E>` and `Option<T>` for all fallible operations.
- Leverage the `?` operator for concise error propagation.
- Define custom error types where appropriate, or use `anyhow::Error` for simpler cases.
- Provide meaningful error messages that aid debugging.

### 3.2. Asynchronous Programming

- Use `tokio` as the asynchronous runtime.
- Employ `async/await` syntax for asynchronous operations (e.g., HTTP requests).
- The `main` function should be synchronous, and `tokio::runtime::Runtime::new().unwrap().block_on(async { ... })` should be used to execute the asynchronous server or CLI logic.
- **Crucially:** Ensure that `async` blocks passed to `block_on` explicitly return a `Result` type (e.g., `async -> Result<(), Box<dyn std::error.Error>>`) if they use the `?` operator. All paths within these `async` blocks must return `Ok(())` or propagate errors.

### 3.3. Dependency Management

- Manage dependencies via `Cargo.toml` and `Cargo.lock`.
- Keep `Cargo.lock` updated by running `nix develop -c -- cargo update` after modifying `Cargo.toml`.
- Prefer well-maintained and widely used crates (e.g., `reqwest` for HTTP, `serde` for serialization/deserialization, `actix-web` for web server).

### 3.4. Code Organization

- Organize code into logical modules and functions.
- Separate concerns (e.g., API interaction logic from server setup logic).
- Use clear and descriptive naming for variables, functions, and types.

### 3.5. Testing

- Implement unit tests for individual functions and modules.
- Consider integration tests for API endpoints to ensure end-to-end functionality.

### 3.6. Clarity and Readability

- Write clean, idiomatic Rust code.
- Add comments where the *why* of the code is not immediately obvious.
- Follow Rust formatting conventions (`cargo fmt`).

## 4. Nix Build Specifics

### 4.1. `flake.nix` Structure

- The `flake.nix` should define `devShells.default` for the development environment and `packages.default` for the buildable application.
- It should also define `apps.default` for the CLI and `apps.server` for the server, allowing easy execution via `nix run .#default` and `nix run .#server`.

### 4.2. `crane` Usage

- Use `crane` (specifically `crane.mkLib pkgs` and `craneLib.buildPackage`) for building the Rust project within Nix. This handles many complexities of Rust builds in Nix.
- Ensure `craneLib.cleanCargoSource ./.;` is used for the `src` to maintain Nix purity.

### 4.3. Handling OpenSSL and `pkg-config`

- This has been a persistent challenge. Ensure `openssl` and `pkg-config` are correctly provided to the build environment.
- They should be in `buildInputs` for `craneLib.buildPackage`.
- If `openssl-sys` continues to fail, consider explicitly setting environment variables like `PKG_CONFIG_PATH`, `OPENSSL_DIR`, `OPENSSL_LIB_DIR`, and `OPENSSL_INCLUDE_DIR` within the `buildPackage` definition, pointing to the correct Nix store paths for `pkgs.openssl`.

### 4.4. Git Integration

- **Crucial:** Always `git add` and `git commit` all relevant files (`flake.nix`, `Cargo.toml`, `Cargo.lock`, `src/main.rs`, etc.) before attempting a Nix build. Nix flakes operate on the Git working tree, and unstaged/uncommitted changes can lead to build failures or unexpected behavior.
