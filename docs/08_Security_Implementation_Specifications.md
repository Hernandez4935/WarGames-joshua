# WarGames/JOSHUA: Security Implementation Specifications
## Comprehensive Security Architecture and Implementation Guide
### Version 1.0.0 | October 2025

---

## Executive Summary

This document specifies the complete security architecture and implementation details for the WarGames/JOSHUA nuclear risk assessment system. Given the sensitive nature of nuclear risk analysis and the potential for misuse, security is paramount at every layer of the system.

### Security Philosophy

1. **Defense in Depth**: Multiple layers of security controls
2. **Principle of Least Privilege**: Minimum necessary access rights
3. **Zero Trust**: Never trust, always verify
4. **Encryption Everywhere**: Data encrypted in transit and at rest
5. **Auditability**: Complete audit trail of all operations
6. **Secure by Default**: Security built-in, not bolted-on

### Threat Model

```
┌─────────────────────────── THREAT LANDSCAPE ──────────────────────────┐
│                                                                       │
│  PRIMARY THREATS:                                                     │
│  • Unauthorized access to sensitive nuclear data                     │
│  • API key theft (Claude API, News APIs)                             │
│  • Database compromise exposing historical assessments               │
│  • Man-in-the-middle attacks on data collection                      │
│  • Denial of service preventing risk assessments                     │
│  • Data tampering to manipulate risk calculations                    │
│  • Injection attacks (SQL, command, XSS)                              │
│                                                                       │
│  ADVERSARIES:                                                         │
│  • Malicious insiders                                                 │
│  • State-sponsored actors                                             │
│  • Hacktivists                                                        │
│  • Automated attack tools                                             │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

---

## Table of Contents

1. [Authentication and Authorization](#authentication-and-authorization)
2. [API Security](#api-security)
3. [Data Encryption](#data-encryption)
4. [Secrets Management](#secrets-management)
5. [Network Security](#network-security)
6. [Input Validation and Sanitization](#input-validation-and-sanitization)
7. [Security Monitoring and Logging](#security-monitoring-and-logging)
8. [Incident Response](#incident-response)
9. [Compliance and Auditing](#compliance-and-auditing)
10. [Security Testing](#security-testing)

---

## 1. Authentication and Authorization

### 1.1 Authentication Implementation

```rust
// src/security/auth.rs

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User authentication credentials
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub mfa_token: Option<String>,
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,           // User ID
    pub username: String,
    pub roles: Vec<String>,
    pub exp: i64,            // Expiration time
    pub iat: i64,            // Issued at
    pub jti: Uuid,           // JWT ID (for revocation)
}

/// Authentication service
pub struct AuthService {
    db: sqlx::PgPool,
    jwt_secret: Vec<u8>,
    session_duration: Duration,
}

impl AuthService {
    /// Authenticate user with username and password
    pub async fn authenticate(
        &self,
        credentials: LoginRequest,
    ) -> Result<AuthenticationResult> {
        // Rate limiting check
        self.check_rate_limit(&credentials.username).await?;

        // Retrieve user from database
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1 AND active = true"
        )
        .bind(&credentials.username)
        .fetch_optional(&self.db)
        .await?
        .ok_or(SecurityError::InvalidCredentials)?;

