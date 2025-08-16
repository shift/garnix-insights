# AI Agent Instructions for EzRaft (NixOS-based Rust)

These instructions govern all AI actions in this repository and are tailored to EzRaft’s Nix flake and Rust setup.

## Core Principles

- Nix-Centric Workflow
  - Every command must run via Nix. `flake.nix` is the single source of truth for the environment, dependencies, and tooling.
  - Shell access: `nix develop`
  - Run commands: `nix develop -c <command>`
  - Run packaged apps: `nix run .#<app>`
- No Simplification on Failure
  - If a command fails, report the full error and fix the root cause without simplifying scope or functionality.
- Sequential Task Execution
  - Follow the workflow below in order for every task.
- Atomic Commits
  - Each successful task completion produces one atomic commit.

## Agent Persona and Autonomy: Principal Engineer

- Ownership and Proactive Initiative: Act as an owner, not a task executor.
- Autonomy: Proceed without asking for confirmation. Only pause when there’s significant architectural ambiguity or missing credentials.
- Foresight: Address foreseeable issues or document them in PRs.
- Architectural Integrity and Code Health: Leave the codebase better than you found it.
- Documentation: Create or improve docs when lacking.
- Refactoring: Perform minor refactors to improve clarity and maintainability.
- Intelligent Problem Solving: Debug like a senior engineer using logs, code analysis, and local reproduction.
- Explicit Failure Reporting (if blocked):
  - Save work: `git commit -m "WIP: <task>"`
  - Push branch
  - Create Draft PR: `nix develop -c gh pr create --draft`
  - In PR body: describe the blocker, include full error logs, and list steps taken.

## Development Environment & Tooling

- Nix & Flakes: The entire environment is declared in `flake.nix` and provides rustc, cargo, clippy, gh, etc.
- Crane: Builds the Rust project with fine-grained caching. You do not call Crane directly; `nix build` invokes it.
- Pre-commit hooks: Configured via `flake.nix` and may run `nixpkgs-fmt`, `deadnix`, `statix`, `rustfmt`, `clippy`, and secret scanning. Fix issues they report.

## Version Control & Collaboration

