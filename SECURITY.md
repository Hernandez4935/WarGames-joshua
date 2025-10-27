# Security Policy

## Overview

The WarGames/JOSHUA nuclear risk assessment system handles sensitive operations including API communications, data collection from external sources, and risk calculations. This document outlines our security policies and procedures for reporting vulnerabilities.

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          | Status                    |
| ------- | ------------------ | ------------------------- |
| 0.1.x   | :white_check_mark: | Current development       |
| < 0.1.0 | :x:                | Planning phase only       |

**Note**: As this project is in active development (Phase 0 complete, Phase 1 in progress), security patches will be applied to the current development branch and included in the next release.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them responsibly using one of the following methods:

### Preferred Method: Private Security Advisory

1. Go to the [Security tab](../../security) of this repository
2. Click "Report a vulnerability"
3. Fill out the form with detailed information
4. Submit the report

GitHub will notify the maintainers privately, and we can coordinate a fix before public disclosure.

### Alternative Method: Direct Contact

If you prefer, you can report vulnerabilities via:

- **Email**: Security reports can be sent to project maintainers (configure when project is public)
- **Encrypted Communication**: PGP key available (coming soon)

### What to Include in Your Report

Please include as much of the following information as possible:

1. **Vulnerability Type**
   - Authentication bypass
   - SQL injection
   - API key exposure
   - Code injection
   - Denial of Service
   - Other (please describe)

2. **Affected Component**
   - Data collection engine
   - Claude API integration
   - Database layer
   - Configuration system
   - CLI interface
   - Other module

3. **Vulnerability Details**
   - Description of the vulnerability
   - Steps to reproduce
   - Proof of concept (if applicable)
   - Potential impact assessment

4. **Affected Versions**
   - Version(s) where the vulnerability exists
   - Version(s) tested (if known)