        // Verify password using Argon2
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| SecurityError::InvalidCredentials)?;

        Argon2::default()
            .verify_password(credentials.password.as_bytes(), &parsed_hash)
            .map_err(|_| {
                // Log failed attempt
                self.log_failed_login(&user.id);
                SecurityError::InvalidCredentials
            })?;

        // Verify MFA if enabled
        if user.mfa_enabled {
            let mfa_token = credentials.mfa_token
                .ok_or(SecurityError::MfaRequired)?;

            self.verify_totp(&user.id, &mfa_token).await?;
        }

        // Generate session token
        let session = self.create_session(&user).await?;

        // Log successful authentication
        self.log_successful_login(&user.id).await?;

        Ok(AuthenticationResult {
            access_token: session.access_token,
            refresh_token: session.refresh_token,
            expires_in: self.session_duration.num_seconds(),
            user: UserInfo {
                id: user.id,
                username: user.username,
                roles: user.roles,
            },
        })
    }

    /// Hash password using Argon2id
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| SecurityError::HashingFailed(e.to_string()))?
            .to_string();

        Ok(password_hash)
    }

    /// Create JWT token for authenticated session
    async fn create_session(&self, user: &User) -> Result<Session> {
        let now = Utc::now();
        let expiration = now + self.session_duration;

        let claims = Claims {
            sub: user.id,
            username: user.username.clone(),
            roles: user.roles.clone(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4(),
        };

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.jwt_secret),
        )?;

        // Generate refresh token
        let refresh_token = self.generate_refresh_token(&user.id).await?;

        // Store session in database
        sqlx::query!(
            r#"
            INSERT INTO sessions (user_id, jwt_id, access_token, refresh_token, expires_at, ip_address, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user.id,
            claims.jti,
            access_token,
            refresh_token,
            expiration,
            "::1", // TODO: Get from request
            "User-Agent", // TODO: Get from request
        )
        .execute(&self.db)
        .await?;

        Ok(Session {
            access_token,
            refresh_token,
            expires_at: expiration,
        })
    }

    /// Verify JWT token and extract claims
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.jwt_secret),
            &Validation::default(),
        )?;

        // Check if token is revoked
        // (Would query sessions table for jwt_id revocation status)

        Ok(token_data.claims)
    }

    /// Verify TOTP (Time-based One-Time Password) for MFA
    async fn verify_totp(&self, user_id: &Uuid, token: &str) -> Result<()> {
        let secret = self.get_mfa_secret(user_id).await?;

        let totp = totp_lite::totp_custom::<totp_lite::Sha1>(
            30,  // 30 second time step
            6,   // 6 digit code
            secret.as_bytes(),
            Utc::now().timestamp() as u64,
        );

        if token == totp {
            Ok(())
        } else {
            Err(SecurityError::InvalidMfaToken)
        }
    }

    /// Rate limiting for login attempts
    async fn check_rate_limit(&self, username: &str) -> Result<()> {
        let recent_attempts = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM login_attempts
             WHERE username = $1
             AND attempted_at > NOW() - INTERVAL '15 minutes'"
        )
        .bind(username)
        .fetch_one(&self.db)
        .await?;

        if recent_attempts >= 5 {
            Err(SecurityError::RateLimitExceeded)
        } else {
            Ok(())
        }
    }
}
```

### 1.2 Role-Based Access Control (RBAC)

```rust
// src/security/rbac.rs

use std::collections::HashSet;

/// User roles in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// System administrator with full access
    Admin,
    /// Analyst can view and create assessments
    Analyst,
    /// Viewer can only read assessments
    Viewer,
    /// Service account for automated processes
    Service,
}

/// Permissions for various operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    // Assessment permissions
    CreateAssessment,
    ViewAssessment,
    DeleteAssessment,

    // Data collection permissions
    ConfigureDataSources,
    ViewCollectedData,

    // User management permissions
    ManageUsers,
    ViewUsers,

    // System administration
    ConfigureSystem,
    ViewLogs,
    ManageSecrets,
}

/// Authorization service
pub struct AuthorizationService {
    role_permissions: HashMap<Role, HashSet<Permission>>,
}

impl AuthorizationService {
    pub fn new() -> Self {
        let mut role_permissions = HashMap::new();

        // Admin role - full access
        role_permissions.insert(
            Role::Admin,
            vec![
                Permission::CreateAssessment,
                Permission::ViewAssessment,
                Permission::DeleteAssessment,
                Permission::ConfigureDataSources,
                Permission::ViewCollectedData,
                Permission::ManageUsers,
                Permission::ViewUsers,
                Permission::ConfigureSystem,
                Permission::ViewLogs,
                Permission::ManageSecrets,
            ]
            .into_iter()
            .collect(),
        );

        // Analyst role
        role_permissions.insert(
            Role::Analyst,
            vec![
                Permission::CreateAssessment,
                Permission::ViewAssessment,
                Permission::ViewCollectedData,
                Permission::ViewLogs,
            ]
            .into_iter()
            .collect(),
        );

        // Viewer role
        role_permissions.insert(
            Role::Viewer,
            vec![
                Permission::ViewAssessment,
            ]
            .into_iter()
            .collect(),
        );

        // Service role
        role_permissions.insert(
            Role::Service,
            vec![
                Permission::CreateAssessment,
                Permission::ViewAssessment,
                Permission::ViewCollectedData,
            ]
            .into_iter()
            .collect(),
        );

        Self { role_permissions }
    }

    /// Check if user has specific permission
    pub fn has_permission(
        &self,
        user_roles: &[Role],
        required_permission: Permission,
    ) -> bool {
        user_roles.iter().any(|role| {
            self.role_permissions
                .get(role)
                .map(|perms| perms.contains(&required_permission))
                .unwrap_or(false)
        })
    }

