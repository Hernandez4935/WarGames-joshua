# WarGames/JOSHUA: Deployment Guide
## Version 1.0.0 | November 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Deployment Options](#deployment-options)
3. [Docker Deployment](#docker-deployment)
4. [Manual Deployment](#manual-deployment)
5. [Database Setup](#database-setup)
6. [Configuration](#configuration)
7. [Security Hardening](#security-hardening)
8. [Monitoring Setup](#monitoring-setup)
9. [Post-Deployment Verification](#post-deployment-verification)

---

## Overview

This guide covers deploying the WarGames/JOSHUA nuclear risk assessment system to production environments.

### System Requirements

**Minimum**:
- CPU: 2 cores
- RAM: 2GB
- Disk: 10GB
- OS: Linux (Ubuntu 20.04+, Debian 11+, or RHEL 8+)

**Recommended**:
- CPU: 4 cores
- RAM: 4GB
- Disk: 50GB SSD
- OS: Ubuntu 22.04 LTS

### Prerequisites

- Docker 20.10+ and Docker Compose 2.0+ (for Docker deployment)
- PostgreSQL 14+ (for manual deployment)
- Anthropic Claude API key
- SSL/TLS certificates (for production)

---

## Deployment Options

### Option 1: Docker Deployment (Recommended)

**Pros**:
- Isolated environment
- Easy scaling
- Reproducible deployments
- Built-in health checks

**Cons**:
- Requires Docker knowledge
- Slightly higher resource usage

### Option 2: Manual Deployment

**Pros**:
- Direct system access
- Lower overhead
- Fine-grained control

**Cons**:
- More complex setup
- Manual dependency management
- Harder to reproduce

---

## Docker Deployment

### Step 1: Clone Repository

```bash
git clone https://github.com/yourusername/wargames-joshua.git
cd wargames-joshua
```

### Step 2: Configure Environment

```bash
# Create configuration directory
mkdir -p config output logs

# Copy and edit configuration
cp config.example.toml config/config.toml
nano config/config.toml
```

**Important**: Set your Claude API key in `config/config.toml`:

```toml
[api]
anthropic_api_key = "sk-ant-your-actual-key-here"
```

### Step 3: Review docker-compose.yml

```bash
# Edit if needed
nano docker-compose.yml

# Update PostgreSQL password for production!
# Change: POSTGRES_PASSWORD=joshua_password
```

### Step 4: Build and Start

```bash
# Build the image
docker-compose build

# Start services
docker-compose up -d

# View logs
docker-compose logs -f joshua
```

### Step 5: Verify Deployment

```bash
# Run system diagnostic
docker-compose exec joshua /app/joshua diagnose

# Run test assessment
docker-compose exec joshua /app/joshua assess --force

# Check logs
docker-compose logs --tail=100 joshua
```

### Docker Management Commands

```bash
# Stop services
docker-compose down

# Restart services
docker-compose restart

# View resource usage
docker stats wargames-joshua

# Access container shell
docker-compose exec joshua /bin/bash

# View PostgreSQL data
docker-compose exec postgres psql -U joshua -d joshua
```

---

## Manual Deployment

### Step 1: Install Dependencies

**Ubuntu/Debian**:
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install build dependencies
sudo apt install -y build-essential pkg-config libssl-dev libsqlite3-dev

# Install PostgreSQL
sudo apt install -y postgresql postgresql-contrib

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**RHEL/CentOS**:
```bash
# Update system
sudo dnf update -y

# Install dependencies
sudo dnf install -y gcc pkg-config openssl-devel sqlite-devel

# Install PostgreSQL
sudo dnf install -y postgresql-server postgresql-contrib
sudo postgresql-setup --initdb
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 2: Set Up Database

```bash
# Switch to postgres user
sudo -u postgres psql

# Create database and user
CREATE DATABASE joshua;
CREATE USER joshua WITH PASSWORD 'secure_password_here';
GRANT ALL PRIVILEGES ON DATABASE joshua TO joshua;
\q
```

### Step 3: Build Application

```bash
# Clone repository
git clone https://github.com/yourusername/wargames-joshua.git
cd wargames-joshua

# Build release version
cargo build --release

# Install binary
sudo cp target/release/joshua /usr/local/bin/
sudo chmod +x /usr/local/bin/joshua
```

### Step 4: Configure Application

```bash
# Create directories
sudo mkdir -p /etc/wargames-joshua
sudo mkdir -p /var/lib/wargames-joshua
sudo mkdir -p /var/log/wargames-joshua

# Copy configuration
sudo cp config.example.toml /etc/wargames-joshua/config.toml

# Edit configuration
sudo nano /etc/wargames-joshua/config.toml
```

### Step 5: Create Systemd Service

Create `/etc/systemd/system/wargames-joshua.service`:

```ini
[Unit]
Description=WarGames/JOSHUA Nuclear Risk Assessment System
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=joshua
Group=joshua
WorkingDirectory=/var/lib/wargames-joshua
Environment=WARGAMES_CONFIG=/etc/wargames-joshua/config.toml
Environment=RUST_LOG=info
ExecStart=/usr/local/bin/joshua schedule --daemon
Restart=on-failure
RestartSec=10s

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/wargames-joshua /var/log/wargames-joshua

[Install]
WantedBy=multi-user.target
```

### Step 6: Start Service

```bash
# Create user
sudo useradd -r -s /bin/false joshua

# Set permissions
sudo chown -R joshua:joshua /var/lib/wargames-joshua
sudo chown -R joshua:joshua /var/log/wargames-joshua

# Reload systemd
sudo systemctl daemon-reload

# Start service
sudo systemctl start wargames-joshua
sudo systemctl enable wargames-joshua

# Check status
sudo systemctl status wargames-joshua
```

---

## Database Setup

### PostgreSQL Configuration

**Edit `/etc/postgresql/14/main/postgresql.conf`**:

```conf
# Performance tuning
shared_buffers = 256MB
effective_cache_size = 1GB
maintenance_work_mem = 64MB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
work_mem = 8MB
min_wal_size = 1GB
max_wal_size = 4GB

# Connection settings
max_connections = 100
```

**Edit `/etc/postgresql/14/main/pg_hba.conf`**:

```conf
# TYPE  DATABASE        USER            ADDRESS                 METHOD
local   joshua          joshua                                  scram-sha-256
host    joshua          joshua          127.0.0.1/32            scram-sha-256
host    joshua          joshua          ::1/128                 scram-sha-256
```

**Restart PostgreSQL**:
```bash
sudo systemctl restart postgresql
```

### Run Migrations

```bash
# From application directory
sqlx database create
sqlx migrate run
```

---

## Configuration

### Critical Configuration Items

1. **API Key** (REQUIRED):
   ```toml
   [api]
   anthropic_api_key = "sk-ant-your-key"
   ```

2. **Database Connection** (REQUIRED):
   ```toml
   [database]
   url = "postgresql://joshua:password@localhost:5432/joshua"
   ```

3. **Security** (REQUIRED):
   ```toml
   [security]
   encrypt_api_keys = true
   audit_logging = true
   ```

4. **Output Directories**:
   ```toml
   [output]
   output_dir = "/var/lib/wargames-joshua/output"
   log_dir = "/var/log/wargames-joshua"
   ```

### Environment Variables

```bash
# Set in shell or systemd service file
export WARGAMES_CONFIG=/etc/wargames-joshua/config.toml
export RUST_LOG=info
export DATABASE_URL=postgresql://joshua:password@localhost/joshua
```

---

## Security Hardening

### 1. API Key Protection

```bash
# Encrypt API key
joshua configure --api-key "sk-ant-..." --encrypt

# Verify encryption
ls -la ~/.config/wargames-joshua/.keyring
# Should show: -rw------- (600 permissions)
```

### 2. File Permissions

```bash
# Configuration files
sudo chmod 600 /etc/wargames-joshua/config.toml
sudo chown joshua:joshua /etc/wargames-joshua/config.toml

# Data directory
sudo chmod 700 /var/lib/wargames-joshua
sudo chown joshua:joshua /var/lib/wargames-joshua

# Log directory
sudo chmod 750 /var/log/wargames-joshua
sudo chown joshua:joshua /var/log/wargames-joshua
```

### 3. PostgreSQL Security

```bash
# Set strong password
sudo -u postgres psql
ALTER USER joshua WITH PASSWORD 'strong_random_password_here';

# Limit connections
ALTER USER joshua CONNECTION LIMIT 20;
```

### 4. Firewall Configuration

```bash
# Allow only necessary ports
sudo ufw allow 22/tcp    # SSH
sudo ufw enable

# PostgreSQL should NOT be exposed to internet
# Only allow local connections
```

### 5. SSL/TLS (if exposing API)

```bash
# Generate self-signed cert (development)
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Use Let's Encrypt (production)
sudo apt install certbot
sudo certbot certonly --standalone -d your-domain.com
```

---

## Monitoring Setup

### 1. Log Monitoring

```bash
# Set up logrotate
sudo nano /etc/logrotate.d/wargames-joshua
```

```conf
/var/log/wargames-joshua/*.log {
    daily
    rotate 30
    compress
    delaycompress
    notifempty
    create 640 joshua joshua
    sharedscripts
    postrotate
        systemctl reload wargames-joshua > /dev/null 2>&1 || true
    endscript
}
```

### 2. System Monitoring

```bash
# Install monitoring tools
sudo apt install -y prometheus-node-exporter

# Monitor with systemd
journalctl -u wargames-joshua -f

# Monitor resource usage
htop
```

### 3. Health Checks

```bash
# Add to cron for automated health checks
crontab -e
```

```cron
# Health check every 5 minutes
*/5 * * * * /usr/local/bin/joshua diagnose >> /var/log/wargames-joshua/health.log 2>&1
```

---

## Post-Deployment Verification

### Verification Checklist

```bash
# 1. System diagnostic
joshua diagnose

# 2. Run test assessment
joshua assess --force

# 3. Check database connectivity
psql -U joshua -d joshua -c "SELECT COUNT(*) FROM assessments;"

# 4. Verify API key encryption
joshua diagnose --check-api

# 5. Test scheduled assessments
joshua schedule --cron "0 0 * * *" --enable
joshua schedule --status

# 6. Review audit logs
tail -100 /var/log/wargames-joshua/audit.log

# 7. Check file permissions
ls -la /etc/wargames-joshua/
ls -la /var/lib/wargames-joshua/

# 8. Run full test suite
cd /path/to/repo
cargo test --release

# 9. Verify visualizations
ls -la /var/lib/wargames-joshua/output/

# 10. Monitor resource usage
top -p $(pgrep joshua)
```

### Expected Results

✅ All tests passing (67/67)
✅ Assessment completes in < 5 minutes
✅ Memory usage < 500MB
✅ API key encrypted and stored securely
✅ Audit logging operational
✅ Database accessible
✅ Visualizations generated
✅ No critical errors in logs

---

## Troubleshooting

### Issue: Cannot Connect to Database

```bash
# Check PostgreSQL status
sudo systemctl status postgresql

# Test connection
psql -U joshua -d joshua -h localhost

# Check pg_hba.conf
sudo cat /etc/postgresql/14/main/pg_hba.conf
```

### Issue: Permission Denied Errors

```bash
# Fix ownership
sudo chown -R joshua:joshua /var/lib/wargames-joshua
sudo chown -R joshua:joshua /var/log/wargames-joshua

# Fix permissions
sudo chmod 700 /var/lib/wargames-joshua
sudo chmod 750 /var/log/wargames-joshua
```

### Issue: API Key Not Found

```bash
# Re-configure API key
joshua configure --api-key "sk-ant-..." --force

# Verify storage
cat ~/.config/wargames-joshua/.keyring
```

---

## Support

- **Documentation**: See `OPERATIONAL_RUNBOOK.md`
- **Issues**: https://github.com/yourusername/wargames-joshua/issues
- **Security**: security@wargames-joshua.org

---

**Last Updated**: November 19, 2025
**Document Version**: 1.0.0
