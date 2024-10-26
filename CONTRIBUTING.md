# Contributing to the Kadena Rust Library

First off, thank you for considering contributing to the Kadena Rust Library! Your contributions help make this project better for everyone.

We welcome contributions of all kinds, including bug reports, feature requests, documentation improvements, and code contributions.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to abide by its terms.

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please check the [existing issues](https://github.com/ledger-things/kadena-rust-lib/issues) to see if the problem has already been reported. If not, you can help us by submitting a detailed and clear report.

**Bug Report Guidelines:**

- **Title**: Use a clear and descriptive title.
- **Description**: Provide a detailed description of the issue.
- **Steps to Reproduce**: List the steps to reproduce the issue.
- **Expected Behavior**: Explain what you expected to happen.
- **Actual Behavior**: Describe what actually happened.
- **Environment**: Include Rust version, OS, and any other relevant details.
- **Error Messages**: Attach any error messages, stack traces, or logs.

### Suggesting Enhancements

We welcome suggestions for new features or improvements.

**Enhancement Suggestion Guidelines:**

- **Title**: Use a concise and descriptive title.
- **Motivation**: Explain why this feature would be useful.
- **Description**: Provide a detailed description of the proposed enhancement.
- **Alternatives**: Mention any alternative solutions or features you've considered.
- **Additional Context**: Add any other context or screenshots about the feature request.

### Contributing Code

We appreciate your help in improving the library! To contribute code:

1. **Fork the Repository**

   Click the "Fork" button on the repository's page to create your own copy.

2. **Clone Your Fork**

   ```
   git clone https://github.com/your-username/kadena-rust-library.git
   cd kadena-rust-library
   ```

3. **Set Upstream Remote**

   ```
   git remote add upstream https://github.com/ledger-things/kadena-rust-lib.git
   ```

4. **Create a Feature Branch**

   ```
   git checkout -b feature/your-feature-name
   ```

5. **Install Dependencies**

   Ensure you have the latest stable Rust toolchain installed via [rustup](https://rustup.rs/).

   ```
   rustup update stable
   ```

6. **Implement Your Changes**

   - Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
   - Write idiomatic Rust code.
   - Document public APIs using Rustdoc comments (`///`).
   - Include unit tests and, if applicable, integration tests.

7. **Format and Lint Your Code**

   ```
   # Format code
   cargo fmt

   # Lint code
   cargo clippy -D warnings
   ```

8. **Run Tests**

   ```
   cargo test
   ```

9. **Commit Your Changes**

   ```
   git add .
   git commit -m "feat: add amazing new feature"
   ```

   **Commit Message Guidelines:**

   - Use [Conventional Commits](https://www.conventionalcommits.org/).
   - Start with a type (`feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`).
   - Use the imperative mood (e.g., "Add feature" not "Added feature").

10. **Push to Your Fork**

    ```
    git push origin feature/your-feature-name
    ```

11. **Open a Pull Request**

    - Go to your fork on GitHub and click the "Compare & pull request" button.
    - Fill out the pull request template.
    - Ensure the PR description clearly explains the problem and solution.
    - Link to any relevant issues.

### Pull Request Guidelines

- **Keep PRs Focused**: PRs should be as small as possible while still solving the problem.
- **One PR per Feature/Bugfix**: Avoid combining unrelated changes.
- **Pass CI Checks**: Ensure all Continuous Integration (CI) checks pass.
- **Respond to Feedback**: Be responsive to any code review comments.
- **Update Documentation**: Update any affected documentation in your PR.

### Code Review Process

- PRs will be reviewed by maintainers.
- Changes may be requested before merging.
- Once approved, your PR will be merged.

## Development Process

### Branching Strategy

- **Main Branch**: The latest stable release.
- **Feature Branches**: Use `feature/your-feature-name` for new features.
- **Bugfix Branches**: Use `fix/your-bugfix-name` for bug fixes.

### Setting Up the Development Environment

1. **Install Rust**

   Ensure you have Rust installed via [rustup](https://rustup.rs/).

2. **Clone the Repository**

   ```
   git clone https://github.com/ledger-things/kadena-rust-lib.git
   cd kadena-rust-library
   ```

3. **Run Tests**

   ```
   cargo test
   ```

### Running Tests

```
cargo test
```

### Linting and Formatting

- **Formatting**

  ```
  cargo fmt --all -- --check
  ```

- **Linting**

  ```
  cargo clippy -- -D warnings
  ```

### Continuous Integration

Our CI pipeline runs the following checks:

- Build
- Tests
- Formatting
- Linting

Ensure these pass before submitting a PR.

## Style Guide

- **Formatting**: Use `rustfmt`. CI will enforce code formatting.
- **Linting**: Use `clippy` to catch common mistakes.
- **Comments**: Use Rustdoc comments (`///`) for public items.
- **Error Handling**: Prefer using `Result` and `Option` over panicking.
- **Safety**: Avoid `unsafe` code unless necessary.

## Documentation

- Update documentation for any user-facing changes.
- Ensure examples are up to date.
- Use [docs.rs](https://docs.rs/) conventions.

## Commit Message Convention

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- **Format**: `<type>(optional scope): <description>`

  **Types**:

  - `feat`: A new feature
  - `fix`: A bug fix
  - `docs`: Documentation changes
  - `style`: Code style changes (formatting, missing semicolons, etc.)
  - `refactor`: Code changes that neither fix a bug nor add a feature
  - `test`: Adding or updating tests
  - `chore`: Changes to the build process or auxiliary tools

- **Examples**:

  - `feat: add support for transaction batching`
  - `fix(wallet): resolve panic on empty input`
  - `docs: update contributing guidelines`

## Communication

- **GitHub Issues**: Use for bug reports and feature requests.
- **Pull Requests**: Use for code submissions.
- **Discussions**: Join in [GitHub Discussions](https://github.com/ledger-things/kadena-rust-lib/discussions) for general topics.

## Contributor License Agreement

By contributing, you agree that your contributions will be licensed under the same license as the project, [MIT License](LICENSE).

## Acknowledgments

Thank you for your interest in contributing to the Kadena Rust Library! Your efforts help make this project successful.
