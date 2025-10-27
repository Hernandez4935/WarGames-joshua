-- Initial database schema for WarGames/JOSHUA
-- Migration: 20250101000001_initial_schema

-- Core assessment tracking
CREATE TABLE IF NOT EXISTS assessments (
    id UUID PRIMARY KEY,
    assessment_date TIMESTAMP WITH TIME ZONE NOT NULL,
    seconds_to_midnight INTEGER NOT NULL CHECK (seconds_to_midnight >= 0 AND seconds_to_midnight <= 1440),
    raw_risk_score DECIMAL(5,4) NOT NULL CHECK (raw_risk_score >= 0.0 AND raw_risk_score <= 1.0),
    bayesian_adjusted_score DECIMAL(5,4) NOT NULL,
    overall_confidence VARCHAR(20) NOT NULL CHECK (overall_confidence IN ('VeryLow', 'Low', 'Moderate', 'High', 'VeryHigh')),
    trend_direction VARCHAR(20) CHECK (trend_direction IN ('Increasing', 'Decreasing', 'Stable', 'Uncertain')),
    trend_magnitude DECIMAL(5,4),
    delta_from_previous INTEGER,
    executive_summary TEXT NOT NULL,
    detailed_analysis TEXT,
    claude_model_version VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT assessments_date_idx UNIQUE (assessment_date)
);

-- Indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_assessments_seconds ON assessments(seconds_to_midnight);
CREATE INDEX IF NOT EXISTS idx_assessments_date_desc ON assessments(assessment_date DESC);
CREATE INDEX IF NOT EXISTS idx_assessments_trend ON assessments(trend_direction);
CREATE INDEX IF NOT EXISTS idx_assessments_created ON assessments(created_at);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger for assessments updated_at
CREATE TRIGGER update_assessments_updated_at
    BEFORE UPDATE ON assessments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
