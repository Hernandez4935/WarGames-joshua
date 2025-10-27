## Description

<!-- Provide a clear and concise description of your changes -->

## Type of Change

<!-- Check the boxes that apply to this PR -->

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Code refactoring (no functional changes)
- [ ] Performance improvement
- [ ] Test coverage improvement
- [ ] Build/CI configuration change
- [ ] Dependency update

## Related Issues

<!-- Link to related issues using the format: Closes #123, Fixes #456, Relates to #789 -->

Closes #
Fixes #
Relates to #

## Changes Made

<!-- Provide a detailed list of changes -->

-
-
-

## Phase/Sprint Reference

<!-- Which development phase or sprint does this relate to? -->

- Phase: <!-- e.g., Phase 1: Data Collection Engine -->
- Sprint: <!-- e.g., Sprint 1.2 -->
- Related Planning Doc: <!-- e.g., docs/01_Development_Roadmap_and_Sprint_Planning.md -->

## Testing Checklist

<!-- Check all that apply -->

- [ ] All existing tests pass (`cargo test`)
- [ ] New tests added for new functionality
- [ ] Integration tests added/updated
- [ ] Test coverage maintained or improved (target: 95%+)
- [ ] Property-based tests added (if applicable)
- [ ] Manual testing performed
- [ ] Edge cases tested
- [ ] Error handling tested

### Test Results

<!-- Paste relevant test output -->

```bash
# cargo test output
```

**Test Coverage**: <!-- e.g., 96.5% -->

## Code Quality Checklist

<!-- Check all that apply -->

- [ ] Code follows project coding standards
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted with rustfmt (`cargo fmt`)
- [ ] No new compiler warnings
- [ ] Passes `cargo check --all-features`
- [ ] Documentation builds without errors (`cargo doc`)

### Clippy Output

<!-- If there are any clippy warnings, explain them here -->

```bash
# cargo clippy output
```

## Documentation Checklist

<!-- Check all that apply -->

- [ ] Public APIs documented with rustdoc
- [ ] Module-level documentation updated
- [ ] README.md updated (if user-facing changes)
- [ ] CHANGELOG.md updated with changes
- [ ] Code comments added for complex logic
- [ ] Examples added/updated in documentation
- [ ] Architecture diagrams updated (if applicable)
- [ ] Planning documents updated (if applicable)

## Breaking Changes

<!-- If this PR introduces breaking changes, describe them here -->

### Breaking Changes Description

<!-- What breaks? Why was this necessary? -->

### Migration Guide

<!-- How should users migrate from the old behavior to the new? -->

```rust
// Before
// ...

// After
// ...
```

## Performance Impact

<!-- Describe any performance implications -->

- [ ] No performance impact
- [ ] Performance improved (provide benchmarks)
- [ ] Performance degraded (explain why and if acceptable)
- [ ] Benchmarks added/updated

### Benchmark Results

<!-- If applicable, paste benchmark results -->

```bash
# cargo bench output
```

## Security Considerations

<!-- Address any security implications -->

- [ ] No security implications
- [ ] Security improved (describe how)
- [ ] Potential security concern (explain and justify)
- [ ] API keys/secrets handled securely
- [ ] Input validation added/updated
- [ ] SQL injection prevention verified (prepared statements)

### Security Review

<!-- If there are security considerations, explain them -->

## Database Changes

<!-- If this PR affects the database -->

- [ ] No database changes
- [ ] Migration scripts added (in `migrations/`)
- [ ] Migration tested (up and down)
- [ ] Database schema documentation updated
- [ ] Backward compatible with existing data

### Migration Details

<!-- Describe database changes -->

## Configuration Changes

<!-- If this PR affects configuration -->

- [ ] No configuration changes
- [ ] Configuration options added
- [ ] Configuration options deprecated
- [ ] Default configuration updated
- [ ] Configuration documentation updated

### Configuration Updates

<!-- Describe configuration changes -->

```toml
# New/changed configuration
```

## Dependencies

<!-- If this PR adds, removes, or updates dependencies -->

- [ ] No dependency changes
- [ ] Dependencies added (justify each)
- [ ] Dependencies removed
- [ ] Dependencies updated
- [ ] `Cargo.lock` updated
- [ ] License compatibility verified

### Dependency Changes

<!-- List dependency changes with justification -->

| Dependency | Change | Version | Justification |
|------------|--------|---------|---------------|
| example-crate | Added | 1.0.0 | Needed for... |

## Compatibility

<!-- Check all that apply -->

- [ ] Tested on Linux
- [ ] Tested on macOS
- [ ] Tested on Windows
- [ ] Cross-platform considerations addressed
- [ ] Backward compatible with previous versions

## Deployment Considerations

<!-- If applicable -->

- [ ] No deployment changes required
- [ ] Environment variables added/changed
- [ ] Infrastructure changes required
- [ ] Deployment documentation updated

## Screenshots/Examples

<!-- If applicable, add screenshots or example output -->

## Additional Notes

<!-- Any other information reviewers should know -->

## Reviewer Checklist

<!-- For reviewers to complete -->

- [ ] Code review completed
- [ ] Tests reviewed and sufficient
- [ ] Documentation reviewed
- [ ] Security implications considered
- [ ] Performance implications considered
- [ ] Breaking changes justified and documented
- [ ] CHANGELOG.md entry appropriate

## Pre-Merge Checklist

<!-- Final checks before merging -->

- [ ] All CI checks passing
- [ ] All review comments addressed
- [ ] At least one approval received
- [ ] Branch is up-to-date with target branch
- [ ] Merge conflicts resolved
- [ ] Squash/rebase if needed

---

By submitting this pull request, I confirm that my contribution is made under the terms of the MIT and Apache-2.0 licenses.
