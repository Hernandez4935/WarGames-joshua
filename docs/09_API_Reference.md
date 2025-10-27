# WarGames/JOSHUA: Complete API Reference
## REST API Documentation for Nuclear Risk Assessment System
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [Overview](#1-overview)
2. [Authentication](#2-authentication)
3. [Rate Limiting](#3-rate-limiting)
4. [Error Handling](#4-error-handling)
5. [Assessment Endpoints](#5-assessment-endpoints)
6. [Data Collection Endpoints](#6-data-collection-endpoints)
7. [Risk Analysis Endpoints](#7-risk-analysis-endpoints)
8. [Historical Data Endpoints](#8-historical-data-endpoints)
9. [Reporting Endpoints](#9-reporting-endpoints)
10. [Administration Endpoints](#10-administration-endpoints)
11. [Webhooks](#11-webhooks)
12. [SDK Examples](#12-sdk-examples)

---

## 1. Overview

### 1.1 Base URL

```
Production:  https://api.wargames-joshua.example.com/v1
Staging:     https://api-staging.wargames-joshua.example.com/v1
Development: http://localhost:8080/v1
```

### 1.2 API Versioning

The API uses URL-based versioning:
- Current version: `v1`
- Version format: `/v{major_version}/`
- Breaking changes require new major version
- Backward-compatible changes increment minor version in response headers

### 1.3 Request/Response Format

**Content Type:**
- Requests: `application/json`
- Responses: `application/json`
- Reports: `application/pdf` or `application/json`

**Request Headers:**
```http
Content-Type: application/json
Authorization: Bearer <jwt_token>
X-API-Key: <api_key>
User-Agent: YourApp/1.0.0
Accept: application/json
```

**Response Format:**
```json
{
  "status": "success",
  "data": {
    // Response payload
  },
  "metadata": {
    "timestamp": "2025-10-27T12:34:56Z",
    "request_id": "req_1234567890abcdef",
    "api_version": "1.2.0"
  }
}
```

### 1.4 HTTP Status Codes

| Code | Meaning | Usage |
|------|---------|-------|
| 200 | OK | Successful GET, PUT, PATCH |
| 201 | Created | Successful POST with resource creation |
| 202 | Accepted | Request accepted for async processing |
| 204 | No Content | Successful DELETE |
| 400 | Bad Request | Invalid request parameters |
| 401 | Unauthorized | Missing or invalid authentication |
| 403 | Forbidden | Authenticated but insufficient permissions |
| 404 | Not Found | Resource doesn't exist |
| 409 | Conflict | Resource state conflict |
| 422 | Unprocessable Entity | Validation error |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |
| 503 | Service Unavailable | Temporary unavailability |

---

## 2. Authentication

### 2.1 Authentication Methods

**JWT Bearer Token (Primary):**
```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**API Key (Secondary):**
```http
X-API-Key: wg_live_1234567890abcdefghijklmnopqrstuvwxyz
```

### 2.2 Obtaining JWT Token

**Endpoint:** `POST /auth/login`

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securePassword123!",
  "mfa_code": "123456"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "user": {
      "id": "usr_1234567890",
      "email": "user@example.com",
      "roles": ["analyst", "viewer"]
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:34:56Z",
    "request_id": "req_auth_001"
  }
}
```

### 2.3 Token Refresh

**Endpoint:** `POST /auth/refresh`

**Request:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:** Same format as login response with new tokens.

### 2.4 API Key Management

**Generate API Key:** `POST /auth/api-keys`

**Request:**
```json
{
  "name": "Production Integration Key",
  "scopes": ["assessments:read", "data:read"],
  "expires_at": "2026-10-27T00:00:00Z"
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": "key_1234567890",
    "key": "wg_live_1234567890abcdefghijklmnopqrstuvwxyz",
    "name": "Production Integration Key",
    "scopes": ["assessments:read", "data:read"],
    "created_at": "2025-10-27T12:34:56Z",
    "expires_at": "2026-10-27T00:00:00Z"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:34:56Z",
    "request_id": "req_key_001"
  }
}
```

**⚠️ CRITICAL:** Store the API key securely. It's only shown once and cannot be retrieved later.

---

## 3. Rate Limiting

### 3.1 Rate Limit Tiers

| Tier | Requests/Minute | Requests/Hour | Requests/Day |
|------|----------------|---------------|--------------|
| Free | 10 | 100 | 1,000 |
| Basic | 60 | 1,000 | 10,000 |
| Professional | 300 | 10,000 | 100,000 |
| Enterprise | Custom | Custom | Custom |

### 3.2 Rate Limit Headers

```http
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1698412800
X-RateLimit-Window: 60
```

### 3.3 Rate Limit Exceeded Response

**Status:** `429 Too Many Requests`

```json
{
  "status": "error",
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Please retry after 45 seconds.",
    "details": {
      "limit": 60,
      "window": "1 minute",
      "retry_after": 45
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:34:56Z",
    "request_id": "req_rate_001"
  }
}
```

---

## 4. Error Handling

### 4.1 Error Response Format

```json
{
  "status": "error",
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": {
      "field": "Specific error details",
      "validation_errors": []
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:34:56Z",
    "request_id": "req_err_001",
    "trace_id": "trace_1234567890"
  }
}
```

### 4.2 Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_REQUEST` | 400 | Malformed request |
| `VALIDATION_ERROR` | 422 | Request validation failed |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `CONFLICT` | 409 | Resource state conflict |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |
| `SERVICE_UNAVAILABLE` | 503 | Temporary unavailability |
| `CLAUDE_API_ERROR` | 502 | Claude API unavailable |
| `DATA_COLLECTION_FAILED` | 503 | Data collection failure |

### 4.3 Validation Error Example

```json
{
  "status": "error",
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Request validation failed",
    "details": {
      "validation_errors": [
        {
          "field": "start_date",
          "message": "Must be a valid ISO 8601 date",
          "value": "2025-13-01"
        },
        {
          "field": "risk_threshold",
          "message": "Must be between 0.0 and 1.0",
          "value": 1.5
        }
      ]
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:34:56Z",
    "request_id": "req_val_001"
  }
}
```

---

## 5. Assessment Endpoints

### 5.1 Trigger New Assessment

**Endpoint:** `POST /assessments`

**Description:** Triggers a new nuclear risk assessment. This is an asynchronous operation.

**Request:**
```json
{
  "trigger_source": "manual",
  "priority": "high",
  "include_monte_carlo": true,
  "monte_carlo_iterations": 10000,
  "confidence_threshold": 0.7,
  "notification_channels": ["email", "slack"]
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "assessment_id": "assess_1234567890",
    "status": "pending",
    "created_at": "2025-10-27T12:34:56Z",
    "estimated_completion": "2025-10-27T12:39:56Z",
    "priority": "high"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:34:56Z",
    "request_id": "req_assess_001"
  }
}
```

### 5.2 Get Assessment Status

**Endpoint:** `GET /assessments/{assessment_id}`

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": "assess_1234567890",
    "status": "completed",
    "created_at": "2025-10-27T12:34:56Z",
    "completed_at": "2025-10-27T12:39:32Z",
    "processing_time_seconds": 276,
    "result": {
      "seconds_to_midnight": 90,
      "risk_level": "critical",
      "confidence": 0.87,
      "primary_risk_factors": [
        {
          "category": "regional_conflicts",
          "weight": 0.35,
          "score": 0.89,
          "description": "Escalating tensions in Eastern Europe"
        },
        {
          "category": "nuclear_modernization",
          "weight": 0.25,
          "score": 0.82,
          "description": "Accelerated weapons development programs"
        }
      ],
      "trend": {
        "direction": "deteriorating",
        "change_from_previous": -30,
        "velocity": -2.5
      }
    },
    "data_quality": {
      "sources_collected": 42,
      "sources_failed": 3,
      "overall_quality_score": 0.91,
      "reliability": "high"
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_status_001"
  }
}
```

**Status Values:**
- `pending` - Assessment queued
- `collecting_data` - Gathering data from sources
- `analyzing` - Claude API analysis in progress
- `calculating` - Risk calculation engine running
- `generating_report` - Creating output documents
- `completed` - Assessment finished successfully
- `failed` - Assessment failed (see error details)

### 5.3 List Recent Assessments

**Endpoint:** `GET /assessments`

**Query Parameters:**
- `limit` (integer, default: 10, max: 100)
- `offset` (integer, default: 0)
- `status` (string: "completed", "failed", "pending")
- `start_date` (ISO 8601 date)
- `end_date` (ISO 8601 date)
- `sort` (string: "date", "risk_score", default: "date")
- `order` (string: "asc", "desc", default: "desc")

**Example:** `GET /assessments?limit=20&status=completed&sort=date&order=desc`

**Response:**
```json
{
  "status": "success",
  "data": {
    "assessments": [
      {
        "id": "assess_1234567890",
        "created_at": "2025-10-27T12:34:56Z",
        "status": "completed",
        "seconds_to_midnight": 90,
        "risk_level": "critical",
        "confidence": 0.87
      },
      // ... more assessments
    ],
    "pagination": {
      "total": 156,
      "limit": 20,
      "offset": 0,
      "has_more": true
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_list_001"
  }
}
```

### 5.4 Get Latest Assessment

**Endpoint:** `GET /assessments/latest`

**Description:** Returns the most recent completed assessment.

**Response:** Same format as "Get Assessment Status" endpoint.

### 5.5 Cancel Assessment

**Endpoint:** `DELETE /assessments/{assessment_id}`

**Description:** Cancels a pending or in-progress assessment.

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": "assess_1234567890",
    "status": "cancelled",
    "cancelled_at": "2025-10-27T12:45:00Z"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:45:00Z",
    "request_id": "req_cancel_001"
  }
}
```

---

## 6. Data Collection Endpoints

### 6.1 Get Data Sources Status

**Endpoint:** `GET /data/sources`

**Response:**
```json
{
  "status": "success",
  "data": {
    "sources": [
      {
        "id": "src_reuters",
        "name": "Reuters News API",
        "type": "news",
        "status": "active",
        "reliability": 0.95,
        "last_successful_collection": "2025-10-27T12:30:00Z",
        "total_collections": 5432,
        "failed_collections": 12,
        "average_latency_ms": 243,
        "data_points_collected": 12847
      },
      {
        "id": "src_sipri",
        "name": "SIPRI Database",
        "type": "research",
        "status": "active",
        "reliability": 0.98,
        "last_successful_collection": "2025-10-27T10:00:00Z",
        "total_collections": 1234,
        "failed_collections": 2,
        "average_latency_ms": 567,
        "data_points_collected": 3456
      },
      {
        "id": "src_twitter",
        "name": "Twitter Geopolitical Feed",
        "type": "social",
        "status": "degraded",
        "reliability": 0.72,
        "last_successful_collection": "2025-10-27T11:45:00Z",
        "total_collections": 8765,
        "failed_collections": 234,
        "average_latency_ms": 1234,
        "data_points_collected": 45678,
        "error": "Rate limit throttling"
      }
    ],
    "summary": {
      "total_sources": 18,
      "active": 15,
      "degraded": 2,
      "failed": 1,
      "overall_health": 0.89
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_sources_001"
  }
}
```

### 6.2 Trigger Manual Data Collection

**Endpoint:** `POST /data/collect`

**Request:**
```json
{
  "sources": ["src_reuters", "src_sipri"],
  "priority": "high",
  "force_refresh": true
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "collection_id": "collect_1234567890",
    "status": "in_progress",
    "sources": ["src_reuters", "src_sipri"],
    "started_at": "2025-10-27T12:40:00Z",
    "estimated_completion": "2025-10-27T12:42:00Z"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_collect_001"
  }
}
```

### 6.3 Get Collected Data

**Endpoint:** `GET /data/points`

**Query Parameters:**
- `source_id` (string, optional)
- `category` (string: "news", "research", "government", "social")
- `start_date` (ISO 8601)
- `end_date` (ISO 8601)
- `limit` (integer, default: 50, max: 500)
- `offset` (integer, default: 0)

**Example:** `GET /data/points?category=news&start_date=2025-10-26T00:00:00Z&limit=100`

**Response:**
```json
{
  "status": "success",
  "data": {
    "data_points": [
      {
        "id": "dp_1234567890",
        "source": "Reuters News API",
        "category": "news",
        "title": "Nuclear modernization programs accelerate in major powers",
        "content": "...",
        "url": "https://reuters.com/article/...",
        "published_at": "2025-10-27T10:00:00Z",
        "collected_at": "2025-10-27T10:05:32Z",
        "relevance_score": 0.94,
        "sentiment": {
          "score": -0.67,
          "magnitude": 0.82,
          "label": "negative"
        },
        "entities": [
          {"name": "Russia", "type": "country"},
          {"name": "United States", "type": "country"},
          {"name": "ICBM", "type": "weapon_system"}
        ],
        "risk_indicators": [
          "nuclear_modernization",
          "arms_race"
        ]
      }
      // ... more data points
    ],
    "pagination": {
      "total": 8432,
      "limit": 100,
      "offset": 0,
      "has_more": true
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_data_001"
  }
}
```

---

## 7. Risk Analysis Endpoints

### 7.1 Get Current Risk Score

**Endpoint:** `GET /risk/current`

**Response:**
```json
{
  "status": "success",
  "data": {
    "seconds_to_midnight": 90,
    "risk_level": "critical",
    "confidence": 0.87,
    "last_updated": "2025-10-27T12:39:32Z",
    "change_from_previous": {
      "seconds": -30,
      "percent": -25.0,
      "direction": "deteriorating"
    },
    "risk_factors": {
      "nuclear_arsenal_changes": 0.78,
      "arms_control_breakdown": 0.85,
      "regional_conflicts": 0.89,
      "leadership_instability": 0.56,
      "technical_incidents": 0.45,
      "communication_failures": 0.67,
      "emerging_tech_risks": 0.71
    },
    "critical_indicators": [
      {
        "indicator": "Regional military mobilization",
        "severity": "high",
        "confidence": 0.92
      },
      {
        "indicator": "Deteriorating diplomatic relations",
        "severity": "high",
        "confidence": 0.88
      }
    ]
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_risk_001"
  }
}
```

### 7.2 Get Risk Trend

**Endpoint:** `GET /risk/trend`

**Query Parameters:**
- `period` (string: "24h", "7d", "30d", "90d", "1y", default: "30d")
- `resolution` (string: "hourly", "daily", "weekly", default: "daily")

**Example:** `GET /risk/trend?period=90d&resolution=weekly`

**Response:**
```json
{
  "status": "success",
  "data": {
    "period": "90d",
    "resolution": "weekly",
    "data_points": [
      {
        "timestamp": "2025-07-29T00:00:00Z",
        "seconds_to_midnight": 120,
        "risk_level": "elevated",
        "confidence": 0.85
      },
      {
        "timestamp": "2025-08-05T00:00:00Z",
        "seconds_to_midnight": 115,
        "risk_level": "elevated",
        "confidence": 0.87
      },
      // ... more data points
      {
        "timestamp": "2025-10-27T00:00:00Z",
        "seconds_to_midnight": 90,
        "risk_level": "critical",
        "confidence": 0.87
      }
    ],
    "statistics": {
      "mean": 108.5,
      "median": 110,
      "std_dev": 12.3,
      "min": 90,
      "max": 125,
      "trend_direction": "deteriorating",
      "trend_strength": 0.78
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_trend_001"
  }
}
```

### 7.3 Run Risk Simulation

**Endpoint:** `POST /risk/simulate`

**Description:** Runs Monte Carlo simulation to project future risk scenarios.

**Request:**
```json
{
  "simulation_type": "monte_carlo",
  "iterations": 10000,
  "time_horizon_days": 90,
  "scenarios": [
    {
      "name": "baseline",
      "adjustments": {}
    },
    {
      "name": "escalation",
      "adjustments": {
        "regional_conflicts": 0.9,
        "leadership_instability": 0.8
      }
    },
    {
      "name": "de-escalation",
      "adjustments": {
        "arms_control_breakdown": 0.3,
        "regional_conflicts": 0.4
      }
    }
  ],
  "confidence_level": 0.95
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "simulation_id": "sim_1234567890",
    "completed_at": "2025-10-27T12:45:00Z",
    "iterations": 10000,
    "time_horizon_days": 90,
    "scenarios": [
      {
        "name": "baseline",
        "results": {
          "mean_seconds": 85,
          "median_seconds": 87,
          "p5": 65,
          "p95": 105,
          "probability_critical": 0.68,
          "probability_severe": 0.24,
          "probability_elevated": 0.08
        }
      },
      {
        "name": "escalation",
        "results": {
          "mean_seconds": 62,
          "median_seconds": 60,
          "p5": 40,
          "p95": 85,
          "probability_critical": 0.92,
          "probability_severe": 0.07,
          "probability_elevated": 0.01
        }
      },
      {
        "name": "de-escalation",
        "results": {
          "mean_seconds": 125,
          "median_seconds": 128,
          "p5": 95,
          "p95": 155,
          "probability_critical": 0.12,
          "probability_severe": 0.35,
          "probability_elevated": 0.53
        }
      }
    ],
    "visualization_url": "https://api.wargames-joshua.example.com/visualizations/sim_1234567890.png"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:45:00Z",
    "request_id": "req_sim_001"
  }
}
```

---

## 8. Historical Data Endpoints

### 8.1 Get Historical Assessments

**Endpoint:** `GET /history/assessments`

**Query Parameters:**
- `start_date` (ISO 8601, required)
- `end_date` (ISO 8601, required)
- `include_data_points` (boolean, default: false)
- `include_analysis` (boolean, default: false)

**Example:** `GET /history/assessments?start_date=2025-01-01T00:00:00Z&end_date=2025-10-27T23:59:59Z`

**Response:**
```json
{
  "status": "success",
  "data": {
    "assessments": [
      {
        "id": "assess_0123456789",
        "timestamp": "2025-01-01T06:00:00Z",
        "seconds_to_midnight": 150,
        "risk_level": "elevated",
        "confidence": 0.82
      },
      // ... more assessments
    ],
    "statistics": {
      "total_assessments": 298,
      "mean_risk": 112.5,
      "trend": "deteriorating",
      "critical_periods": [
        {
          "start": "2025-08-15T00:00:00Z",
          "end": "2025-09-05T00:00:00Z",
          "reason": "Regional military escalation"
        }
      ]
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_hist_001"
  }
}
```

### 8.2 Compare Assessments

**Endpoint:** `POST /history/compare`

**Request:**
```json
{
  "assessment_ids": [
    "assess_1234567890",
    "assess_0987654321"
  ]
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "comparisons": [
      {
        "assessment_id": "assess_1234567890",
        "timestamp": "2025-10-27T12:39:32Z",
        "seconds_to_midnight": 90,
        "risk_factors": {
          "nuclear_arsenal_changes": 0.78,
          "arms_control_breakdown": 0.85,
          "regional_conflicts": 0.89
        }
      },
      {
        "assessment_id": "assess_0987654321",
        "timestamp": "2025-09-27T12:39:32Z",
        "seconds_to_midnight": 120,
        "risk_factors": {
          "nuclear_arsenal_changes": 0.65,
          "arms_control_breakdown": 0.72,
          "regional_conflicts": 0.78
        }
      }
    ],
    "differences": {
      "seconds_to_midnight": -30,
      "percent_change": -25.0,
      "risk_factor_changes": {
        "nuclear_arsenal_changes": 0.13,
        "arms_control_breakdown": 0.13,
        "regional_conflicts": 0.11
      },
      "major_changes": [
        {
          "factor": "regional_conflicts",
          "change": 0.11,
          "description": "Significant escalation in Eastern Europe"
        }
      ]
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_compare_001"
  }
}
```

---

## 9. Reporting Endpoints

### 9.1 Generate Report

**Endpoint:** `POST /reports`

**Request:**
```json
{
  "assessment_id": "assess_1234567890",
  "report_type": "executive_summary",
  "format": "pdf",
  "include_visualizations": true,
  "include_raw_data": false,
  "language": "en"
}
```

**Report Types:**
- `executive_summary` - High-level summary for leadership
- `technical_analysis` - Detailed technical report
- `trend_analysis` - Historical trend analysis
- `scenario_comparison` - Monte Carlo scenario results
- `data_quality` - Data source quality report

**Response:**
```json
{
  "status": "success",
  "data": {
    "report_id": "rpt_1234567890",
    "status": "generating",
    "estimated_completion": "2025-10-27T12:42:00Z"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_report_001"
  }
}
```

### 9.2 Get Report Status

**Endpoint:** `GET /reports/{report_id}`

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": "rpt_1234567890",
    "status": "completed",
    "created_at": "2025-10-27T12:40:00Z",
    "completed_at": "2025-10-27T12:41:32Z",
    "report_type": "executive_summary",
    "format": "pdf",
    "file_size_bytes": 2458672,
    "download_url": "https://api.wargames-joshua.example.com/reports/rpt_1234567890/download",
    "expires_at": "2025-10-28T12:41:32Z"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:42:00Z",
    "request_id": "req_report_status_001"
  }
}
```

### 9.3 Download Report

**Endpoint:** `GET /reports/{report_id}/download`

**Response:** Binary content with appropriate `Content-Type` header.

```http
HTTP/1.1 200 OK
Content-Type: application/pdf
Content-Disposition: attachment; filename="nuclear-risk-assessment-2025-10-27.pdf"
Content-Length: 2458672

[PDF binary content]
```

### 9.4 List Reports

**Endpoint:** `GET /reports`

**Query Parameters:**
- `assessment_id` (string, optional)
- `report_type` (string, optional)
- `start_date` (ISO 8601, optional)
- `end_date` (ISO 8601, optional)
- `limit` (integer, default: 20)
- `offset` (integer, default: 0)

---

## 10. Administration Endpoints

### 10.1 System Health

**Endpoint:** `GET /admin/health`

**Response:**
```json
{
  "status": "success",
  "data": {
    "overall_status": "healthy",
    "components": {
      "api": {
        "status": "healthy",
        "response_time_ms": 12,
        "uptime_seconds": 8654321
      },
      "database": {
        "status": "healthy",
        "connections": 12,
        "max_connections": 100,
        "query_time_ms": 8
      },
      "redis": {
        "status": "healthy",
        "used_memory_mb": 234,
        "hit_rate": 0.92
      },
      "claude_api": {
        "status": "healthy",
        "last_successful_call": "2025-10-27T12:39:00Z",
        "average_latency_ms": 1234
      },
      "data_collectors": {
        "status": "degraded",
        "active_sources": 15,
        "failed_sources": 3,
        "collection_success_rate": 0.83
      }
    },
    "version": "1.2.0",
    "environment": "production"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_health_001"
  }
}
```

### 10.2 System Metrics

**Endpoint:** `GET /admin/metrics`

**Authorization:** Requires `admin` role.

**Response:**
```json
{
  "status": "success",
  "data": {
    "assessments": {
      "total": 5432,
      "completed_today": 3,
      "failed_today": 0,
      "average_duration_seconds": 276,
      "success_rate": 0.998
    },
    "data_collection": {
      "data_points_collected_today": 8765,
      "sources_active": 15,
      "sources_failed": 3,
      "collection_success_rate": 0.83
    },
    "api": {
      "requests_today": 125434,
      "average_response_time_ms": 45,
      "p95_response_time_ms": 123,
      "p99_response_time_ms": 234,
      "error_rate": 0.002
    },
    "storage": {
      "database_size_gb": 34.5,
      "s3_storage_gb": 123.4,
      "total_assessments": 5432,
      "total_data_points": 2345678
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_metrics_001"
  }
}
```

### 10.3 Audit Log

**Endpoint:** `GET /admin/audit`

**Authorization:** Requires `admin` or `auditor` role.

**Query Parameters:**
- `user_id` (string, optional)
- `action` (string, optional)
- `resource` (string, optional)
- `start_date` (ISO 8601, optional)
- `end_date` (ISO 8601, optional)
- `limit` (integer, default: 100)
- `offset` (integer, default: 0)

**Response:**
```json
{
  "status": "success",
  "data": {
    "audit_entries": [
      {
        "id": "audit_1234567890",
        "timestamp": "2025-10-27T12:39:32Z",
        "user_id": "usr_0987654321",
        "user_email": "analyst@example.com",
        "action": "assessment.create",
        "resource_type": "assessment",
        "resource_id": "assess_1234567890",
        "ip_address": "203.0.113.42",
        "user_agent": "WarGamesClient/1.0.0",
        "status": "success",
        "details": {
          "trigger_source": "manual",
          "priority": "high"
        }
      }
      // ... more audit entries
    ],
    "pagination": {
      "total": 45678,
      "limit": 100,
      "offset": 0,
      "has_more": true
    }
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_audit_001"
  }
}
```

---

## 11. Webhooks

### 11.1 Configure Webhook

**Endpoint:** `POST /webhooks`

**Request:**
```json
{
  "url": "https://your-app.example.com/webhooks/wargames",
  "events": [
    "assessment.completed",
    "assessment.failed",
    "risk.critical_change",
    "data.collection_failed"
  ],
  "secret": "whsec_1234567890abcdefghijklmnopqrstuvwxyz",
  "active": true
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": "hook_1234567890",
    "url": "https://your-app.example.com/webhooks/wargames",
    "events": [
      "assessment.completed",
      "assessment.failed",
      "risk.critical_change",
      "data.collection_failed"
    ],
    "secret": "whsec_***", 
    "active": true,
    "created_at": "2025-10-27T12:40:00Z"
  },
  "metadata": {
    "timestamp": "2025-10-27T12:40:00Z",
    "request_id": "req_webhook_001"
  }
}
```

### 11.2 Webhook Event Format

**Headers:**
```http
Content-Type: application/json
X-Webhook-Signature: sha256=abc123...
X-Webhook-Event: assessment.completed
X-Webhook-ID: evt_1234567890
X-Webhook-Timestamp: 2025-10-27T12:39:32Z
```

**Payload:**
```json
{
  "event": "assessment.completed",
  "timestamp": "2025-10-27T12:39:32Z",
  "data": {
    "assessment_id": "assess_1234567890",
    "seconds_to_midnight": 90,
    "risk_level": "critical",
    "confidence": 0.87,
    "change_from_previous": -30
  }
}
```

### 11.3 Webhook Events

| Event | Description | Triggers When |
|-------|-------------|---------------|
| `assessment.started` | Assessment begins | New assessment triggered |
| `assessment.completed` | Assessment finishes | Assessment completes successfully |
| `assessment.failed` | Assessment fails | Assessment encounters error |
| `risk.critical_change` | Major risk change | Risk score crosses critical threshold |
| `risk.level_change` | Risk level changes | Risk moves between levels |
| `data.collection_started` | Data collection begins | Collection triggered |
| `data.collection_completed` | Collection finishes | Data collected successfully |
| `data.collection_failed` | Collection fails | Collection encounters error |
| `data.source_failed` | Source unavailable | Individual source fails |
| `system.health_degraded` | System issues | Component health degrades |

### 11.4 Webhook Signature Verification

**Node.js Example:**
```javascript
const crypto = require('crypto');

function verifyWebhookSignature(payload, signature, secret) {
  const expectedSignature = crypto
    .createHmac('sha256', secret)
    .update(JSON.stringify(payload))
    .digest('hex');
  
  return signature === `sha256=${expectedSignature}`;
}

// Usage
const isValid = verifyWebhookSignature(
  req.body,
  req.headers['x-webhook-signature'],
  'whsec_1234567890abcdefghijklmnopqrstuvwxyz'
);
```

**Python Example:**
```python
import hmac
import hashlib
import json

def verify_webhook_signature(payload: dict, signature: str, secret: str) -> bool:
    expected_signature = hmac.new(
        secret.encode('utf-8'),
        json.dumps(payload).encode('utf-8'),
        hashlib.sha256
    ).hexdigest()
    
    return signature == f"sha256={expected_signature}"
```

---

## 12. SDK Examples

### 12.1 Python SDK

```python
from wargames import WarGamesClient

# Initialize client
client = WarGamesClient(
    api_key="wg_live_1234567890abcdefghijklmnopqrstuvwxyz",
    base_url="https://api.wargames-joshua.example.com/v1"
)

# Trigger new assessment
assessment = client.assessments.create(
    trigger_source="manual",
    priority="high",
    include_monte_carlo=True
)

print(f"Assessment ID: {assessment.id}")
print(f"Status: {assessment.status}")

# Wait for completion
assessment = client.assessments.wait_for_completion(
    assessment.id,
    timeout=600
)

# Get results
print(f"Seconds to Midnight: {assessment.result.seconds_to_midnight}")
print(f"Risk Level: {assessment.result.risk_level}")
print(f"Confidence: {assessment.result.confidence}")

# Get current risk score
risk = client.risk.get_current()
print(f"Current Risk: {risk.seconds_to_midnight} seconds ({risk.risk_level})")

# Get historical trend
trend = client.risk.get_trend(period="30d", resolution="daily")
for point in trend.data_points:
    print(f"{point.timestamp}: {point.seconds_to_midnight} seconds")

# Generate report
report = client.reports.create(
    assessment_id=assessment.id,
    report_type="executive_summary",
    format="pdf"
)

# Wait for report generation
report = client.reports.wait_for_completion(report.id)

# Download report
client.reports.download(report.id, "risk-assessment-report.pdf")
```

### 12.2 JavaScript/TypeScript SDK

```typescript
import { WarGamesClient } from '@wargames/sdk';

// Initialize client
const client = new WarGamesClient({
  apiKey: 'wg_live_1234567890abcdefghijklmnopqrstuvwxyz',
  baseUrl: 'https://api.wargames-joshua.example.com/v1'
});

// Trigger new assessment
const assessment = await client.assessments.create({
  triggerSource: 'manual',
  priority: 'high',
  includeMonteCarlo: true
});

console.log(`Assessment ID: ${assessment.id}`);
console.log(`Status: ${assessment.status}`);

// Wait for completion
const completed = await client.assessments.waitForCompletion(
  assessment.id,
  { timeout: 600000 }
);

// Get results
console.log(`Seconds to Midnight: ${completed.result.secondsToMidnight}`);
console.log(`Risk Level: ${completed.result.riskLevel}`);
console.log(`Confidence: ${completed.result.confidence}`);

// Get current risk score
const risk = await client.risk.getCurrent();
console.log(`Current Risk: ${risk.secondsToMidnight} seconds (${risk.riskLevel})`);

// Get historical trend
const trend = await client.risk.getTrend({
  period: '30d',
  resolution: 'daily'
});

for (const point of trend.dataPoints) {
  console.log(`${point.timestamp}: ${point.secondsToMidnight} seconds`);
}

// Subscribe to webhooks (using Express)
app.post('/webhooks/wargames', (req, res) => {
  const signature = req.headers['x-webhook-signature'];
  const isValid = client.webhooks.verifySignature(
    req.body,
    signature,
    'whsec_1234567890abcdefghijklmnopqrstuvwxyz'
  );
  
  if (!isValid) {
    return res.status(401).send('Invalid signature');
  }
  
  const event = req.body;
  console.log(`Received event: ${event.event}`);
  
  if (event.event === 'assessment.completed') {
    console.log(`Assessment ${event.data.assessment_id} completed`);
    console.log(`Risk Score: ${event.data.seconds_to_midnight}`);
  }
  
  res.status(200).send('OK');
});
```

### 12.3 Rust SDK

```rust
use wargames_client::{WarGamesClient, AssessmentRequest};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = WarGamesClient::new(
        "wg_live_1234567890abcdefghijklmnopqrstuvwxyz",
        "https://api.wargames-joshua.example.com/v1"
    );
    
    // Trigger new assessment
    let request = AssessmentRequest {
        trigger_source: "manual".to_string(),
        priority: "high".to_string(),
        include_monte_carlo: true,
        ..Default::default()
    };
    
    let assessment = client.assessments.create(&request).await?;
    
    println!("Assessment ID: {}", assessment.id);
    println!("Status: {}", assessment.status);
    
    // Wait for completion
    let completed = client.assessments
        .wait_for_completion(&assessment.id, 600)
        .await?;
    
    // Get results
    if let Some(result) = completed.result {
        println!("Seconds to Midnight: {}", result.seconds_to_midnight);
        println!("Risk Level: {}", result.risk_level);
        println!("Confidence: {}", result.confidence);
    }
    
    // Get current risk score
    let risk = client.risk.get_current().await?;
    println!("Current Risk: {} seconds ({})", 
        risk.seconds_to_midnight, risk.risk_level);
    
    // Get historical trend
    let trend = client.risk
        .get_trend("30d", "daily")
        .await?;
    
    for point in trend.data_points {
        println!("{}: {} seconds", 
            point.timestamp, point.seconds_to_midnight);
    }
    
    Ok(())
}
```

### 12.4 cURL Examples

**Trigger Assessment:**
```bash
curl -X POST https://api.wargames-joshua.example.com/v1/assessments \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "trigger_source": "manual",
    "priority": "high",
    "include_monte_carlo": true
  }'
```

**Get Current Risk:**
```bash
curl https://api.wargames-joshua.example.com/v1/risk/current \
  -H "Authorization: Bearer $JWT_TOKEN"
```

**Get Assessment Status:**
```bash
curl https://api.wargames-joshua.example.com/v1/assessments/assess_1234567890 \
  -H "Authorization: Bearer $JWT_TOKEN"
```

**Download Report:**
```bash
curl https://api.wargames-joshua.example.com/v1/reports/rpt_1234567890/download \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -o risk-assessment-report.pdf
```

---

## Best Practices

### 12.5 Error Handling

Always implement proper error handling:

```python
from wargames import WarGamesClient, WarGamesError

client = WarGamesClient(api_key="...")

try:
    assessment = client.assessments.create(...)
except WarGamesError as e:
    if e.code == "RATE_LIMIT_EXCEEDED":
        # Wait and retry
        time.sleep(e.retry_after)
        assessment = client.assessments.create(...)
    elif e.code == "UNAUTHORIZED":
        # Refresh token
        client.auth.refresh_token()
    else:
        # Log and handle
        logger.error(f"Assessment failed: {e.message}")
```

### 12.6 Rate Limiting

Implement exponential backoff:

```python
import time
from functools import wraps

def retry_with_backoff(max_retries=3, base_delay=1):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            for attempt in range(max_retries):
                try:
                    return func(*args, **kwargs)
                except WarGamesError as e:
                    if e.code == "RATE_LIMIT_EXCEEDED" and attempt < max_retries - 1:
                        delay = base_delay * (2 ** attempt)
                        time.sleep(delay)
                    else:
                        raise
        return wrapper
    return decorator

@retry_with_backoff(max_retries=3)
def trigger_assessment():
    return client.assessments.create(...)
```

### 12.7 Webhook Handling

Implement idempotency:

```python
processed_events = set()

@app.route('/webhooks/wargames', methods=['POST'])
def handle_webhook():
    event_id = request.headers.get('X-Webhook-ID')
    
    # Check if already processed
    if event_id in processed_events:
        return jsonify({'status': 'already_processed'}), 200
    
    # Verify signature
    if not verify_webhook_signature(
        request.json,
        request.headers.get('X-Webhook-Signature'),
        WEBHOOK_SECRET
    ):
        return jsonify({'error': 'Invalid signature'}), 401
    
    # Process event
    event = request.json
    process_event(event)
    
    # Mark as processed
    processed_events.add(event_id)
    
    return jsonify({'status': 'success'}), 200
```

---

## Support

### Documentation
- API Documentation: https://docs.wargames-joshua.example.com
- SDK Documentation: https://docs.wargames-joshua.example.com/sdks
- Code Examples: https://github.com/wargames-joshua/examples

### Support Channels
- Email: api-support@wargames-joshua.example.com
- Slack: #wargames-api
- Status Page: https://status.wargames-joshua.example.com

### Rate Limit Increases
For rate limit increases, contact enterprise@wargames-joshua.example.com

---

**Document Version:** 1.0.0  
**Last Updated:** October 2025  
**API Version:** v1  
**Maintained By:** WarGames/JOSHUA Development Team  
**Next Review:** November 2025

*"The only way to win is to know the game."*