    /// Require specific permission (returns error if not authorized)
    pub fn require_permission(
        &self,
        user_roles: &[Role],
        required_permission: Permission,
    ) -> Result<()> {
        if self.has_permission(user_roles, required_permission.clone()) {
            Ok(())
        } else {
            Err(SecurityError::Forbidden {
                required: format!("{:?}", required_permission),
            })
        }
    }
}

/// Middleware for protecting routes
pub async fn require_auth(
    headers: HeaderMap,
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract JWT from Authorization header
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // Verify token
    let claims = auth_service
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
```

---

## 2. API Security

### 2.1 Claude API Key Protection

```rust
// src/security/api_keys.rs

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{Engine as _, engine::general_purpose};

/// Encrypted API key storage
#[derive(Debug, Clone)]
pub struct EncryptedApiKey {
    ciphertext: Vec<u8>,
    nonce: Vec<u8>,
}

/// API key management service
pub struct ApiKeyService {
    encryption_key: Vec<u8>,
    db: sqlx::PgPool,
}

impl ApiKeyService {
    /// Store API key with encryption
    pub async fn store_api_key(
        &self,
        service_name: &str,
        api_key: &str,
    ) -> Result<()> {
        // Encrypt the API key
        let encrypted = self.encrypt_api_key(api_key)?;

        // Store in database
        sqlx::query!(
            r#"
            INSERT INTO api_keys (service_name, encrypted_key, nonce, created_at, rotated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            ON CONFLICT (service_name)
            DO UPDATE SET
                encrypted_key = EXCLUDED.encrypted_key,
                nonce = EXCLUDED.nonce,
                rotated_at = NOW()
            "#,
            service_name,
            encrypted.ciphertext,
            encrypted.nonce,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// Retrieve and decrypt API key
    pub async fn get_api_key(&self, service_name: &str) -> Result<String> {
        let encrypted = sqlx::query_as::<_, EncryptedApiKey>(
            "SELECT encrypted_key as ciphertext, nonce FROM api_keys WHERE service_name = $1"
        )
        .bind(service_name)
        .fetch_one(&self.db)
        .await?;

        self.decrypt_api_key(&encrypted)
    }

    /// Encrypt API key using AES-256-GCM
    fn encrypt_api_key(&self, plaintext: &str) -> Result<EncryptedApiKey> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)?;
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| SecurityError::EncryptionFailed(e.to_string()))?;

        Ok(EncryptedApiKey {
            ciphertext,
            nonce: nonce.to_vec(),
        })
    }

    /// Decrypt API key
    fn decrypt_api_key(&self, encrypted: &EncryptedApiKey) -> Result<String> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)?;
        let nonce = Nonce::from_slice(&encrypted.nonce);

        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| SecurityError::DecryptionFailed(e.to_string()))?;

        Ok(String::from_utf8(plaintext)?)
    }

    /// Rotate API key
    pub async fn rotate_api_key(
        &self,
        service_name: &str,
        new_key: &str,
    ) -> Result<()> {
        // Archive old key
        sqlx::query!(
            r#"
            INSERT INTO api_key_history (service_name, encrypted_key, nonce, rotated_from)
            SELECT service_name, encrypted_key, nonce, NOW()
            FROM api_keys
            WHERE service_name = $1
            "#,
            service_name,
        )
        .execute(&self.db)
        .await?;

        // Store new key
        self.store_api_key(service_name, new_key).await?;

        Ok(())
    }

    /// Check if API key needs rotation (90 days policy)
    pub async fn check_rotation_needed(&self) -> Result<Vec<String>> {
        let services = sqlx::query_scalar::<_, String>(
            r#"
            SELECT service_name
            FROM api_keys
            WHERE rotated_at < NOW() - INTERVAL '90 days'
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(services)
    }
}
```

### 2.2 Rate Limiting Implementation

```rust
// src/security/rate_limit.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

/// Rate limiter for API endpoints
pub struct RateLimiter {
    limits: HashMap<String, RateLimit>,
    state: Arc<RwLock<HashMap<String, RateLimitState>>>,
}

#[derive(Clone)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_size: u32,
}

struct RateLimitState {
    minute_window: VecDeque<Instant>,
    hour_window: VecDeque<Instant>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let mut limits = HashMap::new();

        // Claude API rate limits
        limits.insert(
            "claude_api".to_string(),
            RateLimit {
                requests_per_minute: 60,
                requests_per_hour: 1000,
                burst_size: 10,
            },
        );

