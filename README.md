# Garnix Fetcher

A Rust application to fetch and parse build status information from the Garnix.io API.

## Features

- Retrieve a summary of a commit's builds.
- Display a detailed table of all packages with their pass/fail status (using emojis).
- Fetch and display logs for failed builds.
- Command-line interface for easy usage.

## Installation

This project is built using Nix flakes, ensuring a reproducible development environment.

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/garnix-fetcher.git
    cd garnix-fetcher
    ```

2.  **Build the application using Nix:**

    ```bash
    nix build .#default
    ```

    This will build the `garnix-fetcher` executable and place it in `result/bin/garnix-fetcher`.

## Usage

The application requires a JWT token for authentication and a commit ID to fetch build information.

```bash
./result/bin/garnix-fetcher <JWT_TOKEN> <COMMIT_ID>
```

**Example:**

```bash
./result/bin/garnix-fetcher "your_jwt_token_here" "3402d0072ce57370ed58ce28fe879c32a3501392"
```

## Licensing

This project is dual-licensed under the MIT License and the Apache License 2.0.

- See [LICENSE-MIT](LICENSE-MIT) for details on the MIT License.
- See [LICENSE-APACHE](LICENSE-APACHE) for details on the Apache License 2.0.

## Contributing

Contributions are welcome! Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project. (Note: This file is not yet created, but will be added soon.)