5. **Suggested Fix** (optional)
   - Proposed solution or mitigation
   - Code patches (if you've developed a fix)

6. **Your Information**
   - Name (or handle)
   - Contact information
   - Whether you'd like to be credited in the security advisory

## Response Timeline

We are committed to responding to security reports promptly:

| Timeline | Action                                           |
| -------- | ------------------------------------------------ |
| 24 hours | Initial acknowledgment of report                 |
| 72 hours | Preliminary assessment and severity rating       |
| 7 days   | Detailed investigation and fix development       |
| 30 days  | Patch release and public disclosure (if needed)  |

**Note**: Complex vulnerabilities may require more time. We will keep you informed of our progress throughout the process.

## Security Update Process

When a security vulnerability is confirmed:

1. **Fix Development**: We develop and test a fix on a private branch
2. **CVE Assignment**: We request a CVE identifier for tracking (if applicable)
3. **Security Advisory**: We prepare a security advisory with details
4. **Patch Release**: We release a patch version (e.g., 0.1.1 → 0.1.2)
5. **Public Disclosure**: We publish the security advisory
6. **Credit**: We credit the reporter (unless anonymity is requested)

## Disclosure Policy

We follow **coordinated disclosure**:

- We will work with you to understand and fix the vulnerability
- We request 90 days before public disclosure to develop and release a fix
- We will credit you in the security advisory (if desired)
- If you publish before we've released a fix, we ask that you:
  - Give us reasonable notice
  - Include mitigation recommendations
  - Avoid exploiting the vulnerability maliciously

## Security Considerations in Development

### API Keys and Secrets

- **Never commit API keys** to version control
- Use environment variables: `ANTHROPIC_API_KEY`, `DATABASE_URL`, etc.
- Configuration file `config/local_config.toml` is in `.gitignore`
- Use AWS Secrets Manager or similar for production deployments
- API keys are encrypted at rest (Phase 6 implementation)

### Database Security

- **SQL Injection Prevention**: All queries use SQLx prepared statements
- **Connection Security**: TLS required for production database connections
- **Authentication**: Strong passwords and credential rotation policies
- **Access Control**: Principle of least privilege for database users
- **Audit Logging**: All database modifications logged

### External API Communications

- **TLS 1.3**: All external HTTP requests use TLS
- **Certificate Validation**: Certificate pinning for critical APIs (Phase 2+)
- **Rate Limiting**: Implemented to prevent abuse and DoS
- **Timeout Configuration**: Reasonable timeouts prevent hanging requests
- **Input Validation**: All external data validated before processing

### Data Collection Security

- **Source Validation**: Only trusted sources configured
- **Content Sanitization**: All collected data sanitized before storage
- **Malicious Content Detection**: Scanning for suspicious patterns
- **Quota Enforcement**: Limits on data collection volume

### Authentication and Authorization

- **No Authentication in CLI**: Current version is single-user CLI tool
- **Future Web API**: Will implement JWT-based authentication (Phase 5+)
- **Role-Based Access Control**: Planned for multi-user scenarios
- **Audit Logging**: All security-relevant actions logged

### Input Validation

- **CLI Arguments**: Validated with clap constraints
- **Configuration Files**: Validated with structured parsing (TOML)
- **Database Inputs**: Type-checked and sanitized
- **API Responses**: Schema validation before processing

### Cryptography

- **Hashing**: Using `argon2` for password hashing (if needed)
- **Encryption**: AES-256-GCM for sensitive data at rest
- **Random Number Generation**: Using cryptographically secure RNGs
- **No Custom Crypto**: Using well-vetted libraries only

## Known Security Considerations

### Current Phase (Phase 0-1)

- **Development Status**: Project is in active development
- **API Key Storage**: Currently uses environment variables (encryption planned for Phase 6)
- **No Authentication**: CLI tool has no authentication (by design for single-user)
- **Testing Coverage**: Security testing framework being developed

### Planned Security Enhancements (Phase 6)

- **API Key Encryption**: Encryption at rest for stored credentials
- **Security Audit**: Professional security audit before v1.0
- **Penetration Testing**: Third-party penetration testing
- **OWASP Compliance**: Alignment with OWASP security guidelines
- **Security Scanning**: Automated dependency vulnerability scanning
- **Secrets Management**: Integration with enterprise secrets management

## Security Best Practices for Users

### Running the Application

1. **Use Environment Variables** for sensitive configuration:
   ```bash
   export ANTHROPIC_API_KEY="your-key-here"
   export DATABASE_URL="postgresql://..."
   ```

2. **Protect Configuration Files**:
   ```bash
   chmod 600 config/local_config.toml
   ```

3. **Keep Dependencies Updated**:
   ```bash
   cargo update
   cargo audit
   ```

4. **Run in Restricted Environments**:
   - Use containers or VMs for isolation
   - Apply principle of least privilege
   - Monitor system resources

5. **Review Logs Regularly**:
   - Check for suspicious activity
   - Monitor API usage
   - Track database access patterns

### Development Security

1. **Install Security Tools**:
   ```bash
   cargo install cargo-audit
   cargo install cargo-deny
   ```

2. **Run Security Checks**:
   ```bash
   cargo audit              # Check for vulnerable dependencies
   cargo deny check         # Check for licensing and security issues
   cargo clippy -- -W clippy::all  # Lint for common issues
   ```

3. **Pre-Commit Hooks**:
   - Run security scans before commits
   - Prevent committing secrets
   - Validate configuration files

## Security-Related Configuration

### Recommended Settings

```toml
[general]
# Enable audit logging
audit_logging = true

[claude_api]
# Use reasonable token limits
max_tokens = 8000
timeout_seconds = 120

[data_collection]
# Limit concurrent requests
max_concurrent_requests = 5
request_timeout_seconds = 30

[database]
# Use connection pooling limits
max_connections = 10
min_connections = 2

[logging]
# Log security events
level = "info"
log_security_events = true
```

### Environment-Specific Settings

- **Development**: Verbose logging, relaxed timeouts
- **Production**: Minimal logging, strict timeouts, encryption enabled

## Compliance and Standards

This project aims to comply with:

- **OWASP Top 10**: Mitigation of common web application vulnerabilities
- **CWE Top 25**: Common Weakness Enumeration considerations
- **Rust Security Guidelines**: Following official Rust security best practices
- **CVE Program**: Participating in CVE assignment for vulnerabilities

## Dependencies and Supply Chain Security

### Dependency Management

- **Regular Updates**: Dependencies updated regularly
- **Security Audits**: Using `cargo-audit` in CI/CD
- **Version Pinning**: Critical dependencies pinned to specific versions
- **Source Verification**: Using crates.io official registry only

### Current Dependencies

Key security-relevant dependencies:

- **tokio**: Async runtime (widely vetted)
- **sqlx**: Database library with compile-time verification
- **reqwest**: HTTP client with TLS support
- **clap**: CLI parsing with input validation
- **thiserror**: Error handling
- **argon2**: Password hashing
- **sha2**: Cryptographic hashing

### Monitoring

- Dependabot alerts enabled
- Weekly `cargo audit` runs
- Automatic PR creation for security updates

## Security-Related Features

### Implemented (Phase 0-1)

- ✅ Structured error handling
- ✅ Input validation framework
- ✅ SQL injection prevention (prepared statements)
- ✅ TLS for external communications
- ✅ Rate limiting infrastructure
- ✅ Audit logging framework

### In Progress (Phase 2-3)

- 🔄 API key encryption at rest
- 🔄 Enhanced input sanitization
- 🔄 Comprehensive audit logging

### Planned (Phase 4-6)

- ⏳ Professional security audit
- ⏳ Penetration testing
- ⏳ Security monitoring and alerting
- ⏳ Incident response procedures
- ⏳ Security documentation for deployment

## Contact

For non-security related questions, please use:
- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For general questions

For security-related matters, please use the private reporting methods described above.

## Acknowledgments

We appreciate the security research community's efforts in responsibly disclosing vulnerabilities. Contributors who report valid security issues will be:

- Credited in security advisories (if desired)
- Listed in our security hall of fame
- Thanked in release notes

---

**Remember**: "The only winning move is not to play." In security, the only winning move is to play defensively.

**Last Updated**: 2025-10-27