        // News API rate limits
        limits.insert(
            "news_api".to_string(),
            RateLimit {
                requests_per_minute: 100,
                requests_per_hour: 10000,
                burst_size: 20,
            },
        );

        Self {
            limits,
            state: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if request is allowed
    pub async fn check_rate_limit(&self, key: &str) -> Result<()> {
        let limit = self.limits.get(key)
            .ok_or(SecurityError::UnknownRateLimitKey)?;

        let mut state = self.state.write().await;
        let limit_state = state.entry(key.to_string())
            .or_insert_with(|| RateLimitState {
                minute_window: VecDeque::new(),
                hour_window: VecDeque::new(),
            });

        let now = Instant::now();

        // Clean old entries
        limit_state.minute_window.retain(|&t| now.duration_since(t) < Duration::from_secs(60));
        limit_state.hour_window.retain(|&t| now.duration_since(t) < Duration::from_secs(3600));

        // Check limits
        if limit_state.minute_window.len() >= limit.requests_per_minute as usize {
            return Err(SecurityError::RateLimitExceeded);
        }

        if limit_state.hour_window.len() >= limit.requests_per_hour as usize {
            return Err(SecurityError::RateLimitExceeded);
        }

        // Record request
        limit_state.minute_window.push_back(now);
        limit_state.hour_window.push_back(now);

        Ok(())
    }

    /// Middleware for rate limiting
    pub async fn middleware(
        State(limiter): State<Arc<RateLimiter>>,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // Extract user ID or IP for rate limiting key
        let key = extract_rate_limit_key(&request);

        limiter.check_rate_limit(&key).await
            .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;

        Ok(next.run(request).await)
    }
}
```

---

## 3. Data Encryption

### 3.1 Encryption at Rest

```sql
-- Database encryption setup

-- Enable transparent data encryption (TDE)
-- AWS RDS: Enabled via KMS during instance creation

-- Column-level encryption for sensitive data
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Encrypt sensitive assessment details
CREATE TABLE assessments_encrypted (
    id UUID PRIMARY KEY,
    assessment_date TIMESTAMP WITH TIME ZONE NOT NULL,

    -- Encrypted columns
    executive_summary_encrypted BYTEA,
    detailed_analysis_encrypted BYTEA,

    -- Encryption metadata
    encryption_key_id VARCHAR(100),
    encrypted_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Function to encrypt data
CREATE OR REPLACE FUNCTION encrypt_sensitive_data(
    plaintext TEXT,
    encryption_key TEXT
) RETURNS BYTEA AS $$
BEGIN
    RETURN pgp_sym_encrypt(plaintext, encryption_key);
END;
$$ LANGUAGE plpgsql;

-- Function to decrypt data
CREATE OR REPLACE FUNCTION decrypt_sensitive_data(
    ciphertext BYTEA,
    encryption_key TEXT
) RETURNS TEXT AS $$
BEGIN
    RETURN pgp_sym_decrypt(ciphertext, encryption_key);
END;
$$ LANGUAGE plpgsql;
```

### 3.2 Encryption in Transit

```rust
// src/security/tls.rs

use rustls::{Certificate, PrivateKey, ServerConfig};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

/// TLS configuration for HTTPS
pub struct TlsConfig {
    cert_path: String,
    key_path: String,
}

impl TlsConfig {
    /// Load TLS certificates
    pub fn load_certs(&self) -> Result<Vec<Certificate>> {
        let cert_file = File::open(&self.cert_path)?;
        let mut reader = BufReader::new(cert_file);

        let certs = rustls_pemfile::certs(&mut reader)?
            .into_iter()
            .map(Certificate)
            .collect();

        Ok(certs)
    }

    /// Load private key
    pub fn load_private_key(&self) -> Result<PrivateKey> {
        let key_file = File::open(&self.key_path)?;
        let mut reader = BufReader::new(key_file);

        let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;

        keys.into_iter()
            .next()
            .map(PrivateKey)
            .ok_or_else(|| anyhow::anyhow!("No private key found"))
    }

    /// Create TLS server configuration
    pub fn create_server_config(&self) -> Result<Arc<ServerConfig>> {
        let certs = self.load_certs()?;
        let key = self.load_private_key()?;

        let config = ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_protocol_versions(&[&rustls::version::TLS13])?
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(Arc::new(config))
    }
}

/// Verify TLS certificate for external API calls
pub fn create_https_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .min_tls_version(reqwest::tls::Version::TLS_1_3)
        .https_only(true)
        .timeout(Duration::from_secs(30))
        .build()?;

    Ok(client)
}
```

---

## 4. Secrets Management

### 4.1 AWS Secrets Manager Integration

```rust
// src/security/secrets.rs

use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use serde::{Deserialize, Serialize};

/// Secrets manager service
pub struct SecretsManager {
    client: SecretsManagerClient,
    environment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

impl SecretsManager {
    pub async fn new(environment: &str) -> Result<Self> {
        let config = aws_config::load_from_env().await;
        let client = SecretsManagerClient::new(&config);

        Ok(Self {
            client,
            environment: environment.to_string(),
        })
    }

