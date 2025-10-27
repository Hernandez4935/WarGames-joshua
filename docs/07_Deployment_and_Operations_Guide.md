# WarGames/JOSHUA: Deployment & Operations Guide
## Production Deployment and Operational Procedures
### Version 1.0.0 | October 2025

---

## Executive Summary

This document provides comprehensive deployment and operational procedures for the WarGames/JOSHUA nuclear risk assessment system. It covers environment setup, deployment strategies, infrastructure provisioning, operational runbooks, and day-to-day maintenance procedures.

### Deployment Philosophy

1. **Infrastructure as Code**: All infrastructure defined in version control
2. **Immutable Infrastructure**: Deploy new versions, never modify running systems
3. **Zero-Downtime Deployments**: Blue-green or canary deployments
4. **Observability First**: Comprehensive logging and monitoring from day one
5. **Security Hardened**: Defense-in-depth security at all layers
6. **Disaster Recovery Ready**: Automated backups and tested recovery procedures

---

## Table of Contents

1. [Environment Architecture](#environment-architecture)
2. [Infrastructure Provisioning](#infrastructure-provisioning)
3. [Deployment Procedures](#deployment-procedures)
4. [Configuration Management](#configuration-management)
5. [Service Management](#service-management)
6. [Monitoring and Health Checks](#monitoring-and-health-checks)
7. [Operational Runbooks](#operational-runbooks)
8. [Maintenance Procedures](#maintenance-procedures)
9. [Troubleshooting Guide](#troubleshooting-guide)
10. [Scaling Procedures](#scaling-procedures)

---

## 1. Environment Architecture

### 1.1 Environment Tiers

```
┌──────────────────────────────────────────────────────────────────┐
│                    ENVIRONMENT HIERARCHY                         │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┏━━━━━━━━━━━━━━┓                                                │
│  ┃ Development  ┃  ← Local machines, Docker Compose             │
│  ┗━━━━━━━━━━━━━━┛    Single developer instance                  │
│         │                                                        │
│         ▼                                                        │
│  ┏━━━━━━━━━━━━━━┓                                                │
│  ┃   Testing    ┃  ← CI/CD runners, automated tests             │
│  ┗━━━━━━━━━━━━━━┛    Ephemeral test environments                │
│         │                                                        │
│         ▼                                                        │
│  ┏━━━━━━━━━━━━━━┓                                                │
│  ┃   Staging    ┃  ← Pre-production environment                 │
│  ┗━━━━━━━━━━━━━━┛    Production-identical infrastructure        │
│         │                                                        │
│         ▼                                                        │
│  ┏━━━━━━━━━━━━━━┓                                                │
│  ┃ Production   ┃  ← Live system serving assessments            │
│  ┗━━━━━━━━━━━━━━┛    High availability, monitored 24/7          │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### 1.2 Production Infrastructure Components

```yaml
# infrastructure/production/components.yml
production_stack:
  compute:
    application_servers:
      count: 3  # Minimum for high availability
      type: "c6i.2xlarge"  # 8 vCPU, 16 GB RAM
      os: "Ubuntu 22.04 LTS"

    scheduler:
      count: 1
      type: "t3.medium"
      purpose: "Cron jobs and scheduled assessments"

  database:
    postgresql:
      type: "db.r6g.xlarge"  # 4 vCPU, 32 GB RAM
      storage: "500 GB SSD"
      backup_retention: "30 days"
      multi_az: true
      replicas: 2  # Read replicas for analytics

  cache:
    redis:
      type: "cache.m6g.large"
      nodes: 3  # Cluster mode

  storage:
    artifacts:
      type: "S3"
      bucket: "wargames-assessments-prod"
      lifecycle:
        transition_to_glacier: "90 days"
        expiration: "7 years"

    backups:
      type: "S3"
      bucket: "wargames-backups-prod"
      versioning: enabled
      cross_region_replication: true

  networking:
    load_balancer:
      type: "Application Load Balancer"
      ssl_certificate: "ACM managed"
      health_check_interval: "30 seconds"

    vpc:
      cidr: "10.0.0.0/16"
      availability_zones: 3
      public_subnets: 3
      private_subnets: 3
      nat_gateways: 3

  security:
    waf: enabled
    ddos_protection: "AWS Shield Standard"
    secrets_manager: enabled
    kms_encryption: enabled
```

### 1.3 Service Topology

```
┌────────────────────────── PRODUCTION TOPOLOGY ───────────────────────────┐
│                                                                          │
│                          Internet Gateway                                │
│                                 │                                        │
│                                 ▼                                        │
│  ┌────────────────────────────────────────────────────────────────┐     │
│  │                    Application Load Balancer                   │     │
│  │              (SSL Termination, WAF, DDoS Protection)           │     │
│  └────────────────┬───────────────────┬───────────────────┬───────┘     │
│                   │                   │                   │             │
│         ┌─────────▼────────┐ ┌───────▼────────┐ ┌───────▼────────┐     │
│         │ WarGames App 1   │ │ WarGames App 2 │ │ WarGames App 3 │     │
│         │ (Auto Scaling)   │ │ (Auto Scaling) │ │ (Auto Scaling) │     │
│         └─────────┬────────┘ └───────┬────────┘ └───────┬────────┘     │
│                   │                   │                   │             │
│                   └─────────┬─────────┴─────────┬─────────┘             │
│                             │                   │                       │
│                   ┌─────────▼────────┐ ┌───────▼────────┐               │
│                   │   PostgreSQL     │ │     Redis      │               │
│                   │   Primary + 2    │ │    Cluster     │               │
│                   │   Read Replicas  │ │   (3 nodes)    │               │
│                   └──────────────────┘ └────────────────┘               │
│                                                                          │
│  ┌────────────────────────────────────────────────────────────────┐     │
│  │                    External Dependencies                       │     │
│  │  • Claude API (Anthropic)                                      │     │
│  │  • News APIs (NewsAPI, Reuters, etc.)                          │     │
│  │  • Government Data Sources                                     │     │
│  │  • Research Databases (SIPRI, Carnegie)                        │     │
│  └────────────────────────────────────────────────────────────────┘     │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Infrastructure Provisioning

### 2.1 Terraform Infrastructure as Code

```hcl
# infrastructure/terraform/main.tf

terraform {
  required_version = ">= 1.6.0"

  backend "s3" {
    bucket         = "wargames-terraform-state"
    key            = "production/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "wargames-terraform-locks"
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region

  default_tags {
    tags = {
      Project     = "WarGames-JOSHUA"
      Environment = var.environment
      ManagedBy   = "Terraform"
      CostCenter  = "Research"
    }
  }
}

# ────────────────────────────────────────────────────────────
# VPC and Networking
# ────────────────────────────────────────────────────────────

module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  version = "~> 5.0"

  name = "wargames-${var.environment}-vpc"
  cidr = var.vpc_cidr

  azs             = data.aws_availability_zones.available.names
  private_subnets = var.private_subnet_cidrs
  public_subnets  = var.public_subnet_cidrs

  enable_nat_gateway   = true
  enable_dns_hostnames = true
  enable_dns_support   = true

  # One NAT Gateway per AZ for high availability
  single_nat_gateway = false
  one_nat_gateway_per_az = true
}

# ────────────────────────────────────────────────────────────
# Security Groups
# ────────────────────────────────────────────────────────────

resource "aws_security_group" "application" {
  name        = "wargames-${var.environment}-app"
  description = "Security group for WarGames application servers"
  vpc_id      = module.vpc.vpc_id

  # Allow inbound from ALB
  ingress {
    from_port       = 8080
    to_port         = 8080
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id]
  }

  # Allow outbound to internet (for API calls)
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "wargames-${var.environment}-app-sg"
  }
}

resource "aws_security_group" "database" {
  name        = "wargames-${var.environment}-db"
  description = "Security group for PostgreSQL database"
  vpc_id      = module.vpc.vpc_id

  # Allow PostgreSQL from application servers
  ingress {
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.application.id]
  }

  tags = {
    Name = "wargames-${var.environment}-db-sg"
  }
}

# ────────────────────────────────────────────────────────────
# RDS PostgreSQL Database
# ────────────────────────────────────────────────────────────

resource "aws_db_instance" "postgresql" {
  identifier     = "wargames-${var.environment}"
  engine         = "postgres"
  engine_version = "15.4"

  instance_class    = var.db_instance_class
  allocated_storage = var.db_allocated_storage
  storage_type      = "gp3"
  storage_encrypted = true
  kms_key_id        = aws_kms_key.database.arn

  db_name  = "wargames"
  username = "wargames_admin"
  password = random_password.db_password.result

  # High Availability
  multi_az               = var.environment == "production"
  backup_retention_period = 30
  backup_window          = "03:00-04:00"  # UTC
  maintenance_window     = "sun:04:00-sun:05:00"  # UTC

  # Networking
  db_subnet_group_name   = aws_db_subnet_group.database.name
  vpc_security_group_ids = [aws_security_group.database.id]
  publicly_accessible    = false

  # Monitoring
  enabled_cloudwatch_logs_exports = ["postgresql", "upgrade"]
  monitoring_interval             = 60
  monitoring_role_arn             = aws_iam_role.rds_monitoring.arn

  # Performance Insights
  performance_insights_enabled    = true
  performance_insights_retention_period = 7

  # Parameter Group
  parameter_group_name = aws_db_parameter_group.postgresql.name

  # Deletion Protection
  deletion_protection = var.environment == "production"
  skip_final_snapshot = var.environment != "production"
  final_snapshot_identifier = var.environment == "production" ? "wargames-final-${formatdate("YYYY-MM-DD-hhmm", timestamp())}" : null

  tags = {
    Name = "wargames-${var.environment}-db"
  }
}

# Read Replicas for Production
resource "aws_db_instance" "read_replica" {
  count = var.environment == "production" ? 2 : 0

  identifier          = "wargames-${var.environment}-replica-${count.index + 1}"
  replicate_source_db = aws_db_instance.postgresql.identifier

  instance_class = var.db_replica_instance_class

  publicly_accessible = false

  tags = {
    Name = "wargames-${var.environment}-replica-${count.index + 1}"
  }
}

# ────────────────────────────────────────────────────────────
# ElastiCache Redis Cluster
# ────────────────────────────────────────────────────────────

resource "aws_elasticache_replication_group" "redis" {
  replication_group_id       = "wargames-${var.environment}"
  replication_group_description = "Redis cluster for WarGames caching"

  engine               = "redis"
  engine_version       = "7.0"
  node_type            = var.redis_node_type
  num_cache_clusters   = var.redis_num_nodes
  parameter_group_name = "default.redis7"
  port                 = 6379

  subnet_group_name  = aws_elasticache_subnet_group.redis.name
  security_group_ids = [aws_security_group.redis.id]

  at_rest_encryption_enabled = true
  transit_encryption_enabled = true
  auth_token                 = random_password.redis_auth.result

  automatic_failover_enabled = var.environment == "production"
  multi_az_enabled           = var.environment == "production"

  snapshot_retention_limit = 7
  snapshot_window         = "03:00-05:00"
  maintenance_window      = "sun:05:00-sun:07:00"

  tags = {
    Name = "wargames-${var.environment}-redis"
  }
}

# ────────────────────────────────────────────────────────────
# Application Load Balancer
# ────────────────────────────────────────────────────────────

resource "aws_lb" "application" {
  name               = "wargames-${var.environment}-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.alb.id]
  subnets            = module.vpc.public_subnets

  enable_deletion_protection = var.environment == "production"
  enable_http2              = true
  enable_waf_fail_open      = false

  access_logs {
    bucket  = aws_s3_bucket.alb_logs.id
    enabled = true
  }

  tags = {
    Name = "wargames-${var.environment}-alb"
  }
}

resource "aws_lb_target_group" "application" {
  name     = "wargames-${var.environment}-tg"
  port     = 8080
  protocol = "HTTP"
  vpc_id   = module.vpc.vpc_id

  health_check {
    enabled             = true
    healthy_threshold   = 2
    unhealthy_threshold = 3
    timeout             = 5
    interval            = 30
    path                = "/health"
    matcher             = "200"
  }

  stickiness {
    type            = "lb_cookie"
    cookie_duration = 3600
    enabled         = true
  }

  deregistration_delay = 30

  tags = {
    Name = "wargames-${var.environment}-tg"
  }
}

# ────────────────────────────────────────────────────────────
# Auto Scaling Group
# ────────────────────────────────────────────────────────────

resource "aws_launch_template" "application" {
  name_prefix   = "wargames-${var.environment}-"
  image_id      = data.aws_ami.ubuntu.id
  instance_type = var.app_instance_type

  vpc_security_group_ids = [aws_security_group.application.id]

  iam_instance_profile {
    name = aws_iam_instance_profile.application.name
  }

  user_data = base64encode(templatefile("${path.module}/user_data.sh", {
    environment    = var.environment
    app_version    = var.app_version
    db_endpoint    = aws_db_instance.postgresql.endpoint
    redis_endpoint = aws_elasticache_replication_group.redis.primary_endpoint_address
  }))

  monitoring {
    enabled = true
  }

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"  # IMDSv2
    http_put_response_hop_limit = 1
  }

  tag_specifications {
    resource_type = "instance"

    tags = {
      Name        = "wargames-${var.environment}-app"
      Environment = var.environment
    }
  }
}

resource "aws_autoscaling_group" "application" {
  name                = "wargames-${var.environment}-asg"
  vpc_zone_identifier = module.vpc.private_subnets

  min_size         = var.asg_min_size
  max_size         = var.asg_max_size
  desired_capacity = var.asg_desired_capacity

  health_check_type         = "ELB"
  health_check_grace_period = 300

  launch_template {
    id      = aws_launch_template.application.id
    version = "$Latest"
  }

  target_group_arns = [aws_lb_target_group.application.arn]

  enabled_metrics = [
    "GroupDesiredCapacity",
    "GroupInServiceInstances",
    "GroupMaxSize",
    "GroupMinSize",
    "GroupPendingInstances",
    "GroupStandbyInstances",
    "GroupTerminatingInstances",
    "GroupTotalInstances",
  ]

  tag {
    key                 = "Name"
    value               = "wargames-${var.environment}-app"
    propagate_at_launch = true
  }
}

# Auto Scaling Policies
resource "aws_autoscaling_policy" "scale_up" {
  name                   = "wargames-${var.environment}-scale-up"
  autoscaling_group_name = aws_autoscaling_group.application.name
  adjustment_type        = "ChangeInCapacity"
  scaling_adjustment     = 1
  cooldown               = 300
}

resource "aws_autoscaling_policy" "scale_down" {
  name                   = "wargames-${var.environment}-scale-down"
  autoscaling_group_name = aws_autoscaling_group.application.name
  adjustment_type        = "ChangeInCapacity"
  scaling_adjustment     = -1
  cooldown               = 300
}

# CloudWatch Alarms for Auto Scaling
resource "aws_cloudwatch_metric_alarm" "high_cpu" {
  alarm_name          = "wargames-${var.environment}-high-cpu"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/EC2"
  period              = 120
  statistic           = "Average"
  threshold           = 80

  dimensions = {
    AutoScalingGroupName = aws_autoscaling_group.application.name
  }

  alarm_actions = [aws_autoscaling_policy.scale_up.arn]
}

resource "aws_cloudwatch_metric_alarm" "low_cpu" {
  alarm_name          = "wargames-${var.environment}-low-cpu"
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/EC2"
  period              = 120
  statistic           = "Average"
  threshold           = 20

  dimensions = {
    AutoScalingGroupName = aws_autoscaling_group.application.name
  }

  alarm_actions = [aws_autoscaling_policy.scale_down.arn]
}

# ────────────────────────────────────────────────────────────
# S3 Buckets
# ────────────────────────────────────────────────────────────

resource "aws_s3_bucket" "assessments" {
  bucket = "wargames-${var.environment}-assessments"

  tags = {
    Name = "wargames-${var.environment}-assessments"
  }
}

resource "aws_s3_bucket_versioning" "assessments" {
  bucket = aws_s3_bucket.assessments.id

  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "assessments" {
  bucket = aws_s3_bucket.assessments.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm     = "aws:kms"
      kms_master_key_id = aws_kms_key.s3.arn
    }
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "assessments" {
  bucket = aws_s3_bucket.assessments.id

  rule {
    id     = "transition-to-glacier"
    status = "Enabled"

    transition {
      days          = 90
      storage_class = "GLACIER"
    }

    expiration {
      days = 2555  # 7 years
    }
  }
}

# ────────────────────────────────────────────────────────────
# Secrets Manager
# ────────────────────────────────────────────────────────────

resource "aws_secretsmanager_secret" "claude_api_key" {
  name        = "wargames/${var.environment}/claude-api-key"
  description = "Claude API key for WarGames system"

  kms_key_id = aws_kms_key.secrets.arn
}

resource "aws_secretsmanager_secret" "database_credentials" {
  name        = "wargames/${var.environment}/database"
  description = "Database credentials for WarGames system"

  kms_key_id = aws_kms_key.secrets.arn
}

resource "aws_secretsmanager_secret_version" "database_credentials" {
  secret_id     = aws_secretsmanager_secret.database_credentials.id
  secret_string = jsonencode({
    username = aws_db_instance.postgresql.username
    password = random_password.db_password.result
    host     = aws_db_instance.postgresql.endpoint
    port     = aws_db_instance.postgresql.port
    database = aws_db_instance.postgresql.db_name
  })
}

# ────────────────────────────────────────────────────────────
# Outputs
# ────────────────────────────────────────────────────────────

output "alb_dns_name" {
  description = "DNS name of the application load balancer"
  value       = aws_lb.application.dns_name
}

output "database_endpoint" {
  description = "PostgreSQL database endpoint"
  value       = aws_db_instance.postgresql.endpoint
  sensitive   = true
}

output "redis_endpoint" {
  description = "Redis cluster endpoint"
  value       = aws_elasticache_replication_group.redis.primary_endpoint_address
  sensitive   = true
}
```

### 2.2 User Data Script for EC2 Instances

```bash
#!/bin/bash
# infrastructure/terraform/user_data.sh

set -e

# ────────────────────────────────────────────────────────────
# System Update and Dependencies
# ────────────────────────────────────────────────────────────

apt-get update
apt-get upgrade -y
apt-get install -y \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    postgresql-client \
    redis-tools \
    awscli \
    jq

# ────────────────────────────────────────────────────────────
# Install Rust
# ────────────────────────────────────────────────────────────

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# ────────────────────────────────────────────────────────────
# CloudWatch Agent
# ────────────────────────────────────────────────────────────

wget https://s3.amazonaws.com/amazoncloudwatch-agent/ubuntu/amd64/latest/amazon-cloudwatch-agent.deb
dpkg -i -E ./amazon-cloudwatch-agent.deb

cat > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json <<'EOF'
{
  "agent": {
    "metrics_collection_interval": 60,
    "run_as_user": "cwagent"
  },
  "logs": {
    "logs_collected": {
      "files": {
        "collect_list": [
          {
            "file_path": "/var/log/wargames/application.log",
            "log_group_name": "/wargames/${environment}/application",
            "log_stream_name": "{instance_id}"
          }
        ]
      }
    }
  },
  "metrics": {
    "namespace": "WarGames/${environment}",
    "metrics_collected": {
      "cpu": {
        "measurement": [
          {"name": "cpu_usage_idle", "rename": "CPU_IDLE", "unit": "Percent"},
          {"name": "cpu_usage_iowait", "rename": "CPU_IOWAIT", "unit": "Percent"}
        ],
        "metrics_collection_interval": 60,
        "totalcpu": false
      },
      "disk": {
        "measurement": [
          {"name": "used_percent", "rename": "DISK_USED", "unit": "Percent"}
        ],
        "metrics_collection_interval": 60,
        "resources": ["*"]
      },
      "mem": {
        "measurement": [
          {"name": "mem_used_percent", "rename": "MEM_USED", "unit": "Percent"}
        ],
        "metrics_collection_interval": 60
      }
    }
  }
}
EOF

/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl \
    -a fetch-config \
    -m ec2 \
    -s \
    -c file:/opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json

# ────────────────────────────────────────────────────────────
# Create Application User
# ────────────────────────────────────────────────────────────

useradd -m -s /bin/bash wargames
mkdir -p /opt/wargames
mkdir -p /var/log/wargames
chown -R wargames:wargames /opt/wargames /var/log/wargames

# ────────────────────────────────────────────────────────────
# Fetch Application from S3
# ────────────────────────────────────────────────────────────

aws s3 cp s3://wargames-artifacts/${environment}/wargames-joshua-${app_version}.tar.gz /tmp/
tar -xzf /tmp/wargames-joshua-${app_version}.tar.gz -C /opt/wargames/
chown -R wargames:wargames /opt/wargames/

# ────────────────────────────────────────────────────────────
# Fetch Configuration from Secrets Manager
# ────────────────────────────────────────────────────────────

aws secretsmanager get-secret-value \
    --secret-id wargames/${environment}/database \
    --query SecretString \
    --output text | jq -r 'to_entries | .[] | "\(.key)=\(.value)"' > /opt/wargames/.env.database

aws secretsmanager get-secret-value \
    --secret-id wargames/${environment}/claude-api-key \
    --query SecretString \
    --output text > /opt/wargames/.env.claude

chmod 600 /opt/wargames/.env.*
chown wargames:wargames /opt/wargames/.env.*

# ────────────────────────────────────────────────────────────
# Create Systemd Service
# ────────────────────────────────────────────────────────────

cat > /etc/systemd/system/wargames.service <<'EOF'
[Unit]
Description=WarGames/JOSHUA Nuclear Risk Assessment System
After=network.target

[Service]
Type=simple
User=wargames
Group=wargames
WorkingDirectory=/opt/wargames
EnvironmentFile=/opt/wargames/.env.database
EnvironmentFile=/opt/wargames/.env.claude
ExecStart=/opt/wargames/bin/joshua server
Restart=always
RestartSec=10
StandardOutput=append:/var/log/wargames/application.log
StandardError=append:/var/log/wargames/error.log

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/wargames /opt/wargames/data

[Install]
WantedBy=multi-user.target
EOF

# ────────────────────────────────────────────────────────────
# Start Service
# ────────────────────────────────────────────────────────────

systemctl daemon-reload
systemctl enable wargames
systemctl start wargames

# ────────────────────────────────────────────────────────────
# Health Check
# ────────────────────────────────────────────────────────────

for i in {1..30}; do
    if curl -f http://localhost:8080/health; then
        echo "Application started successfully"
        exit 0
    fi
    sleep 10
done

echo "Application failed to start"
exit 1
```

---

## 3. Deployment Procedures

### 3.1 Blue-Green Deployment Strategy

```yaml
# deployment/blue-green-deployment.yml

blue_green_deployment:
  overview: |
    Blue-green deployment eliminates downtime by running two identical
    production environments. At any time, only one (blue) serves production
    traffic. The other (green) is idle or used for testing the new version.

  steps:
    1_prepare_green:
      description: "Deploy new version to green environment"
      commands:
        - terraform workspace select green
        - terraform apply -var="app_version=${NEW_VERSION}"
        - ./scripts/run-smoke-tests.sh green

    2_validate_green:
      description: "Run comprehensive tests on green"
      checks:
        - health_endpoints: "All /health checks return 200"
        - database_migrations: "Schema version matches application"
        - integration_tests: "All integration tests pass"
        - performance_tests: "Response times within SLA"
        - security_scan: "No new vulnerabilities"

    3_switch_traffic:
      description: "Gradually shift traffic from blue to green"
      strategy: "Canary with 10% increments every 5 minutes"
      rollback_trigger: "Error rate > 1% or p99 latency > 2x baseline"
      commands:
        - ./scripts/shift-traffic.sh --target green --percent 10
        - sleep 300
        - ./scripts/check-metrics.sh
        - ./scripts/shift-traffic.sh --target green --percent 50
        - sleep 300
        - ./scripts/check-metrics.sh
        - ./scripts/shift-traffic.sh --target green --percent 100

    4_verify_production:
      description: "Monitor green environment under full load"
      duration: "30 minutes"
      metrics:
        - error_rate: "< 0.1%"
        - latency_p99: "< 2 seconds"
        - cpu_utilization: "< 70%"
        - memory_usage: "< 80%"

    5_decommission_blue:
      description: "Keep blue running for 24 hours for rollback"
      actions:
        - Scale blue ASG to minimum
        - Monitor green for anomalies
        - After 24h: terraform destroy blue infrastructure
```

### 3.2 Deployment Script

```bash
#!/bin/bash
# deployment/deploy.sh

set -e

ENVIRONMENT=$1
VERSION=$2
DEPLOY_STRATEGY=${3:-blue-green}

if [[ -z "$ENVIRONMENT" ]] || [[ -z "$VERSION" ]]; then
    echo "Usage: $0 <environment> <version> [strategy]"
    echo "Example: $0 production v1.2.3 blue-green"
    exit 1
fi

echo "════════════════════════════════════════════════════════════"
echo "  WarGames/JOSHUA Deployment"
echo "  Environment: $ENVIRONMENT"
echo "  Version: $VERSION"
echo "  Strategy: $DEPLOY_STRATEGY"
echo "════════════════════════════════════════════════════════════"

# ────────────────────────────────────────────────────────────
# Pre-deployment Checks
# ────────────────────────────────────────────────────────────

echo "➤ Running pre-deployment checks..."

# Check if version exists
if ! aws s3 ls "s3://wargames-artifacts/$ENVIRONMENT/wargames-joshua-$VERSION.tar.gz" &>/dev/null; then
    echo "✗ Version $VERSION not found in artifacts bucket"
    exit 1
fi

# Check database connectivity
if ! PGPASSWORD=$(aws secretsmanager get-secret-value --secret-id wargames/$ENVIRONMENT/database --query SecretString --output text | jq -r '.password') \
     psql -h $(aws secretsmanager get-secret-value --secret-id wargames/$ENVIRONMENT/database --query SecretString --output text | jq -r '.host') \
          -U $(aws secretsmanager get-secret-value --secret-id wargames/$ENVIRONMENT/database --query SecretString --output text | jq -r '.username') \
          -d wargames \
          -c "SELECT 1" &>/dev/null; then
    echo "✗ Cannot connect to database"
    exit 1
fi

echo "✓ Pre-deployment checks passed"

# ────────────────────────────────────────────────────────────
# Create Deployment Record
# ────────────────────────────────────────────────────────────

DEPLOYMENT_ID=$(uuidgen)
DEPLOYMENT_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat > "/tmp/deployment-$DEPLOYMENT_ID.json" <<EOF
{
  "deployment_id": "$DEPLOYMENT_ID",
  "environment": "$ENVIRONMENT",
  "version": "$VERSION",
  "strategy": "$DEPLOY_STRATEGY",
  "started_at": "$DEPLOYMENT_TIME",
  "initiated_by": "${USER}",
  "git_commit": "$(git rev-parse HEAD)"
}
EOF

aws s3 cp "/tmp/deployment-$DEPLOYMENT_ID.json" "s3://wargames-deployments/$ENVIRONMENT/$DEPLOYMENT_ID.json"

# ────────────────────────────────────────────────────────────
# Execute Deployment Strategy
# ────────────────────────────────────────────────────────────

case $DEPLOY_STRATEGY in
    blue-green)
        ./deployment/strategies/blue-green.sh "$ENVIRONMENT" "$VERSION" "$DEPLOYMENT_ID"
        ;;
    rolling)
        ./deployment/strategies/rolling.sh "$ENVIRONMENT" "$VERSION" "$DEPLOYMENT_ID"
        ;;
    canary)
        ./deployment/strategies/canary.sh "$ENVIRONMENT" "$VERSION" "$DEPLOYMENT_ID"
        ;;
    *)
        echo "✗ Unknown deployment strategy: $DEPLOY_STRATEGY"
        exit 1
        ;;
esac

# ────────────────────────────────────────────────────────────
# Post-deployment Verification
# ────────────────────────────────────────────────────────────

echo "➤ Running post-deployment verification..."

# Wait for instances to be healthy
sleep 60

# Run smoke tests
./tests/smoke-tests.sh "$ENVIRONMENT"

# Check metrics
./monitoring/check-deployment-metrics.sh "$ENVIRONMENT" "$DEPLOYMENT_ID"

echo "✓ Deployment completed successfully!"
echo ""
echo "Deployment ID: $DEPLOYMENT_ID"
echo "Monitor: https://console.aws.amazon.com/cloudwatch/home?region=us-east-1#dashboards:name=WarGames-$ENVIRONMENT"
```

---

## 4. Configuration Management

### 4.1 Configuration Hierarchy

```
Configuration Priority (highest to lowest):
┌─────────────────────────────────────────────────────────┐
│ 1. Environment Variables (runtime)                     │
│ 2. Secrets Manager (credentials, API keys)             │
│ 3. Parameter Store (feature flags, tuning params)      │
│ 4. Configuration Files (config/production.toml)        │
│ 5. Default Values (embedded in code)                   │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Configuration File Structure

```toml
# config/production.toml

[environment]
name = "production"
log_level = "info"
debug = false

[server]
host = "0.0.0.0"
port = 8080
workers = 4
keep_alive_timeout = 75
request_timeout = 300
max_connections = 10000

[database]
# Connection details fetched from Secrets Manager
max_connections = 20
min_connections = 5
connection_timeout = 30
idle_timeout = 600
max_lifetime = 1800

[database.pool]
statement_cache_capacity = 100

[redis]
# Connection details fetched from Secrets Manager
pool_size = 10
connection_timeout = 5
command_timeout = 10
max_retries = 3

[claude_api]
# API key fetched from Secrets Manager
base_url = "https://api.anthropic.com"
model = "claude-3-5-sonnet-20241022"
max_tokens = 8000
temperature = 0.1
timeout = 180
max_retries = 3
retry_delay = 5

[data_collection]
enabled = true
parallel_collectors = 10
collection_timeout = 300
min_sources_required = 5

[data_collection.schedule]
# Cron expression for automated assessments
daily_assessment = "0 6 * * *"  # 6 AM UTC daily
weekly_detailed = "0 8 * * 1"   # 8 AM UTC Mondays

[risk_calculation]
monte_carlo_iterations = 10000
bayesian_samples = 5000
confidence_threshold = 0.7

[visualization]
output_format = ["svg", "png"]
dpi = 300
color_scheme = "professional"

[storage]
assessments_bucket = "wargames-production-assessments"
reports_bucket = "wargames-production-reports"
retention_days = 2555  # 7 years

[monitoring]
metrics_interval = 60
health_check_interval = 30
performance_logging = true

[alerts]
enabled = true
channels = ["email", "slack", "pagerduty"]
severity_levels = ["warning", "severe", "critical"]

[alerts.thresholds]
seconds_to_midnight_critical = 90
seconds_to_midnight_warning = 300
data_quality_minimum = 0.7
collection_failure_threshold = 3

[security]
api_key_rotation_days = 90
session_timeout = 3600
max_login_attempts = 5
require_mfa = true

[rate_limiting]
requests_per_minute = 60
requests_per_hour = 1000
burst_size = 100
```

---

## 5. Service Management

### 5.1 Systemd Service Configuration

```ini
# /etc/systemd/system/wargames.service

[Unit]
Description=WarGames/JOSHUA Nuclear Risk Assessment System
Documentation=https://docs.wargames.internal
After=network-online.target postgresql.service redis.service
Wants=network-online.target

[Service]
Type=notify
User=wargames
Group=wargames
WorkingDirectory=/opt/wargames

# Environment configuration
EnvironmentFile=/opt/wargames/.env
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"

# Main process
ExecStartPre=/opt/wargames/bin/joshua check-health
ExecStart=/opt/wargames/bin/joshua server --config /opt/wargames/config/production.toml
ExecReload=/bin/kill -HUP $MAINPID
ExecStop=/bin/kill -TERM $MAINPID
TimeoutStopSec=30

# Restart policy
Restart=always
RestartSec=10
StartLimitInterval=60
StartLimitBurst=3

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true

# Allowed paths
ReadWritePaths=/var/log/wargames /opt/wargames/data
ReadOnlyPaths=/opt/wargames/config

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=wargames

[Install]
WantedBy=multi-user.target
```

### 5.2 Service Management Commands

```bash
# Start service
sudo systemctl start wargames

# Stop service
sudo systemctl stop wargames

# Restart service
sudo systemctl restart wargames

# Reload configuration (graceful)
sudo systemctl reload wargames

# Check status
sudo systemctl status wargames

# View logs
sudo journalctl -u wargames -f

# Enable auto-start on boot
sudo systemctl enable wargames

# Disable auto-start
sudo systemctl disable wargames
```

---

## 6. Monitoring and Health Checks

### 6.1 Health Check Endpoint

```rust
// Health check implementation
use axum::{extract::State, Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub checks: HealthChecks,
}

#[derive(Serialize, Deserialize)]
pub struct HealthChecks {
    pub database: ComponentHealth,
    pub redis: ComponentHealth,
    pub claude_api: ComponentHealth,
    pub disk_space: ComponentHealth,
}

#[derive(Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: String,
    pub latency_ms: Option<u64>,
    pub message: Option<String>,
}

pub async fn health_handler(
    State(app): State<Arc<AppState>>,
) -> Json<HealthResponse> {
    let start_time = std::time::Instant::now();

    // Check database
    let db_health = check_database(&app.db).await;

    // Check Redis
    let redis_health = check_redis(&app.redis).await;

    // Check Claude API
    let claude_health = check_claude_api(&app.claude_client).await;

    // Check disk space
    let disk_health = check_disk_space().await;

    // Overall status
    let all_healthy = db_health.status == "healthy" &&
                     redis_health.status == "healthy" &&
                     claude_health.status == "healthy" &&
                     disk_health.status == "healthy";

    Json(HealthResponse {
        status: if all_healthy { "healthy" } else { "degraded" }.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: app.start_time.elapsed().as_secs(),
        checks: HealthChecks {
            database: db_health,
            redis: redis_health,
            claude_api: claude_health,
            disk_space: disk_health,
        },
    })
}

async fn check_database(db: &sqlx::PgPool) -> ComponentHealth {
    let start = std::time::Instant::now();

    match sqlx::query("SELECT 1").fetch_one(db).await {
        Ok(_) => ComponentHealth {
            status: "healthy".to_string(),
            latency_ms: Some(start.elapsed().as_millis() as u64),
            message: None,
        },
        Err(e) => ComponentHealth {
            status: "unhealthy".to_string(),
            latency_ms: None,
            message: Some(e.to_string()),
        },
    }
}
```

### 6.2 CloudWatch Dashboard

```json
{
  "widgets": [
    {
      "type": "metric",
      "properties": {
        "metrics": [
          ["AWS/ApplicationELB", "TargetResponseTime", {"stat": "Average"}],
          [".", ".", {"stat": "p99"}]
        ],
        "period": 300,
        "stat": "Average",
        "region": "us-east-1",
        "title": "Response Time",
        "yAxis": {
          "left": {
            "min": 0
          }
        }
      }
    },
    {
      "type": "metric",
      "properties": {
        "metrics": [
          ["AWS/ApplicationELB", "HTTPCode_Target_2XX_Count", {"stat": "Sum"}],
          [".", "HTTPCode_Target_4XX_Count", {"stat": "Sum"}],
          [".", "HTTPCode_Target_5XX_Count", {"stat": "Sum"}]
        ],
        "period": 300,
        "stat": "Sum",
        "region": "us-east-1",
        "title": "HTTP Response Codes"
      }
    },
    {
      "type": "metric",
      "properties": {
        "metrics": [
          ["AWS/RDS", "CPUUtilization"],
          [".", "DatabaseConnections"],
          [".", "FreeableMemory"]
        ],
        "period": 300,
        "stat": "Average",
        "region": "us-east-1",
        "title": "Database Health"
      }
    }
  ]
}
```

---

## 7. Operational Runbooks

### 7.1 Common Operational Tasks

#### Daily Health Check

```bash
#!/bin/bash
# operations/daily-health-check.sh

echo "═══════════════════════════════════════════════════════════"
echo "  WarGames/JOSHUA Daily Health Check"
echo "  $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
echo "═══════════════════════════════════════════════════════════"

# Check application health
echo "➤ Checking application health..."
HEALTH=$(curl -s https://wargames.prod.internal/health)
if echo "$HEALTH" | jq -e '.status == "healthy"' > /dev/null; then
    echo "✓ Application is healthy"
else
    echo "✗ Application health check failed!"
    echo "$HEALTH" | jq '.'
    exit 1
fi

# Check database
echo "➤ Checking database..."
DB_CONNECTIONS=$(psql -h $DB_HOST -U $DB_USER -d wargames -t -c "SELECT count(*) FROM pg_stat_activity;")
echo "  Active connections: $DB_CONNECTIONS"

# Check disk space
echo "➤ Checking disk space..."
df -h /var/log /opt/wargames | tail -n +2

# Check recent assessments
echo "➤ Checking recent assessments..."
RECENT_COUNT=$(psql -h $DB_HOST -U $DB_USER -d wargames -t -c "SELECT count(*) FROM assessments WHERE assessment_date >= NOW() - INTERVAL '24 hours';")
echo "  Assessments in last 24h: $RECENT_COUNT"

# Check error logs
echo "➤ Checking error logs..."
ERROR_COUNT=$(journalctl -u wargames --since "24 hours ago" | grep -i error | wc -l)
echo "  Errors in last 24h: $ERROR_COUNT"

echo "═══════════════════════════════════════════════════════════"
echo "  Daily health check completed"
echo "═══════════════════════════════════════════════════════════"
```

#### Emergency Restart Procedure

```bash
#!/bin/bash
# operations/emergency-restart.sh

set -e

echo "⚠️  EMERGENCY RESTART PROCEDURE"
echo "This will restart the WarGames/JOSHUA service"
read -p "Continue? (yes/no): " confirm

if [[ "$confirm" != "yes" ]]; then
    echo "Aborted"
    exit 0
fi

# Create incident record
INCIDENT_ID=$(uuidgen)
echo "Incident ID: $INCIDENT_ID"

# Capture pre-restart state
echo "➤ Capturing pre-restart diagnostics..."
journalctl -u wargames --since "1 hour ago" > "/tmp/wargames-logs-$INCIDENT_ID.txt"
systemctl status wargames > "/tmp/wargames-status-$INCIDENT_ID.txt"

# Restart service
echo "➤ Restarting service..."
sudo systemctl restart wargames

# Wait for service to be ready
echo "➤ Waiting for service to be healthy..."
for i in {1..30}; do
    if curl -f http://localhost:8080/health &>/dev/null; then
        echo "✓ Service is healthy"
        break
    fi
    if [[ $i -eq 30 ]]; then
        echo "✗ Service failed to become healthy after restart!"
        exit 1
    fi
    sleep 10
done

# Upload diagnostics
aws s3 cp "/tmp/wargames-logs-$INCIDENT_ID.txt" "s3://wargames-incidents/$INCIDENT_ID/"
aws s3 cp "/tmp/wargames-status-$INCIDENT_ID.txt" "s3://wargames-incidents/$INCIDENT_ID/"

echo "✓ Emergency restart completed"
echo "Incident ID: $INCIDENT_ID"
```

---

## 8. Maintenance Procedures

### 8.1 Database Maintenance

```sql
-- Weekly maintenance tasks
-- Run during maintenance window (Sunday 04:00-05:00 UTC)

-- Vacuum and analyze
VACUUM ANALYZE;

-- Reindex critical tables
REINDEX TABLE CONCURRENTLY assessments;
REINDEX TABLE CONCURRENTLY risk_factors;
REINDEX TABLE CONCURRENTLY collected_data;

-- Update statistics
ANALYZE assessments;
ANALYZE risk_factors;
ANALYZE collected_data;

-- Check for bloat
SELECT
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) AS size,
    n_live_tup,
    n_dead_tup,
    ROUND(100.0 * n_dead_tup / NULLIF(n_live_tup + n_dead_tup, 0), 2) AS dead_pct
FROM pg_stat_user_tables
WHERE n_dead_tup > 1000
ORDER BY n_dead_tup DESC
LIMIT 10;

-- Archive old data
-- Move assessments older than 1 year to archive table
INSERT INTO assessments_archive
SELECT * FROM assessments
WHERE assessment_date < NOW() - INTERVAL '1 year'
  AND id NOT IN (SELECT id FROM assessments_archive);

DELETE FROM assessments
WHERE assessment_date < NOW() - INTERVAL '1 year';
```

### 8.2 Log Rotation

```bash
# /etc/logrotate.d/wargames

/var/log/wargames/*.log {
    daily
    rotate 30
    compress
    delaycompress
    notifempty
    create 0644 wargames wargames
    sharedscripts
    postrotate
        systemctl reload wargames > /dev/null 2>&1 || true
    endscript
}
```

---

## 9. Troubleshooting Guide

### 9.1 Common Issues and Solutions

#### Issue: High CPU Usage

```bash
# Diagnosis
1. Check current CPU usage:
   top -u wargames

2. Identify CPU-intensive processes:
   ps aux --sort=-%cpu | grep wargames | head -20

3. Check for runaway assessments:
   psql -c "SELECT * FROM assessments WHERE created_at > NOW() - INTERVAL '1 hour' ORDER BY analysis_duration_seconds DESC;"

# Solutions
A. If Monte Carlo simulations taking too long:
   - Reduce iterations in config: monte_carlo_iterations = 5000
   - Scale horizontally: increase ASG desired capacity

B. If data collection hanging:
   - Check collector timeouts
   - Review slow data sources
   - Increase parallel_collectors limit

C. If database queries slow:
   - Check pg_stat_statements for slow queries
   - Add missing indexes
   - Optimize query plans
```

#### Issue: Memory Leak

```bash
# Diagnosis
1. Monitor memory over time:
   watch -n 5 'ps aux | grep joshua | awk "{print \$6, \$11}"'

2. Check for memory growth pattern:
   # If memory consistently increases without leveling off = leak

3. Get memory profile:
   # Enable jemalloc heap profiling
   MALLOC_CONF=prof:true,prof_prefix:/tmp/jeprof ./joshua server

# Solutions
A. Immediate: Restart service
   sudo systemctl restart wargames

B. Long-term:
   - Review recent code changes
   - Check for unbounded caches
   - Verify proper Drop implementations
   - Use valgrind/heaptrack for diagnosis
```

#### Issue: Database Connection Exhaustion

```bash
# Diagnosis
psql -c "SELECT count(*), state FROM pg_stat_activity GROUP BY state;"

# Solutions
1. Increase connection pool:
   # In config/production.toml
   [database]
   max_connections = 50  # Increase from 20

2. Find connection leaks:
   psql -c "SELECT pid, usename, application_name, state, query_start, query FROM pg_stat_activity WHERE state <> 'idle' ORDER BY query_start;"

3. Kill long-running queries:
   psql -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE state = 'active' AND query_start < NOW() - INTERVAL '10 minutes';"
```

---

## 10. Scaling Procedures

### 10.1 Horizontal Scaling (Application Tier)

```bash
#!/bin/bash
# operations/scale-application.sh

ENVIRONMENT=$1
DESIRED_CAPACITY=$2

if [[ -z "$ENVIRONMENT" ]] || [[ -z "$DESIRED_CAPACITY" ]]; then
    echo "Usage: $0 <environment> <desired_capacity>"
    exit 1
fi

ASG_NAME="wargames-${ENVIRONMENT}-asg"

echo "Scaling $ASG_NAME to $DESIRED_CAPACITY instances..."

aws autoscaling set-desired-capacity \
    --auto-scaling-group-name "$ASG_NAME" \
    --desired-capacity "$DESIRED_CAPACITY"

echo "Waiting for instances to be InService..."
aws autoscaling wait instance-in-service \
    --auto-scaling-group-name "$ASG_NAME"

echo "✓ Scaling completed"
aws autoscaling describe-auto-scaling-groups \
    --auto-scaling-group-names "$ASG_NAME" \
    --query 'AutoScalingGroups[0].{Desired:DesiredCapacity,Current:length(Instances[?HealthStatus==`Healthy`])}'
```

### 10.2 Vertical Scaling (Database)

```bash
#!/bin/bash
# operations/scale-database.sh

ENVIRONMENT=$1
NEW_INSTANCE_CLASS=$2

DB_IDENTIFIER="wargames-${ENVIRONMENT}"

echo "⚠️  Database Scaling - Requires Downtime!"
echo "This will modify the database instance class"
echo "Current: $(aws rds describe-db-instances --db-instance-identifier $DB_IDENTIFIER --query 'DBInstances[0].DBInstanceClass' --output text)"
echo "New: $NEW_INSTANCE_CLASS"
read -p "Continue? (yes/no): " confirm

if [[ "$confirm" != "yes" ]]; then
    echo "Aborted"
    exit 0
fi

# Create snapshot before modification
SNAPSHOT_ID="${DB_IDENTIFIER}-pre-scale-$(date +%Y%m%d-%H%M%S)"
echo "Creating snapshot: $SNAPSHOT_ID"
aws rds create-db-snapshot \
    --db-instance-identifier "$DB_IDENTIFIER" \
    --db-snapshot-identifier "$SNAPSHOT_ID"

aws rds wait db-snapshot-completed \
    --db-snapshot-identifier "$SNAPSHOT_ID"

# Modify instance class
echo "Modifying database instance class..."
aws rds modify-db-instance \
    --db-instance-identifier "$DB_IDENTIFIER" \
    --db-instance-class "$NEW_INSTANCE_CLASS" \
    --apply-immediately

echo "Waiting for modification to complete..."
aws rds wait db-instance-available \
    --db-instance-identifier "$DB_IDENTIFIER"

echo "✓ Database scaling completed"
```

---

## Conclusion

This deployment and operations guide provides comprehensive procedures for deploying, configuring, and operating the WarGames/JOSHUA system in production. By following these documented procedures, operations teams can ensure reliable, secure, and high-performance operation of this critical nuclear risk assessment system.

### Key Operational Principles

1. **Automation First**: Automate all routine operations
2. **Documentation**: Keep runbooks updated with actual procedures
3. **Monitoring**: Comprehensive observability at all layers
4. **Redundancy**: Multiple availability zones, read replicas, backups
5. **Testing**: Test all procedures in staging before production
6. **Security**: Principle of least privilege, defense in depth
7. **Disaster Recovery**: Tested backup and recovery procedures

**Operations is not a one-time event—it's an ongoing discipline of vigilance and continuous improvement.**

*"In production, hope is not a strategy. Preparation is."*
