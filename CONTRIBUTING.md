# Contributing to SwiftSC (swiftsc-cli)

Thank you for your interest in contributing to the SwiftSC ecosystem! We welcome contributions from developers of all skill levels.

## Code of Conduct

Please note that this project is released with a Contributor Code of Conduct. By participating in this project you agree to abide by its terms.

## CLI Development
*   **Commands**: Add new subcommands in `src/main.rs` using `clap`.
*   **UX**: Output should be colored and user-friendly.
*   **Scaffolding**: Updates to `new` command templates go in `templates/`.

## How to Contribute

1.  **Fork the repository** on GitHub.
2.  **Clone your fork** locally.
3.  **Create a new branch** for your feature or bugfix.
4.  **Make your changes**, ensuring you follow the project's coding standards.
5.  **Run tests** using `cargo test`.
6.  **Submit a Pull Request** to the `main` branch.

## Coding Standards

*   **Rust**: We use `rustfmt` and `clippy`. Please ensure your code passes `cargo fmt --check` and `cargo clippy`.
*   **Commits**: Use conventional commits (e.g., `feat: add new parser`, `fix: resolve memory leak`).

## Reporting Bugs

Please use the [Bug Report Template](.github/ISSUE_TEMPLATE/beta_bug_report.md) when opening an issue.
