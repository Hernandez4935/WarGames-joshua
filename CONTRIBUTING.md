# Contributing to WarGames/JOSHUA

Thank you for your interest in contributing to the WarGames/JOSHUA nuclear risk assessment system! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Message Conventions](#commit-message-conventions)
- [Pull Request Process](#pull-request-process)
- [Phase-Based Development](#phase-based-development)
- [Documentation Requirements](#documentation-requirements)
- [Security Considerations](#security-considerations)
- [Performance Guidelines](#performance-guidelines)
- [Getting Help](#getting-help)

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report, include as many details as possible using our bug report template.

**Good bug reports include:**
- Clear, descriptive title
- Exact steps to reproduce the problem
- Expected behavior vs. actual behavior
- Version information (Rust version, OS, project version)
- Relevant logs or error messages
- Screenshots if applicable

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- Use a clear, descriptive title
- Provide detailed description of the proposed feature
- Explain why this enhancement would be useful
- List any alternatives you've considered
- Include mockups or examples if applicable

### Contributing Code

We welcome code contributions! Here's how to get started:

1. Check existing issues or create a new one to discuss your proposed changes
2. Fork the repository
3. Create a feature branch from `master`
4. Make your changes following our coding standards
5. Add or update tests as needed
6. Ensure all tests pass and code is formatted
7. Submit a pull request

### Improving Documentation

Documentation improvements are always welcome:
- Fix typos or clarify existing documentation
- Add examples or improve existing ones
- Document undocumented features
- Improve rustdoc comments
- Update planning documents if architectures change

## Development Setup

### Prerequisites

- **Rust**: 1.75 or later
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update
  ```

- **PostgreSQL**: 14 or later (optional for development)
  ```bash
  # Ubuntu/Debian
  sudo apt-get install postgresql postgresql-contrib

  # macOS
  brew install postgresql

  # Windows
  # Download from https://www.postgresql.org/download/windows/
  ```

- **Anthropic API Key**: For Claude integration (Phase 2+)
  ```bash
  export ANTHROPIC_API_KEY="your-api-key-here"
  ```

### Initial Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/wargames-joshua.git
   cd wargames-joshua
   ```

2. **Install development tools:**
   ```bash
   # Code formatter
   rustup component add rustfmt

   # Linter
   rustup component add clippy

   # Coverage tool (optional)
   cargo install cargo-tarpaulin

   # Documentation builder
   cargo install cargo-doc
   ```

3. **Build the project:**
   ```bash
   cargo build
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

5. **Set up configuration:**
   ```bash
   cp config/default_config.toml config/local_config.toml
   # Edit local_config.toml with your settings
   ```

## Development Workflow

### Branch Naming

Use descriptive branch names that follow this pattern:

- `feat/feature-name` - New features
- `fix/bug-description` - Bug fixes
- `docs/what-changed` - Documentation updates
- `refactor/component-name` - Code refactoring
- `test/test-description` - Test additions or improvements
- `perf/optimization-area` - Performance improvements
- `chore/task-description` - Maintenance tasks

Examples:
- `feat/claude-api-integration`
- `fix/database-connection-pool-leak`
- `docs/risk-calculation-methodology`
- `refactor/data-collection-engine`

### Development Cycle

1. **Create a branch:**
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. **Make changes:**
   - Write code following our coding standards
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes:**
   ```bash
   # Run all tests
   cargo test

   # Run specific test
   cargo test test_name

   # Run with output
   cargo test -- --nocapture

   # Check code coverage (optional)
   cargo tarpaulin --out Html
   ```

4. **Format and lint:**
   ```bash
   # Format code
   cargo fmt

   # Check formatting
   cargo fmt --check

   # Run clippy
   cargo clippy -- -D warnings
   ```

5. **Commit changes:**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

6. **Push to your fork:**
   ```bash
   git push origin feat/your-feature-name
   ```

7. **Create a pull request** on GitHub

## Coding Standards

### Rust Style Guide

We follow the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and enforce them with `rustfmt` and `clippy`.

#### Code Formatting

- **Always run `cargo fmt` before committing**
- Use 4-space indentation (configured in rustfmt.toml)
- Maximum line length: 100 characters
- Use trailing commas in multi-line constructs

#### Naming Conventions

- **Types**: `PascalCase` (structs, enums, traits)
  ```rust
  struct RiskAssessment { ... }
  enum AlertLevel { ... }
  trait DataCollector { ... }
  ```

- **Functions and methods**: `snake_case`
  ```rust
  fn calculate_risk_score() -> f64 { ... }
  fn process_data_point(&self, point: DataPoint) { ... }
  ```

- **Constants**: `SCREAMING_SNAKE_CASE`
  ```rust
  const DOOMSDAY_CLOCK_BASELINE: u32 = 89;
  const MAX_RETRY_ATTEMPTS: usize = 3;
  ```

- **Module names**: `snake_case`
  ```rust
  mod data_collection;
  mod risk_calculation;
  ```

#### Error Handling

- Use `Result<T, Error>` for fallible operations
- Use `thiserror` for error type definitions
- Provide context with error messages
- Avoid `.unwrap()` and `.expect()` in production code
- Use `?` operator for error propagation

```rust
// Good
fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Error::Configuration(format!("Failed to read config: {}", e)))?;
    toml::from_str(&content)
        .map_err(|e| Error::Configuration(format!("Invalid TOML: {}", e)))
}

// Bad
fn load_config(path: &Path) -> Config {
    let content = std::fs::read_to_string(path).unwrap(); // Don't do this!
    toml::from_str(&content).expect("Invalid config") // Or this!
}
```

#### Async Code

- Use `async-trait` for trait methods
- Prefer `tokio::spawn` for concurrent tasks
- Use `.await` instead of blocking operations
- Handle cancellation gracefully

```rust
#[async_trait]
pub trait DataCollector: Send + Sync {
    async fn collect(&self) -> Result<Vec<DataPoint>>;
    fn source_name(&self) -> &str;
    fn reliability_score(&self) -> f64;
}
```

#### Documentation

- Document all public APIs with rustdoc comments (`///`)
- Include examples in documentation
- Document panics, errors, and safety requirements
- Use markdown formatting in doc comments

```rust
/// Calculates the overall nuclear risk score based on multiple factors.
///
/// # Arguments
///
/// * `factors` - A slice of risk factors to consider
/// * `weights` - Weight configuration for each risk category
///
/// # Returns
///
/// Returns a `Result` containing the calculated risk score (0-1000) or an error
/// if the calculation fails.
///
/// # Errors
///
/// Returns `Error::RiskCalculation` if:
/// - Weights don't sum to 1.0
/// - Any factor has invalid values
///
/// # Examples
///
/// ```
/// use wargames_joshua::{RiskFactor, calculate_risk_score};
///
/// let factors = vec![/* ... */];
/// let score = calculate_risk_score(&factors)?;
/// println!("Risk score: {}", score);
/// ```
pub fn calculate_risk_score(
    factors: &[RiskFactor],
    weights: &RiskWeights,
) -> Result<f64> {
    // Implementation
}
```

### Testing Standards

We maintain a **95%+ test coverage** target. All contributions should include appropriate tests.

#### Test Organization

- **Unit tests**: In the same file as the code, in a `#[cfg(test)]` module
- **Integration tests**: In the `tests/` directory
- **Doc tests**: In documentation comments
- **Benchmarks**: In the `benches/` directory

#### Test Naming

Use descriptive test names that explain what is being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_factor_creation_with_valid_values() {
        // Test implementation
    }

    #[test]
    fn test_risk_factor_rejects_invalid_weight() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_data_collector_handles_timeout() {
        // Async test implementation
    }
}
```

#### What to Test

- **Happy path**: Normal, expected usage
- **Edge cases**: Boundary conditions, empty inputs
- **Error conditions**: Invalid inputs, network failures
- **Concurrent behavior**: Race conditions, deadlocks
- **Performance**: Critical paths, algorithmic complexity

#### Test Quality

- Tests should be **independent** (no shared state)
- Tests should be **deterministic** (no flaky tests)
- Use **mocks** for external dependencies
- Use **property-based testing** for complex logic (with `proptest`)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_risk_score_always_in_valid_range(
            weight in 0.0f64..1.0,
            severity in 0u8..4,
        ) {
            let factor = RiskFactor::new("test", weight, severity);
            let score = factor.calculate_contribution();
            assert!(score >= 0.0 && score <= 1000.0);
        }
    }
}
```

## Commit Message Conventions

We follow [Conventional Commits](https://www.conventionalcommits.org/) for clear, structured commit history.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, missing semicolons, etc.)
- `refactor`: Code refactoring (no functional changes)
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks, dependency updates
- `ci`: CI/CD configuration changes

### Examples

```
feat(data-collection): add Reuters API integration

Implements data collection from Reuters API with rate limiting
and error handling. Includes caching layer for efficiency.

Closes #123
```

```
fix(database): resolve connection pool exhaustion

The connection pool was not properly releasing connections after
failed queries. Added explicit cleanup in error paths.

Fixes #456
```

```
docs(readme): update installation instructions

Added PostgreSQL setup steps and clarified API key configuration
for new contributors.
```

### Scope Guidelines

Use component names as scopes:
- `cli` - Command-line interface
- `data-collection` - Data collection engine
- `risk-calculation` - Risk calculation engine
- `claude-api` - Claude integration
- `database` - Database operations
- `visualization` - Visualization engine
- `config` - Configuration system
- `test` - Testing infrastructure

## Pull Request Process

### Before Submitting

1. **Ensure all tests pass:**
   ```bash
   cargo test --all-features
   ```

2. **Run clippy with no warnings:**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

3. **Format your code:**
   ```bash
   cargo fmt
   ```

4. **Update documentation:**
   - Add or update rustdoc comments
   - Update CHANGELOG.md with your changes
   - Update README.md if needed (features, usage examples)

5. **Test on multiple platforms** (if possible):
   - Linux (Ubuntu/Debian)
   - macOS
   - Windows

### Pull Request Checklist

Use this checklist in your PR description:

```markdown
## Pull Request Checklist

- [ ] Code follows the project's coding standards
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation updated (rustdoc, README, CHANGELOG)
- [ ] Tests added/updated for new functionality
- [ ] Commit messages follow conventional commits format
- [ ] No breaking changes (or clearly documented)
- [ ] Related issue referenced (if applicable)
```

### Review Process

1. **Automated checks** run on all PRs (CI/CD)
2. **Code review** by at least one maintainer
3. **Discussion** and iteration as needed
4. **Approval** and merge by maintainer

### Review Expectations

Reviewers will check for:
- Code quality and adherence to standards
- Test coverage and quality
- Documentation completeness
- Performance implications
- Security considerations
- Breaking changes
- Alignment with project architecture

## Phase-Based Development

The project follows a structured 6-phase development plan. Contributions should align with the current phase or upcoming phases.

### Current Phase: Phase 1 (Data Collection Engine)

Focus areas:
- RSS feed integration
- News API implementations
- Think tank data sources
- Content processing pipeline
- Caching layer

### Contributing to Future Phases

If you want to contribute to features in later phases:
1. Discuss in an issue first
2. Ensure compatibility with current architecture
3. Be prepared for changes as earlier phases evolve

### Phase Documentation

See [Development Roadmap](docs/01_Development_Roadmap_and_Sprint_Planning.md) for detailed phase breakdown and timelines.

## Documentation Requirements

### Code Documentation

- **Public APIs**: Must have rustdoc comments
- **Modules**: Must have module-level documentation
- **Complex algorithms**: Inline comments explaining logic
- **Examples**: Include usage examples in rustdoc

### External Documentation

Update these files when appropriate:
- `CHANGELOG.md` - All user-facing changes
- `README.md` - New features, changed usage
- `docs/` - Architecture changes, new guides
- `CONTRIBUTING.md` - Process changes

### Documentation Style

- Use clear, concise language
- Include code examples
- Link to related documentation
- Keep up-to-date with code changes

## Security Considerations

### Sensitive Data

- **Never commit secrets** (API keys, passwords, tokens)
- Use environment variables for sensitive configuration
- Add sensitive files to `.gitignore`
- Use `config/local_config.toml` for local credentials (gitignored)

### Security Best Practices

- Validate all external inputs
- Use parameterized queries (SQLx prepared statements)
- Implement rate limiting for APIs
- Use TLS for all network communications
- Log security-relevant events
- Handle errors without exposing sensitive information

### Reporting Security Vulnerabilities

**Do not open public issues for security vulnerabilities.**

See [SECURITY.md](SECURITY.md) for responsible disclosure procedures.

## Performance Guidelines

### Performance Considerations

- Profile before optimizing
- Use appropriate data structures (Vec vs. HashMap, etc.)
- Minimize allocations in hot paths
- Use `rayon` for CPU-bound parallelism
- Use `tokio` for I/O-bound concurrency
- Consider memory usage for large datasets

### Benchmarking

Use `criterion` for performance-critical code:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_risk_calculation(c: &mut Criterion) {
    let factors = create_test_factors();

    c.bench_function("calculate_risk_score", |b| {
        b.iter(|| calculate_risk_score(black_box(&factors)))
    });
}

criterion_group!(benches, benchmark_risk_calculation);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

### Performance Targets

- Assessment completion: < 5 minutes
- Memory usage: < 500MB
- Database query response: < 100ms (P95)
- API response caching hit rate: > 80%

## Getting Help

### Resources

- **Documentation**: Start with [README.md](README.md) and [docs/](docs/)
- **Issues**: Search existing issues for similar problems
- **Discussions**: Use GitHub Discussions for questions
- **Code Examples**: Check tests and documentation examples

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: Questions, ideas, general discussion
- **Pull Request Comments**: Code-specific discussions

### Tips for Getting Help

- Provide context and details
- Include error messages and logs
- Share minimal reproducible examples
- Be respectful and patient

## Attribution

Contributors are recognized in the following ways:
- Listed in [AUTHORS.md](AUTHORS.md)
- Mentioned in release notes for significant contributions
- Git commit history preserves authorship

Thank you for contributing to WarGames/JOSHUA! Your efforts help create a more transparent and data-driven approach to nuclear risk assessment.

---

*"A strange game. The only winning move is not to play."*