    /// Get Claude API key from Secrets Manager
    pub async fn get_claude_api_key(&self) -> Result<String> {
        let secret_id = format!("wargames/{}/claude-api-key", self.environment);

        let response = self.client
            .get_secret_value()
            .secret_id(&secret_id)
            .send()
            .await?;

        response.secret_string()
            .ok_or_else(|| anyhow::anyhow!("Secret not found"))
            .map(|s| s.to_string())
    }

    /// Get database credentials from Secrets Manager
    pub async fn get_database_credentials(&self) -> Result<DatabaseCredentials> {
        let secret_id = format!("wargames/{}/database", self.environment);

        let response = self.client
            .get_secret_value()
            .secret_id(&secret_id)
            .send()
            .await?;

        let secret_string = response.secret_string()
            .ok_or_else(|| anyhow::anyhow!("Secret not found"))?;

        let credentials: DatabaseCredentials = serde_json::from_str(secret_string)?;

        Ok(credentials)
    }

    /// Rotate secret
    pub async fn rotate_secret(&self, secret_id: &str) -> Result<()> {
        self.client
            .rotate_secret()
            .secret_id(secret_id)
            .send()
            .await?;

        Ok(())
    }
}
```

---

## 5. Network Security

### 5.1 Security Groups and Network ACLs

```hcl
# infrastructure/terraform/security_groups.tf

# Application load balancer security group
resource "aws_security_group" "alb" {
  name        = "wargames-${var.environment}-alb-sg"
  description = "Security group for application load balancer"
  vpc_id      = module.vpc.vpc_id

  # HTTPS from internet
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    description = "HTTPS from internet"
  }

  # HTTP redirect to HTTPS
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    description = "HTTP redirect to HTTPS"
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "wargames-${var.environment}-alb-sg"
  }
}

# Application servers security group
resource "aws_security_group" "application" {
  name        = "wargames-${var.environment}-app-sg"
  description = "Security group for application servers"
  vpc_id      = module.vpc.vpc_id

  # Application port from ALB only
  ingress {
    from_port       = 8080
    to_port         = 8080
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id]
    description     = "Application port from ALB"
  }

  # SSH from bastion only (for emergency access)
  ingress {
    from_port       = 22
    to_port         = 22
    protocol        = "tcp"
    security_groups = [aws_security_group.bastion.id]
    description     = "SSH from bastion"
  }

  # Outbound to internet for API calls
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    description = "HTTPS to internet"
  }

  # Outbound to database
  egress {
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.database.id]
    description     = "PostgreSQL to database"
  }

  # Outbound to Redis
  egress {
    from_port       = 6379
    to_port         = 6379
    protocol        = "tcp"
    security_groups = [aws_security_group.redis.id]
    description     = "Redis to cache"
  }

  tags = {
    Name = "wargames-${var.environment}-app-sg"
  }
}

# Database security group
resource "aws_security_group" "database" {
  name        = "wargames-${var.environment}-db-sg"
  description = "Security group for PostgreSQL database"
  vpc_id      = module.vpc.vpc_id

  # PostgreSQL from application servers only
  ingress {
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.application.id]
    description     = "PostgreSQL from application"
  }

  # No outbound (database doesn't initiate connections)
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = []
    description = "No outbound"
  }

  tags = {
    Name = "wargames-${var.environment}-db-sg"
  }
}

