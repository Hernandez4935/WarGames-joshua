-- Risk factors table
-- Migration: 20250101000002_risk_factors

CREATE TABLE IF NOT EXISTS risk_factors (
    id UUID PRIMARY KEY,
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    factor_category VARCHAR(50) NOT NULL,
    factor_name VARCHAR(200) NOT NULL,
    raw_value DECIMAL(5,4) NOT NULL CHECK (raw_value >= 0.0 AND raw_value <= 1.0),
    weighted_value DECIMAL(5,4) NOT NULL,
    contribution_to_risk DECIMAL(5,4) NOT NULL,
    confidence_level VARCHAR(20) NOT NULL,
    data_sources TEXT[],
    observed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_risk_factors_assessment ON risk_factors(assessment_id);
CREATE INDEX IF NOT EXISTS idx_risk_factors_category ON risk_factors(factor_category);
CREATE INDEX IF NOT EXISTS idx_risk_factors_contribution ON risk_factors(contribution_to_risk DESC);
CREATE INDEX IF NOT EXISTS idx_risk_factors_assessment_category ON risk_factors(assessment_id, factor_category);
