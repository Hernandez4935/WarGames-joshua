# GitHub Actions Workflows

This directory contains GitHub Actions workflows for continuous integration, deployment, and automation.

## Current Workflows

### CI Workflow (`ci.yml`)

**Status**: ✅ Active

The main CI/CD pipeline that runs on every push and pull request.

**Triggers**:
- Push to any branch
- Pull requests to any branch

**Jobs**:
1. **Test** - Runs on Ubuntu, macOS, and Windows
   - Checks out code
   - Sets up Rust toolchain (stable)
   - Caches dependencies for faster builds
   - Runs `cargo build`
   - Runs `cargo test`
   - Runs `cargo clippy -- -D warnings`
   - Checks formatting with `cargo fmt --check`

**Platforms**:
- Ubuntu Latest (primary)
- macOS Latest
- Windows Latest

**Duration**: ~5-10 minutes (depending on cache)

## Recommended Additional Workflows

The following workflows are recommended for future implementation as the project matures:

### 1. Release Workflow (`release.yml`)

**Purpose**: Automate the release process with binary builds for multiple platforms.

**Suggested Configuration**:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  create-release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1

  build-release:
    name: Build Release
    needs: create-release
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-apple-darwin
            os: macos-latest
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - uses: taiki-e/upload-rust-binary-action@v1
        with:
          bin: joshua
          target: ${{ matrix.target }}
          token: ${{ secrets.GITHUB_TOKEN }}
```

**Features**:
- Automatic binary builds for multiple platforms
- Cross-compilation for ARM architectures
- Upload to GitHub Releases
- Asset naming with version and platform

**When to Implement**: Before v0.2.0 release (after Phase 1 complete)

---

### 2. Security Audit Workflow (`security-audit.yml`)

**Purpose**: Daily security audits of dependencies for known vulnerabilities.

**Suggested Configuration**:
```yaml
name: Security Audit

on:
  schedule:
    - cron: '0 0 * * *'  # Daily at midnight UTC
  push:
    paths:
      - 'Cargo.toml'
      - 'Cargo.lock'
  pull_request:
    paths:
      - 'Cargo.toml'
      - 'Cargo.lock'

jobs:
  security-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

  cargo-deny:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: EmbarkStudios/cargo-deny-action@v1
        with:
          log-level: warn
          command: check
          arguments: --all-features
```

**Features**:
- Checks for known security vulnerabilities (RustSec database)
- License compliance checking
- Dependency graph analysis
- Ban list enforcement
- Daily scheduled runs

**When to Implement**: Immediately (Phase 1)

---

### 3. Documentation Workflow (`docs.yml`)

**Purpose**: Build and deploy documentation to GitHub Pages.

**Suggested Configuration**:
```yaml
name: Documentation

on:
  push:
    branches:
      - master
  pull_request:

jobs:
  build-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build Documentation
        run: cargo doc --no-deps --all-features
      - name: Add index.html redirect
        run: echo '<meta http-equiv="refresh" content="0; url=wargames_joshua">' > target/doc/index.html
      - name: Deploy to GitHub Pages
        if: github.ref == 'refs/heads/master'
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./target/doc
          cname: docs.your-domain.com  # Optional custom domain
```

**Features**:
- Builds rustdoc documentation
- Deploys to GitHub Pages (master branch only)
- Index redirect to main crate documentation
- Optional custom domain support

**When to Implement**: Phase 2 (when API surface is stable)

---

### 4. Benchmark Workflow (`benchmark.yml`)

**Purpose**: Track performance over time and detect regressions.

**Suggested Configuration**:
```yaml
name: Benchmark

on:
  push:
    branches:
      - master
  pull_request:

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run Benchmarks
        run: cargo bench --bench risk_calculation -- --save-baseline master
      - name: Store Benchmark Results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/master/estimates.json
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
          # Alert if performance degrades by 5%
          alert-threshold: '105%'
          comment-on-alert: true
          fail-on-alert: false
```

**Features**:
- Runs Criterion benchmarks
- Stores historical benchmark data
- Compares PR benchmarks against master
- Alerts on performance regressions
- Generates performance graphs

**When to Implement**: Phase 3 (when risk calculation is implemented)

---

### 5. Code Coverage Workflow (`coverage.yml`)

**Purpose**: Track test coverage and enforce minimum thresholds.

**Suggested Configuration**:
```yaml
name: Code Coverage

on:
  push:
    branches:
      - master
  pull_request:

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview
      - name: Install cargo-llvm-cov
        uses: taiki-e/install-action@cargo-llvm-cov
      - name: Generate Coverage
        run: cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
      - name: Upload to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info
          fail_ci_if_error: true
          token: ${{ secrets.CODECOV_TOKEN }}
      - name: Check Coverage Threshold
        run: |
          coverage=$(cargo llvm-cov --all-features --workspace --summary-only | grep "TOTAL" | awk '{print $10}' | sed 's/%//')
          if (( $(echo "$coverage < 95" | bc -l) )); then
            echo "Coverage $coverage% is below 95% threshold"
            exit 1
          fi
