# WarGames/JOSHUA: Disaster Recovery and Business Continuity
## Complete DR/BC Strategy for Nuclear Risk Assessment System
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [Overview](#1-overview)
2. [Recovery Objectives](#2-recovery-objectives)
3. [Backup Strategy](#3-backup-strategy)
4. [Disaster Recovery Procedures](#4-disaster-recovery-procedures)
5. [Failover and High Availability](#5-failover-and-high-availability)
6. [Business Continuity Planning](#6-business-continuity-planning)
7. [Testing and Validation](#7-testing-and-validation)
8. [Incident Scenarios](#8-incident-scenarios)
9. [Communication Plan](#9-communication-plan)
10. [Recovery Playbooks](#10-recovery-playbooks)

---

## 1. Overview

### 1.1 Purpose

The WarGames/JOSHUA system monitors **existential nuclear risk** - system failure during a crisis could have catastrophic consequences. This document specifies comprehensive disaster recovery (DR) and business continuity (BC) procedures to ensure:

- **Continuous Operation:** System remains available during regional disasters
- **Rapid Recovery:** Minimize downtime in worst-case scenarios
- **Data Integrity:** Zero data loss for assessments and analysis
- **Business Continuity:** Maintain nuclear risk monitoring capability under all conditions

### 1.2 Scope

This plan covers:
- Complete system failure (AWS region outage)
- Partial component failures (database, API, workers)
- Data corruption or loss
- Security incidents (ransomware, breach)
- Third-party service failures (Claude API, data sources)
- Natural disasters and physical infrastructure damage

### 1.3 Assumptions

- AWS provides 99.99% SLA for multi-AZ deployments
- Claude API maintains 99.5% availability
- Network connectivity remains available (internet, DNS)
- Encryption keys and secrets are securely backed up
- Team members are trained on DR procedures

---

## 2. Recovery Objectives

### 2.1 Recovery Time Objective (RTO)

**Definition:** Maximum acceptable time to restore service after disruption.

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Component           â"‚ RTO Target  â"‚ Maximum Downtime â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ API Service         â"‚ 15 minutes  â"‚ Immediate        â"‚
â"‚ Worker Nodes        â"‚ 30 minutes  â"‚ Next assessment  â"‚
â"‚ Database (Primary)  â"‚ 5 minutes   â"‚ Immediate        â"‚
â"‚ Database (Replica)  â"‚ 2 minutes   â"‚ Auto-failover    â"‚
â"‚ Cache (Redis)       â"‚ 10 minutes  â"‚ Degraded perf    â"‚
â"‚ Complete System     â"‚ 4 hours     â"‚ Cross-region     â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

**Critical Requirement:** During a nuclear crisis, the system MUST remain available. RTO targets are absolute maximums.

### 2.2 Recovery Point Objective (RPO)

**Definition:** Maximum acceptable data loss measured in time.

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Data Type           â"‚ RPO Target  â"‚ Backup Frequency â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ Assessments         â"‚ Zero        â"‚ Real-time sync   â"‚
â"‚ Collected Data      â"‚ 1 hour      â"‚ Hourly snapshot  â"‚
â"‚ Configuration       â"‚ 24 hours    â"‚ Daily backup     â"‚
â"‚ Historical Data     â"‚ 24 hours    â"‚ Daily backup     â"‚
â"‚ Application Logs    â"‚ 15 minutes  â"‚ Real-time stream â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

**Critical Requirement:** Assessment data must have **ZERO** data loss. Every assessment is critical historical data.

### 2.3 Service Level Agreements

**Internal SLA:**
- 99.9% availability (43 minutes/month maximum downtime)
- < 5 minute detection of critical failures
- < 15 minute response time for critical incidents
- Zero data loss for assessment results

**External Dependencies:**
- AWS: 99.99% (multi-AZ)
- Claude API: 99.5% (Anthropic SLA)
- Data Sources: Best effort (non-critical)

---

## 3. Backup Strategy

### 3.1 Database Backups

#### Continuous Backup (Zero RPO)

```rust
/// PostgreSQL WAL-based continuous backup
pub struct ContinuousBackup {
    db_connection: DatabasePool,
    s3_client: S3Client,
    wal_archive_bucket: String,
}

impl ContinuousBackup {
    /// Archive WAL files continuously
    pub async fn archive_wal_segment(&self, wal_file: &Path) -> Result<()> {
        // Upload WAL segment to S3
        let key = format!("wal/{}", wal_file.file_name().unwrap().to_str().unwrap());
        
        self.s3_client
            .put_object()
            .bucket(&self.wal_archive_bucket)
            .key(&key)
            .body(ByteStream::from_path(wal_file).await?)
            .storage_class(StorageClass::StandardIa)
            .server_side_encryption(ServerSideEncryption::Aes256)
            .send()
            .await?;
        
        tracing::info!("Archived WAL segment: {}", key);
        Ok(())
    }
}
```

**PostgreSQL Configuration:**
```sql
-- Enable continuous archiving
ALTER SYSTEM SET wal_level = 'replica';
ALTER SYSTEM SET archive_mode = 'on';
ALTER SYSTEM SET archive_command = '/usr/local/bin/archive-wal %p %f';
ALTER SYSTEM SET archive_timeout = 60;  -- Force archive every 60 seconds

-- Streaming replication
ALTER SYSTEM SET max_wal_senders = 10;
ALTER SYSTEM SET wal_keep_size = '8GB';
```

#### Snapshot Backups

```bash
#!/bin/bash
# Daily full backup script

set -e

BACKUP_DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="wargames_backup_${BACKUP_DATE}.sql.gz"
S3_BUCKET="s3://wargames-backups-production"

# Create compressed backup
pg_dump -h $DB_HOST -U $DB_USER -d wargames \
    --format=custom \
    --compress=9 \
    --verbose \
    | gzip > /tmp/${BACKUP_FILE}

# Upload to S3 with encryption
aws s3 cp /tmp/${BACKUP_FILE} ${S3_BUCKET}/daily/ \
    --storage-class STANDARD_IA \
    --server-side-encryption AES256 \
    --metadata "backup-type=daily,db-version=$(psql -V | cut -d' ' -f3)"

# Upload to secondary region
aws s3 cp /tmp/${BACKUP_FILE} ${S3_BUCKET}-dr/daily/ \
    --region us-west-2 \
    --storage-class STANDARD_IA \
    --server-side-encryption AES256

# Verify backup integrity
pg_restore --list /tmp/${BACKUP_FILE} > /dev/null
if [ $? -eq 0 ]; then
    echo "Backup verified successfully"
else
    echo "ERROR: Backup verification failed" >&2
    exit 1
fi

# Cleanup old backups (keep 90 days)
aws s3 ls ${S3_BUCKET}/daily/ \
    | awk '{print $4}' \
    | head -n -90 \
    | xargs -I {} aws s3 rm ${S3_BUCKET}/daily/{}

# Log backup completion
aws cloudwatch put-metric-data \
    --namespace "WarGames/Backups" \
    --metric-name "BackupSuccess" \
    --value 1 \
    --timestamp $(date -u +%Y-%m-%dT%H:%M:%S)

rm /tmp/${BACKUP_FILE}
```

**Backup Schedule:**
```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Type        â"‚ Frequency    â"‚ Retention â"‚ Storage      â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ WAL Archive â"‚ Continuous   â"‚ 30 days  â"‚ S3 Standard  â"‚
â"‚ Full Backup â"‚ Daily 02:00  â"‚ 90 days  â"‚ S3 IA        â"‚
â"‚ Weekly      â"‚ Sunday 03:00 â"‚ 1 year   â"‚ S3 IA        â"‚
â"‚ Monthly     â"‚ 1st 04:00    â"‚ 7 years  â"‚ S3 Glacier   â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

### 3.2 Configuration Backups

```bash
#!/bin/bash
# Backup all configuration files

CONFIG_BACKUP_DIR="/backup/config/$(date +%Y%m%d_%H%M%S)"
mkdir -p ${CONFIG_BACKUP_DIR}

# Application configuration
cp -r /opt/wargames/config/* ${CONFIG_BACKUP_DIR}/app/

# Kubernetes manifests
kubectl get all -n wargames-production -o yaml > ${CONFIG_BACKUP_DIR}/k8s/manifests.yaml
kubectl get configmaps -n wargames-production -o yaml > ${CONFIG_BACKUP_DIR}/k8s/configmaps.yaml
kubectl get secrets -n wargames-production -o yaml > ${CONFIG_BACKUP_DIR}/k8s/secrets.yaml

# Terraform state
terraform -chdir=/infrastructure state pull > ${CONFIG_BACKUP_DIR}/terraform/terraform.tfstate

# Encrypt and upload
tar -czf config_backup.tar.gz ${CONFIG_BACKUP_DIR}
gpg --encrypt --recipient wargames-backup@example.com config_backup.tar.gz
aws s3 cp config_backup.tar.gz.gpg s3://wargames-config-backups/

# Cleanup
rm -rf ${CONFIG_BACKUP_DIR} config_backup.tar.gz*
```

### 3.3 Application State Backups

```rust
/// Backup assessment data and application state
pub struct StateBackup {
    db: DatabasePool,
    s3_client: S3Client,
}

impl StateBackup {
    pub async fn backup_assessment_data(&self) -> Result<()> {
        let timestamp = Utc::now();
        
        // Export all assessments from last 90 days
        let assessments = sqlx::query!(
            r#"
            SELECT 
                id,
                timestamp,
                seconds_to_midnight,
                risk_level,
                confidence,
                risk_factors,
                detailed_analysis,
                data_quality
            FROM assessments
            WHERE timestamp >= NOW() - INTERVAL '90 days'
            ORDER BY timestamp DESC
            "#
        )
        .fetch_all(&self.db)
        .await?;
        
        // Serialize to JSON
        let backup_data = serde_json::to_string_pretty(&assessments)?;
        
        // Compress
        let compressed = self.compress_data(&backup_data)?;
        
        // Upload to S3 with versioning
        let key = format!(
            "assessments/backup_{}.json.gz",
            timestamp.format("%Y%m%d_%H%M%S")
        );
        
        self.s3_client
            .put_object()
            .bucket("wargames-state-backups")
            .key(&key)
            .body(ByteStream::from(compressed))
            .server_side_encryption(ServerSideEncryption::Aes256)
            .metadata("backup-timestamp", timestamp.to_rfc3339())
            .metadata("record-count", assessments.len().to_string())
            .send()
            .await?;
        
        tracing::info!(
            "Backed up {} assessments to S3: {}",
            assessments.len(),
            key
        );
        
        Ok(())
    }
}
```

### 3.4 Secrets Management Backup

```bash
#!/bin/bash
# Backup AWS Secrets Manager secrets

# Export all secrets
aws secretsmanager list-secrets --region us-east-1 | \
    jq -r '.SecretList[].Name' | \
    while read secret_name; do
        echo "Backing up secret: ${secret_name}"
        
        # Get secret value
        secret_value=$(aws secretsmanager get-secret-value \
            --secret-id "${secret_name}" \
            --region us-east-1 \
            --query 'SecretString' \
            --output text)
        
        # Store in encrypted backup
        echo "${secret_value}" | \
            gpg --encrypt --recipient wargames-backup@example.com > \
            "/backup/secrets/${secret_name}.gpg"
    done

# Upload encrypted secrets to S3
aws s3 sync /backup/secrets/ s3://wargames-secrets-backup/ \
    --sse AES256 \
    --storage-class STANDARD_IA
```

---

## 4. Disaster Recovery Procedures

### 4.1 Complete System Recovery

**Scenario:** AWS us-east-1 region completely unavailable

**Recovery Steps:**

```bash
#!/bin/bash
# Complete system recovery to us-west-2

set -e

echo "Starting complete system recovery..."

# 1. DNS Failover
echo "Step 1: Updating DNS to point to DR region..."
aws route53 change-resource-record-sets \
    --hosted-zone-id Z1234567890ABC \
    --change-batch file://dns-failover.json

# 2. Restore Database
echo "Step 2: Restoring database from backup..."
LATEST_BACKUP=$(aws s3 ls s3://wargames-backups-dr/daily/ \
    | sort | tail -n 1 | awk '{print $4}')

aws s3 cp s3://wargames-backups-dr/daily/${LATEST_BACKUP} /tmp/
pg_restore -h $DR_DB_HOST -U $DB_USER -d wargames /tmp/${LATEST_BACKUP}

# 3. Apply WAL files for point-in-time recovery
echo "Step 3: Applying WAL files..."
aws s3 sync s3://wargames-backups-dr/wal/ /var/lib/postgresql/wal/

# 4. Deploy Application
echo "Step 4: Deploying application to DR region..."
kubectl config use-context wargames-dr-us-west-2
kubectl apply -f /infrastructure/kubernetes/production/

# 5. Scale up services
echo "Step 5: Scaling services..."
kubectl scale deployment/wargames-api --replicas=3 -n wargames-production
kubectl scale deployment/wargames-workers --replicas=5 -n wargames-production

# 6. Verify system health
echo "Step 6: Verifying system health..."
./scripts/health-check.sh

echo "System recovery complete!"
```

**Expected Recovery Time:** 3-4 hours

### 4.2 Database Recovery

**Point-in-Time Recovery (PITR):**

```bash
#!/bin/bash
# Restore database to specific point in time

TARGET_TIME="2025-10-27 12:00:00 UTC"

# 1. Stop current database
systemctl stop postgresql

# 2. Restore base backup
LATEST_BASE_BACKUP=$(aws s3 ls s3://wargames-backups/daily/ \
    | sort | tail -n 1 | awk '{print $4}')
    
pg_restore -d wargames /backup/${LATEST_BASE_BACKUP}

# 3. Create recovery configuration
cat > /var/lib/postgresql/recovery.conf << EOF
restore_command = 'aws s3 cp s3://wargames-backups/wal/%f %p'
recovery_target_time = '${TARGET_TIME}'
recovery_target_action = 'promote'
EOF

# 4. Start PostgreSQL (will apply WAL up to target time)
systemctl start postgresql

# 5. Verify recovery
psql -d wargames -c "SELECT NOW(), pg_last_wal_replay_lsn();"
```

### 4.3 Data Corruption Recovery

**Scenario:** Database corruption detected

```sql
-- 1. Identify corrupted data
SELECT 
    tablename,
    pg_relation_filepath(oid),
    pg_size_pretty(pg_total_relation_size(oid))
FROM pg_class
WHERE relname = 'assessments';

-- 2. Check for corruption
VACUUM FULL VERBOSE assessments;

-- If corruption detected:
-- 3. Export uncorrupted data
COPY (
    SELECT * FROM assessments 
    WHERE id NOT IN (SELECT id FROM corrupted_records)
) TO '/tmp/assessments_clean.csv' CSV HEADER;

-- 4. Drop and recreate table
DROP TABLE assessments CASCADE;
CREATE TABLE assessments (...);  -- Use schema from backup

-- 5. Import clean data
COPY assessments FROM '/tmp/assessments_clean.csv' CSV HEADER;

-- 6. Restore missing data from backup
pg_restore --table=assessments /backup/latest.backup
```

---

## 5. Failover and High Availability

### 5.1 Architecture Overview

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚                 PRIMARY REGION (us-east-1)            â"‚
â"‚                                                        â"‚
â"‚  â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"     â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"     â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"   â"‚
â"‚  â"‚   AZ-1   â"‚     â"‚   AZ-2   â"‚     â"‚   AZ-3   â"‚   â"‚
â"‚  â"‚          â"‚     â"‚          â"‚     â"‚          â"‚   â"‚
â"‚  â"‚ API      â"‚     â"‚ API      â"‚     â"‚ API      â"‚   â"‚
â"‚  â"‚ Workers  â"‚     â"‚ Workers  â"‚     â"‚ Workers  â"‚   â"‚
â"‚  â"‚ DB-Pri   â"‚====â"‚ DB-Rep   â"‚====â"‚ DB-Rep   â"‚   â"‚
â"‚  â"‚ Redis    â"‚     â"‚ Redis    â"‚     â"‚ Redis    â"‚   â"‚
â"‚  â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜     â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜     â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜   â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
                        â"‚
                        â"‚ Continuous Replication
                        â"‚
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚                        â–¼                            â"‚
â"‚               DR REGION (us-west-2)                  â"‚
â"‚                                                        â"‚
â"‚  â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"     â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"     â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"   â"‚
â"‚  â"‚   AZ-1   â"‚     â"‚   AZ-2   â"‚     â"‚   AZ-3   â"‚   â"‚
â"‚  â"‚          â"‚     â"‚          â"‚     â"‚          â"‚   â"‚
â"‚  â"‚ API      â"‚     â"‚ API      â"‚     â"‚ API      â"‚   â"‚
â"‚  â"‚ Workers  â"‚     â"‚ Workers  â"‚     â"‚ Workers  â"‚   â"‚
â"‚  â"‚ DB-Stby  â"‚====â"‚ DB-Stby  â"‚====â"‚ DB-Stby  â"‚   â"‚
â"‚  â"‚ Redis    â"‚     â"‚ Redis    â"‚     â"‚ Redis    â"‚   â"‚
â"‚  â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜     â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜     â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜   â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

### 5.2 Database Failover

**Automatic Failover (Multi-AZ):**

```sql
-- PostgreSQL streaming replication configuration

-- Primary server (postgresql.conf)
wal_level = 'replica'
max_wal_senders = 10
wal_keep_size = '8GB'
synchronous_commit = 'on'
synchronous_standby_names = 'standby1,standby2'

-- Standby server (postgresql.conf)
hot_standby = 'on'
max_standby_streaming_delay = '30s'
wal_receiver_status_interval = '10s'
```

**Monitoring Script:**
```bash
#!/bin/bash
# Monitor database replication lag

while true; do
    # Check replication lag
    LAG=$(psql -h $PRIMARY_HOST -U $DB_USER -d wargames -t -c \
        "SELECT EXTRACT(EPOCH FROM (NOW() - pg_last_xact_replay_timestamp()))::INTEGER;")
    
    # Alert if lag > 30 seconds
    if [ "$LAG" -gt 30 ]; then
        echo "WARNING: Replication lag is ${LAG} seconds"
        
        # Send alert
        aws sns publish \
            --topic-arn "arn:aws:sns:us-east-1:123456789:wargames-alerts" \
            --message "Database replication lag: ${LAG}s" \
            --subject "Database Replication Alert"
    fi
    
    sleep 10
done
```

### 5.3 Application Failover

**API Service Failover:**

```yaml
# Kubernetes Deployment with Pod Disruption Budget

apiVersion: apps/v1
kind: Deployment
metadata:
  name: wargames-api
  namespace: wargames-production
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  template:
    spec:
      affinity:
        podAntiAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            - labelSelector:
                matchLabels:
                  app: wargames-api
              topologyKey: kubernetes.io/hostname
      containers:
        - name: api
          image: wargames/api:latest
          readinessProbe:
            httpGet:
              path: /health
              port: 8080
            initialDelaySeconds: 10
            periodSeconds: 5
          livenessProbe:
            httpGet:
              path: /health
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 10

---
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: wargames-api-pdb
spec:
  minAvailable: 2
  selector:
    matchLabels:
      app: wargames-api
```

---

## 6. Business Continuity Planning

### 6.1 Critical Functions

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Function            â"‚ Priority â"‚ Manual Fallback    â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ Risk Assessments    â"‚ CRITICAL â"‚ Manual analysis    â"‚
â"‚ Data Collection     â"‚ HIGH     â"‚ Manual monitoring  â"‚
â"‚ Historical Data     â"‚ HIGH     â"‚ Read-only archive  â"‚
â"‚ Reporting           â"‚ MEDIUM   â"‚ Manual reports     â"‚
â"‚ API Access          â"‚ MEDIUM   â"‚ Direct DB queries  â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

### 6.2 Manual Fallback Procedures

**Manual Risk Assessment:**
```markdown
# Emergency Manual Risk Assessment Procedure

When automated system is unavailable:

1. **Data Collection**
   - Monitor news: Reuters, BBC, AP, Al Jazeera
   - Check government sources: State Dept, IAEA, UN
   - Review think tanks: SIPRI, Carnegie, RAND
   - Social media: Twitter, Reddit /r/worldnews

2. **Analysis Framework**
   - Use Doomsday Clock methodology
   - Apply weighted scoring:
     * Nuclear arsenal changes: 20%
     * Arms control breakdown: 20%
     * Regional conflicts: 20%
     * Leadership rhetoric: 10%
     * Technical incidents: 15%
     * Communication breakdown: 10%
     * Emerging technology: 10%
     * Economic factors: 5%

3. **Scoring**
   - 0-120 seconds: CRITICAL
   - 120-300 seconds: SEVERE
   - 300-600 seconds: ELEVATED
   - 600-900 seconds: MODERATE
   - 900+ seconds: LOW

4. **Documentation**
   - Record assessment in shared spreadsheet
   - Document sources and reasoning
   - Email summary to stakeholders
   - Update status page

5. **Communication**
   - Post summary in #wargames-manual-assessments
   - Notify on-call team
   - Update stakeholders every 4 hours
```

### 6.3 Stakeholder Communication

**Communication Matrix:**
```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Stakeholder  â"‚ Incident Type â"‚ Communication Method â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ Executives   â"‚ Critical      â"‚ Phone + Email        â"‚
â"‚ Engineering  â"‚ All           â"‚ Slack + PagerDuty    â"‚
â"‚ Operations   â"‚ All           â"‚ Slack + Email        â"‚
â"‚ End Users    â"‚ Outage        â"‚ Status Page          â"‚
â"‚ Partners     â"‚ Outage        â"‚ Email                â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

---

## 7. Testing and Validation

### 7.1 DR Testing Schedule

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Test Type     â"‚ Frequency  â"‚ Duration â"‚ Impact      â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ Backup Test   â"‚ Weekly     â"‚ 1 hour   â"‚ None        â"‚
â"‚ DB Failover   â"‚ Monthly    â"‚ 30 min   â"‚ < 2 min     â"‚
â"‚ App Failover  â"‚ Monthly    â"‚ 1 hour   â"‚ < 15 min    â"‚
â"‚ Full DR Drill â"‚ Quarterly  â"‚ 4 hours  â"‚ Planned     â"‚
â"‚ Chaos Test    â"‚ Monthly    â"‚ Varies   â"‚ Minimal     â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

### 7.2 Backup Verification

```bash
#!/bin/bash
# Automated backup verification

# 1. Download latest backup
LATEST_BACKUP=$(aws s3 ls s3://wargames-backups/daily/ | sort | tail -n 1 | awk '{print $4}')
aws s3 cp s3://wargames-backups/daily/${LATEST_BACKUP} /tmp/

# 2. Restore to test database
createdb wargames_test
pg_restore -d wargames_test /tmp/${LATEST_BACKUP}

# 3. Verify data integrity
psql -d wargames_test -c "
    SELECT 
        COUNT(*) as total_assessments,
        MAX(timestamp) as latest_assessment,
        MIN(timestamp) as earliest_assessment
    FROM assessments;
"

# 4. Run validation queries
psql -d wargames_test -f /tests/validation_queries.sql

# 5. Cleanup
dropdb wargames_test
rm /tmp/${LATEST_BACKUP}

# 6. Report results
if [ $? -eq 0 ]; then
    aws cloudwatch put-metric-data \
        --namespace "WarGames/Backups" \
        --metric-name "BackupVerification" \
        --value 1
else
    aws cloudwatch put-metric-data \
        --namespace "WarGames/Backups" \
        --metric-name "BackupVerification" \
        --value 0
    
    # Send alert
    aws sns publish \
        --topic-arn "arn:aws:sns:us-east-1:123456789:wargames-critical" \
        --subject "Backup Verification Failed" \
        --message "Latest backup verification failed"
fi
```

### 7.3 DR Drill Checklist

```markdown
# Quarterly DR Drill Checklist

## Pre-Drill (1 week before)
- [ ] Schedule drill window (Saturday 2am-6am ET)
- [ ] Notify all stakeholders
- [ ] Update status page with planned maintenance
- [ ] Review and update DR procedures
- [ ] Verify backup integrity
- [ ] Test DR region access

## During Drill
- [ ] T-0: Simulate primary region failure
- [ ] T+5min: Initiate failover procedures
- [ ] T+30min: Verify DNS propagation
- [ ] T+1hr: Restore database in DR region
- [ ] T+2hr: Deploy application to DR region
- [ ] T+3hr: Run smoke tests
- [ ] T+4hr: Full system validation

## Post-Drill
- [ ] Document actual vs target RTO/RPO
- [ ] Identify issues and gaps
- [ ] Update procedures based on learnings
- [ ] Schedule follow-up for fixes
- [ ] Update DR documentation
- [ ] Send post-mortem report
```

---

## 8. Incident Scenarios

### 8.1 Scenario: AWS Region Failure

**Detection:**
- CloudWatch alarms: Service unavailable
- Health checks failing across all AZs
- Cannot reach any instances in region

**Response:**
1. Confirm region-wide outage (AWS Status Dashboard)
2. Initiate DR failover to us-west-2
3. Update DNS to point to DR region
4. Restore latest backup to DR database
5. Deploy application stack
6. Validate system functionality
7. Update status page
8. Notify stakeholders

**Expected Duration:** 3-4 hours

### 8.2 Scenario: Database Corruption

**Detection:**
- Data validation checks failing
- Inconsistent query results
- PostgreSQL corruption errors in logs

**Response:**
1. Stop writes to database
2. Export uncorrupted data
3. Restore from latest valid backup
4. Apply WAL files for PITR
5. Rerun assessments for affected period
6. Validate data integrity
7. Resume normal operations

**Expected Duration:** 2-3 hours

### 8.3 Scenario: Ransomware Attack

**Detection:**
- Encrypted files detected
- Unusual system behavior
- Ransom note found

**Response:**
1. **IMMEDIATE:** Isolate affected systems
2. Disable all external access
3. Notify security team and management
4. Preserve evidence for forensics
5. Restore from clean backup (verified pre-infection)
6. Rebuild infrastructure from scratch
7. Enhanced security monitoring
8. Post-incident security audit

**Expected Duration:** 1-2 days

---

## 9. Communication Plan

### 9.1 Status Page Updates

```markdown
# Status Page Template

## Investigating
We are currently investigating reports of [issue description]. 
We will provide updates as more information becomes available.

Posted: [timestamp]

## Identified
We have identified the root cause: [description]
Our team is working to resolve this issue.

ETA: [estimated resolution time]
Posted: [timestamp]

## Monitoring
A fix has been implemented and deployed. 
We are monitoring the system for stability.

Posted: [timestamp]

## Resolved
This incident has been resolved. All systems are operating normally.

Total duration: [X hours Y minutes]
Posted: [timestamp]
```

### 9.2 Incident Email Template

```
Subject: [RESOLVED|IN PROGRESS] WarGames/JOSHUA System Incident

Hi team,

STATUS: [Resolved|Investigating|In Progress]

INCIDENT SUMMARY:
[Brief description of what happened]

IMPACT:
- Affected services: [list]
- User impact: [description]
- Duration: [X hours Y minutes]

ROOT CAUSE:
[Technical explanation of what caused the incident]

RESOLUTION:
[Description of fix applied]

NEXT STEPS:
- [ ] Post-mortem scheduled for [date]
- [ ] Follow-up actions: [list]

For questions, contact: [email]

Thanks,
WarGames/JOSHUA Operations Team
```

---

## 10. Recovery Playbooks

### 10.1 Database Failover Playbook

```bash
#!/bin/bash
# Database failover to standby

set -e

echo "Starting database failover..."

# 1. Promote standby to primary
pg_ctl promote -D /var/lib/postgresql/data

# 2. Update application configuration
kubectl set env deployment/wargames-api \
    DATABASE_HOST=standby-1.wargames.internal \
    -n wargames-production

# 3. Verify new primary
psql -h standby-1.wargames.internal -c "SELECT pg_is_in_recovery();"

# 4. Update monitoring
aws cloudwatch put-metric-data \
    --namespace "WarGames/Database" \
    --metric-name "Failover" \
    --value 1 \
    --dimensions Service=postgresql,Event=failover

echo "Database failover complete!"
```

### 10.2 Complete Region Failover Playbook

See Section 4.1 for detailed steps.

---

## Summary

This disaster recovery and business continuity plan ensures:

✓ **Zero data loss** for critical assessment data (RPO: 0)  
✓ **Rapid recovery** from any failure scenario (RTO: < 4 hours)  
✓ **Continuous availability** during regional disasters  
✓ **Comprehensive backups** with automated verification  
✓ **Tested procedures** through regular DR drills  
✓ **Clear communication** during incidents  

**Document Version:** 1.0.0  
**Last Updated:** October 2025  
**Maintained By:** WarGames/JOSHUA Operations Team  
**Next Review:** November 2025  
**Next DR Drill:** Q1 2026

*"Hope for the best, plan for the worst, expect the unexpected."*
