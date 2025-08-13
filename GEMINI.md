# GEMINI.md for garnix-fetcher

## Project Overview

This project (`garnix-fetcher`) is a Rust application designed to fetch and parse build status information from the Garnix.io API. It provides a command-line interface to retrieve a summary of a commit's builds, a detailed table of all packages with their pass/fail status (using emojis), and logs for failed builds.

## NixOS Environment

This project is developed within a NixOS environment and leverages Nix flakes for dependency management and consistent builds. 

- **All shell commands** related to development (e.g., `cargo build`, `cargo run`, `cargo test`) **MUST be prefixed with `nix develop -c --`** to ensure they run within the correct Nix environment defined by `flake.nix`.
- For one-off commands (e.g., `curl`, `git`), prefer using `nix run nixpkgs#<pkg_name> -- <command_args>` if the package is available in `nixpkgs`.
- The `flake.nix` defines the development shell (`devShells.default`) and the buildable package (`packages.default`).

## Rust Development

- Use standard `cargo` commands for building, running, and testing the Rust application.
- Ensure `Cargo.toml` and `Cargo.lock` are kept up-to-date with dependencies.
- When adding new Rust dependencies, run `nix develop -c -- cargo update` to update `Cargo.lock`.

## Git Workflow

- **Always `git add` and `git commit` any changes** to files referenced by `flake.nix` (e.g., `flake.nix` itself, `Cargo.toml`, `Cargo.lock`, `src/main.rs`). Nix flakes rely on Git's working tree, and uncommitted changes can lead to build failures or unexpected behavior.
- Use clear and concise commit messages.

## Debugging Build Failures

- If a Nix build fails, examine the error messages carefully.
- For detailed logs of a failed Nix derivation, use `nix log <DRV_PATH>`, where `<DRV_PATH>` is provided in the build error output.
- For Rust compilation errors, ensure the `devShell` is correctly set up (i.e., `nix develop -c --` is used) and check the Rust compiler output.

## General AI Guidelines

- **Adhere strictly to existing project conventions** (formatting, naming, code structure).
- **Verify library availability** within the project's `flake.nix` before assuming usage.
- **Explain critical commands** (especially those modifying the file system or system state) before execution.
- **Be proactive** in fulfilling the user's request, including reasonable, directly implied follow-up actions.
- **Confirm ambiguity** or significant actions with the user before proceeding.
- **Do not revert changes** unless explicitly requested or if they lead to an error.