```

**Features**:
- Generates LCOV coverage reports
- Uploads to Codecov for visualization
- Enforces 95% coverage threshold
- Fails CI if coverage drops below target
- Per-file and per-function coverage metrics

**When to Implement**: Phase 1 (establish baseline early)

---

### 6. Dependency Update Workflow (`dependency-update.yml`)

**Purpose**: Automated dependency updates with Dependabot and testing.

**Note**: This uses Dependabot configuration, not a workflow file.

**Suggested Configuration** (`.github/dependabot.yml`):
```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    open-pull-requests-limit: 5
    reviewers:
      - "yourusername"
    assignees:
      - "yourusername"
    commit-message:
      prefix: "chore(deps)"
    labels:
      - "dependencies"
      - "automated"
    ignore:
      # Ignore major version updates for stable dependencies
      - dependency-name: "tokio"
        update-types: ["version-update:semver-major"]
```

**Features**:
- Weekly dependency update checks
- Automatic PR creation
- Grouped updates by type
- Custom commit message format
- Automatic assignment and labeling

**When to Implement**: Immediately (Phase 0/1)

---

### 7. Nightly Build Workflow (`nightly.yml`)

**Purpose**: Test against Rust nightly to catch future compatibility issues.

**Suggested Configuration**:
```yaml
name: Nightly Build

on:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM UTC
  workflow_dispatch:  # Allow manual trigger

jobs:
  test-nightly:
    runs-on: ubuntu-latest
    continue-on-error: true  # Don't fail if nightly breaks
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
      - name: Build with Nightly
        run: cargo +nightly build --all-features
      - name: Test with Nightly
        run: cargo +nightly test --all-features
      - name: Create Issue on Failure
        if: failure()
        uses: actions/github-script@v6
        with:
          script: |
            github.rest.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: 'Nightly Build Failure',
              body: 'The nightly build has failed. Please investigate.',
              labels: ['nightly-failure', 'needs-triage']
            })
```

**Features**:
- Daily testing against Rust nightly
- Automatic issue creation on failure
- Doesn't block stable releases
- Early warning of upcoming breaking changes

**When to Implement**: Phase 2 (when codebase is stable)

---

## Workflow Best Practices

### General Guidelines

1. **Use Caching**: Always cache dependencies to speed up builds
   ```yaml
   - uses: Swatinem/rust-cache@v2
   ```

2. **Matrix Testing**: Test on multiple platforms
   ```yaml
   strategy:
     matrix:
       os: [ubuntu-latest, macos-latest, windows-latest]
   ```

3. **Fail Fast**: Set `fail-fast: false` to see all platform failures
   ```yaml
   strategy:
     fail-fast: false
   ```

4. **Timeouts**: Set reasonable timeouts
   ```yaml
   timeout-minutes: 30
   ```

5. **Secrets Management**: Use GitHub Secrets for sensitive data
   ```yaml
   env:
     ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
   ```

### Security Considerations

- **Never log secrets**: Use `::add-mask::` or `echo "::add-mask::$SECRET"`
- **Use pinned actions**: Pin to specific SHA for security
  ```yaml
  uses: actions/checkout@8e5e7e5ab8b370d6c329ec480221332ada57f0ab  # v4.1.0
  ```
- **Review third-party actions**: Audit before using
- **Limit permissions**: Use minimal required permissions
  ```yaml
  permissions:
    contents: read
  ```

### Performance Optimization

- **Parallel Jobs**: Run independent jobs in parallel
- **Conditional Steps**: Skip unnecessary steps
  ```yaml
  - name: Deploy
    if: github.ref == 'refs/heads/master'
  ```
- **Artifact Caching**: Cache build artifacts between jobs
- **Shallow Clones**: Use shallow git clones when full history isn't needed
  ```yaml
  - uses: actions/checkout@v4
    with:
      fetch-depth: 1
  ```

## Implementation Priority

| Workflow | Priority | Phase | Complexity |
|----------|----------|-------|------------|
| Security Audit | High | 1 | Low |
| Dependency Update | High | 1 | Low |
| Code Coverage | Medium | 1-2 | Medium |
| Documentation | Medium | 2 | Low |
| Benchmark | Low | 3 | Medium |
| Release | High | 1 | Medium |
| Nightly | Low | 2 | Low |

## Monitoring and Alerts

Set up notifications for workflow failures:

1. **GitHub Notifications**: Enable in repository settings
2. **Email Alerts**: Configure in personal settings
3. **Slack Integration**: Use GitHub Slack app
4. **Status Badges**: Add to README.md
   ```markdown
   [![CI](https://github.com/user/repo/workflows/CI/badge.svg)](https://github.com/user/repo/actions)
   ```

## Testing Workflows Locally

Use [act](https://github.com/nektos/act) to test workflows locally:

```bash
# Install act
brew install act  # macOS
# or download from GitHub releases

# Run workflows locally
act push                    # Simulate push event
act pull_request            # Simulate PR event
act -j test                 # Run specific job
act -n                      # Dry run (show what would run)
```

## Maintenance

- Review workflows quarterly
- Update action versions regularly
- Monitor workflow execution times
- Optimize slow workflows
- Archive unused workflows

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust GitHub Actions Guide](https://github.com/actions-rs)
- [Cargo Book - CI](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [GitHub Actions Security Best Practices](https://docs.github.com/en/actions/security-guides)

---

For questions about workflows, open a discussion or issue in the repository.