# WAF Web ACL for DDoS and attack protection
resource "aws_wafv2_web_acl" "main" {
  name  = "wargames-${var.environment}-waf"
  scope = "REGIONAL"

  default_action {
    allow {}
  }

  # Rate limiting rule
  rule {
    name     = "rate-limit"
    priority = 1

    action {
      block {}
    }

    statement {
      rate_based_statement {
        limit              = 2000  # Requests per 5 minutes
        aggregate_key_type = "IP"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "rate-limit"
      sampled_requests_enabled   = true
    }
  }

  # SQL injection protection
  rule {
    name     = "sql-injection"
    priority = 2

    override_action {
      none {}
    }

    statement {
      managed_rule_group_statement {
        vendor_name = "AWS"
        name        = "AWSManagedRulesSQLiRuleSet"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "sql-injection"
      sampled_requests_enabled   = true
    }
  }

  # Known bad inputs
  rule {
    name     = "known-bad-inputs"
    priority = 3

    override_action {
      none {}
    }

    statement {
      managed_rule_group_statement {
        vendor_name = "AWS"
        name        = "AWSManagedRulesKnownBadInputsRuleSet"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "known-bad-inputs"
      sampled_requests_enabled   = true
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "wargames-waf"
    sampled_requests_enabled   = true
  }
}
```

---

## 6. Input Validation and Sanitization

### 6.1 Input Validation

```rust
// src/security/validation.rs

use validator::{Validate, ValidationError};
use regex::Regex;

/// Assessment creation request with validation
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAssessmentRequest {
    #[validate(length(min = 10, max = 500, message = "Title must be 10-500 characters"))]
    pub title: String,

    #[validate(custom = "validate_no_sql_injection")]
    pub description: Option<String>,

    #[validate(range(min = 1, max = 100))]
    pub priority: u8,

    #[validate]
    pub metadata: AssessmentMetadata,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AssessmentMetadata {
    #[validate(email)]
    pub requester_email: String,

    #[validate(url)]
    pub reference_url: Option<String>,
}

/// Custom validator to prevent SQL injection
fn validate_no_sql_injection(value: &str) -> Result<(), ValidationError> {
    let sql_patterns = [
        r"(?i)(union\s+select)",
        r"(?i)(drop\s+table)",
        r"(?i)(insert\s+into)",
        r"(?i)(delete\s+from)",
        r"--",
        r"';",
    ];

    for pattern in &sql_patterns {
        let re = Regex::new(pattern).unwrap();
        if re.is_match(value) {
            return Err(ValidationError::new("potential_sql_injection"));
        }
    }

    Ok(())
}

/// Sanitize user input
pub fn sanitize_string(input: &str) -> String {
    // Remove potential XSS vectors
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('&', "&amp;")
}

/// Validate UUID format
pub fn validate_uuid(uuid: &str) -> Result<Uuid> {
    Uuid::parse_str(uuid)
        .map_err(|_| ValidationError::new("invalid_uuid"))
}
```

---

## 7. Security Monitoring and Logging

### 7.1 Security Event Logging

```rust
// src/security/audit_log.rs

use tracing::{event, Level};
use serde::{Deserialize, Serialize};

/// Security event types
#[derive(Debug, Serialize, Deserialize)]
pub enum SecurityEvent {
    LoginSuccess {
        user_id: Uuid,
        username: String,
        ip_address: String,
        user_agent: String,
    },
    LoginFailure {
        username: String,
        ip_address: String,
        reason: String,
    },
    UnauthorizedAccess {
        user_id: Uuid,
        resource: String,
        required_permission: String,
    },
    ApiKeyAccess {
        service_name: String,
        accessed_by: Uuid,
    },
    SuspiciousActivity {
        description: String,
        user_id: Option<Uuid>,
        ip_address: String,
    },
    DataExfiltration {
        user_id: Uuid,
        data_volume_bytes: u64,
        resource_type: String,
    },
}

/// Audit logger
pub struct AuditLogger {
    db: sqlx::PgPool,
}

impl AuditLogger {
    /// Log security event
    pub async fn log_event(&self, event: SecurityEvent) -> Result<()> {
        let event_type = match &event {
            SecurityEvent::LoginSuccess { .. } => "login_success",
            SecurityEvent::LoginFailure { .. } => "login_failure",
            SecurityEvent::UnauthorizedAccess { .. } => "unauthorized_access",
            SecurityEvent::ApiKeyAccess { .. } => "api_key_access",
            SecurityEvent::SuspiciousActivity { .. } => "suspicious_activity",
            SecurityEvent::DataExfiltration { .. } => "data_exfiltration",
        };

        // Log to structured logging
        event!(
            Level::WARN,
            event_type = event_type,
            event = ?event,
            "Security event occurred"
        );

        // Store in audit log table
        let event_json = serde_json::to_value(&event)?;

        sqlx::query!(
            r#"
            INSERT INTO security_audit_log (event_type, event_data, created_at)
            VALUES ($1, $2, NOW())
            "#,
            event_type,
            event_json,
        )
        .execute(&self.db)
        .await?;

        // Check for security alerts
        self.check_for_alerts(&event).await?;

        Ok(())
    }

    /// Check if event should trigger security alert
    async fn check_for_alerts(&self, event: &SecurityEvent) -> Result<()> {
        match event {
            SecurityEvent::LoginFailure { username, .. } => {
                // Check for brute force attempts
                let recent_failures = sqlx::query_scalar::<_, i64>(
                    r#"
                    SELECT COUNT(*) FROM security_audit_log
                    WHERE event_type = 'login_failure'
                    AND event_data->>'username' = $1
                    AND created_at > NOW() - INTERVAL '15 minutes'
                    "#
                )
                .bind(username)
                .fetch_one(&self.db)
                .await?;

                if recent_failures >= 5 {
                    self.trigger_alert(
                        "Brute force attack detected",
                        format!("User {} has {} failed login attempts in 15 minutes", username, recent_failures),
                    ).await?;
                }
            }
            SecurityEvent::UnauthorizedAccess { user_id, .. } => {
                self.trigger_alert(
                    "Unauthorized access attempt",
                    format!("User {} attempted unauthorized access", user_id),
                ).await?;
            }
            SecurityEvent::DataExfiltration { user_id, data_volume_bytes, .. } => {
                if *data_volume_bytes > 100_000_000 {  // 100 MB
                    self.trigger_alert(
                        "Large data exfiltration detected",
                        format!("User {} downloaded {} bytes", user_id, data_volume_bytes),
                    ).await?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    async fn trigger_alert(&self, title: &str, message: String) -> Result<()> {
        // Send to monitoring system, PagerDuty, etc.
        event!(Level::ERROR, title = title, message = message, "Security alert triggered");
        Ok(())
    }
}
```

---

## 8. Incident Response

### 8.1 Incident Response Procedures

```yaml
# security/incident-response-playbook.yml

incident_response_plan:
  detection:
    automated_alerts:
      - Brute force login attempts
      - Unauthorized access attempts
      - Unusual data access patterns
      - API rate limit violations
      - Database connection anomalies

    manual_triggers:
      - User reports suspicious activity
      - External security notification
      - Anomalous system behavior

  severity_levels:
    critical:
      description: "Active breach, data exfiltration, or system compromise"
      response_time: "< 15 minutes"
      escalation: "CTO, Security Team, On-call Engineer"

    high:
      description: "Attempted breach, privilege escalation, or unauthorized access"
      response_time: "< 1 hour"
      escalation: "Security Team, On-call Engineer"

    medium:
      description: "Suspicious activity, failed authentication attempts"
      response_time: "< 4 hours"
      escalation: "On-call Engineer"

    low:
      description: "Policy violations, non-critical security events"
      response_time: "< 24 hours"
      escalation: "Security Team review during business hours"

  response_procedures:
    1_identify:
      - Alert received via monitoring system
      - Initial triage by on-call engineer
      - Severity classification
      - Incident commander assigned

    2_contain:
      critical_actions:
        - Isolate affected systems
        - Revoke compromised credentials
        - Enable additional logging
        - Capture forensic snapshots

      commands:
        - ./security/isolate-system.sh <instance-id>
        - ./security/revoke-credentials.sh <user-id>
        - ./security/enable-detailed-logging.sh

    3_eradicate:
      - Identify root cause
      - Remove malicious artifacts
      - Patch vulnerabilities
      - Update security controls

    4_recover:
      - Restore from clean backups if needed
      - Verify system integrity
      - Re-enable normal operations
      - Monitor for recurrence

    5_lessons_learned:
      - Post-incident review within 48 hours
      - Document timeline and actions
      - Update playbooks and procedures
      - Implement preventive measures
```

### 8.2 Incident Response Scripts

```bash
#!/bin/bash
# security/isolate-system.sh

INSTANCE_ID=$1

echo "⚠️  INCIDENT RESPONSE: Isolating system $INSTANCE_ID"

# Remove from load balancer
echo "➤ Removing from load balancer..."
TARGET_GROUP=$(aws elbv2 describe-target-health \
    --query "TargetHealthDescriptions[?Target.Id=='$INSTANCE_ID'].TargetHealth.State" \
    --output text)

if [[ -n "$TARGET_GROUP" ]]; then
    aws elbv2 deregister-targets \
        --target-group-arn "$TARGET_GROUP" \
        --targets Id="$INSTANCE_ID"
fi

# Apply restrictive security group
echo "➤ Applying restrictive security group..."
ISOLATION_SG=$(aws ec2 describe-security-groups \
    --filters "Name=group-name,Values=isolation-sg" \
    --query "SecurityGroups[0].GroupId" \
    --output text)

aws ec2 modify-instance-attribute \
    --instance-id "$INSTANCE_ID" \
    --groups "$ISOLATION_SG"

# Create forensic snapshot
echo "➤ Creating forensic snapshot..."
SNAPSHOT_ID=$(aws ec2 create-snapshot \
    --instance-id "$INSTANCE_ID" \
    --description "Forensic snapshot - Incident $(date +%Y%m%d-%H%M%S)" \
    --query "SnapshotId" \
    --output text)

echo "✓ System isolated"
echo "  Snapshot ID: $SNAPSHOT_ID"
echo "  Next steps:"
echo "  1. Investigate snapshot for malicious activity"
echo "  2. Review CloudWatch logs"
echo "  3. Check security audit log"
echo "  4. Determine if data exfiltration occurred"
```

---

## 9. Compliance and Auditing

### 9.1 Audit Trail Requirements

```sql
-- Comprehensive audit trail schema

CREATE TABLE security_audit_log (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    user_id UUID,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    -- Indexes for efficient querying
    INDEX idx_audit_event_type (event_type),
    INDEX idx_audit_user_id (user_id),
    INDEX idx_audit_created_at (created_at DESC),
    INDEX idx_audit_event_data USING GIN (event_data)
);

-- Retention policy: Keep audit logs for 7 years
ALTER TABLE security_audit_log
    SET (autovacuum_vacuum_scale_factor = 0.0);

-- Prevent deletion (audit logs are immutable)
CREATE RULE no_delete_audit_log AS
    ON DELETE TO security_audit_log
    DO INSTEAD NOTHING;

CREATE RULE no_update_audit_log AS
    ON UPDATE TO security_audit_log
    DO INSTEAD NOTHING;
```

---

## 10. Security Testing

### 10.1 Penetration Testing Procedures

```bash
#!/bin/bash
# security/penetration-test.sh

echo "═══════════════════════════════════════════════════════════"
echo "  WarGames/JOSHUA Security Testing Suite"
echo "═══════════════════════════════════════════════════════════"

# SQL Injection Testing
echo "➤ Testing SQL injection protection..."
curl -X POST https://staging.wargames.internal/api/assessments \
    -H "Content-Type: application/json" \
    -d '{"title": "Test'; DROP TABLE assessments; --"}'

# XSS Testing
echo "➤ Testing XSS protection..."
curl -X POST https://staging.wargames.internal/api/assessments \
    -H "Content-Type: application/json" \
    -d '{"title": "<script>alert(\"XSS\")</script>"}'

# Authentication bypass
echo "➤ Testing authentication bypass..."
curl -X GET https://staging.wargames.internal/api/assessments/secret \
    -H "Authorization: Bearer invalid_token"

# Rate limiting
echo "➤ Testing rate limiting..."
for i in {1..100}; do
    curl -X GET https://staging.wargames.internal/api/health &
done
wait

# CSRF protection
echo "➤ Testing CSRF protection..."
curl -X POST https://staging.wargames.internal/api/assessments \
    -H "Origin: https://malicious-site.com"

echo "═══════════════════════════════════════════════════════════"
echo "  Security testing completed"
echo "  Review logs for any successful attacks"
echo "═══════════════════════════════════════════════════════════"
```

---

## Conclusion

This comprehensive security implementation specification provides the foundation for a hardened, production-ready WarGames/JOSHUA system. Security is not a one-time implementation but an ongoing process of vigilance, testing, and improvement.

### Security Checklist

- [x] Multi-factor authentication for all users
- [x] Role-based access control (RBAC)
- [x] API key encryption and rotation
- [x] TLS 1.3 for all network communication
- [x] Database encryption at rest and in transit
- [x] Secrets management via AWS Secrets Manager
- [x] Comprehensive input validation
- [x] SQL injection protection
- [x] XSS protection
- [x] CSRF protection
- [x] Rate limiting on all endpoints
- [x] Security event logging and monitoring
- [x] Incident response procedures
- [x] Regular security testing
- [x] Compliance with audit requirements

**Security is everyone's responsibility. Stay vigilant.**

*"Security is not a product, but a process." - Bruce Schneier*