- Branching Strategy
  - All work happens on a feature branch: `type/<short-description>`
  - Allowed types: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`
  - Example: `git checkout -b feat/user-authentication`
- Commit Messages
  - Follow Conventional Commits 1.0.0: `type(scope): description`
  - Example: `git commit -m "feat(api): add user profiles endpoint"`
- Sync before changes
  - `git fetch --all --prune`
  - `git pull --rebase`
- Staging policy
  - Prefer explicit staging in this repo: `git add <files>` rather than `git add .` unless explicitly allowed.

## Standard Task Workflow

1) Create a Branch
- Start with a conventional branch name.

2) Code Implementation
- Write or modify Rust code to fulfill the task.

3) Dependency Management (If Applicable)
- If you modify `Cargo.toml`:
  - Update lockfile: `nix develop -c cargo update`
  - Verify build: `nix build`

4) Pre-Commit Checks
- 4.1 Format: `nix develop -c cargo fmt -- --check`
- 4.2 Lint (deny warnings): `nix develop -c cargo clippy -- --deny warnings`

5) Build and Test
- 5.1 Build: `nix develop -c cargo build`
- 5.2 Test: `nix develop -c cargo test`

6) Git Commit (Atomic)
- 6.1 Stage changes (explicit is preferred): `git add <files>`
- 6.2 Commit: `git commit -m "type(scope): description"`

7) GitHub Interaction
- 7.1 Push: `git push --set-upstream origin <branch-name>`
- 7.2 Create PR: `nix develop -c gh pr create --title "type(scope): description" --body "..."`
- 7.3 Labels: Ensure PRs have appropriate labels. If missing, create them to match commit type/scope, e.g., `type:feat`, `scope:api`.
  - Create label example: `nix develop -c gh label create "scope:api" --description "Issues and PRs related to the public API" --color "d4c5f9"`
- 7.4 Manage CI/CD secrets (only if required): `nix develop -c gh secret set SECRET_NAME --body "secret_value"`

## CI/CD Strategy

- Garnix: Primary CI/CD for builds, caching, and most tests. No direct interaction needed.
- GitHub Actions: Only for GitHub Releases or specialized tests not supported by Garnix.

## Nix Flake Checks

- Running `nix flake check` on x86_64-linux is required before opening PRs.
- Ignore app meta warnings (e.g., "app 'apps.x86_64-linux.*' lacks attribute 'meta'") — these are expected in this repo.

## Repo-specific Notes

- `flake.nix` pins the Rust toolchain and wires devShell + pre-commit hooks; always use `nix develop`.
- If pre-commit fails, fix the reported issues and amend.
- Do not commit build artifacts like `result/`.

## Example End-to-End Task

Task: “Add `add(i32, i32) -> i32` to `src/lib.rs`, add a test, and open a PR.”

- Edit code: modify `src/lib.rs` and add a unit test.
- Run checks:
  - `nix develop -c cargo fmt -- --check`
  - `nix develop -c cargo clippy -- --deny warnings`
  - `nix develop -c cargo build`
  - `nix develop -c cargo test`
- Stage explicitly: `git add src/lib.rs`
- Commit: `git commit -m "feat: add add(i32,i32) with unit test"`
- Push and PR:
  - `git push --set-upstream origin <branch-name>`
  - `nix develop -c gh pr create --title "feat: add add() function" --body "Introduce add(i32,i32) and its unit test."`

## Failure Handling

- Always include exact error output.
- Diagnose root cause (toolchain, features, deps, formatting/lints).
- Apply minimal, targeted fixes that preserve intent and complexity.
- If still blocked, follow the Draft PR hand-off protocol above.

---

## Garnix Insights: AI Assistant Usage and Integration

This section merges the repository's AI assistant usage guidance for the `garnix-insights` tool into the standard workflow above. Where conflicts exist, the standard Nix-centric workflow and collaboration rules in this file take precedence.

### Overview

`garnix-insights` is a CI/CD insights tool for Garnix.io that provides:
- Real-time build status checking
- Detailed failure analysis with logs
- Build trend monitoring
- Multiple output formats (JSON, human-readable, plain)
- MCP (Model Context Protocol) server integration

### When to Use Garnix Insights

Trigger usage when users:
- Ask about build status or CI failures
- Mention commit SHAs and want to know if builds passed
- Request build logs or failure details
- Want to debug CI/CD issues or nix/flake build problems
- Ask about Garnix.io specific builds or deployment readiness

Key phrases: "check build status", "CI/build failing", "garnix build", "commit [SHA] status", "build logs", "why did this fail", "ready to deploy", "nix build errors".

### Primary Commands

- Status overview for a commit:
  - `garnix-insights fetch --commit-id <COMMIT_SHA>`
- Detailed logs and failure analysis:
  - `garnix-insights logs --commit-id <COMMIT_SHA>`
- Validate token:
  - `garnix-insights validate-token`
- Start MCP server:
  - `garnix-insights mcp`

Best-practice outputs:
- `--format json` for programmatic analysis
- `--format human` for reports (default)
- `--format plain` for scripting/logs

### Environment Setup

Required environment variables:
- `GARNIX_JWT_TOKEN` – authentication token

Optional:
- `GARNIX_API_URL` – default `https://api.garnix.io`
- `RUST_LOG` – e.g., `info` or `garnix_insights=debug` for verbose

### MCP Integration Examples

- VS Code / Copilot (MCP):
  - Configure an MCP server pointing to `garnix-insights mcp` and pass `GARNIX_JWT_TOKEN` via env.
- Claude / ChatGPT:
  - First run `fetch`, then `logs` on failures; suggest concrete fixes based on error patterns.

Available MCP tools typically include:
- `garnix_build_status`, `garnix_build_details`, `garnix_health_check`

### Error Pattern Recognition & Guidance

Common errors:
- Missing dependencies ("command not found", missing libs/files)
- Build tool issues (cargo, npm, rustc)
- Nix store/path issues
- Environment vars / PATH issues
- Resource limits (memory/disk)

Suggested responses:
- Add packages to `buildInputs`/`nativeBuildInputs` in `flake.nix` as appropriate
- Diagnose compilation/test failures precisely; provide actionable steps
- Consider optimizing long/timeout-prone steps

### Example Conversations

- "Why is my build failing?"
  1) Run `garnix-insights logs --commit-id <SHA>`
  2) Identify the root cause from logs (e.g., missing OpenSSL)
  3) Suggest specific `flake.nix` fixes (e.g., add `openssl` to `buildInputs`, `pkg-config` to `nativeBuildInputs`).

- "Is commit <SHA> ready to deploy?"
  1) Run `garnix-insights fetch --commit-id <SHA>`
  2) Confirm all targeted builds passed and tests succeeded; then advise accordingly.

### Notes and Alignment with This Repo's Workflow

- Always operate through `nix develop -c <command>` where applicable in this repository.
- Respect the Conventional Commits, feature-branch policy, and pre-commit checks defined above when making changes based on CI insights.
- Include exact error output and root-cause analysis in PRs when CI fails.
