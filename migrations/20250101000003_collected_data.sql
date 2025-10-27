-- Collected data points table
-- Migration: 20250101000003_collected_data

CREATE TABLE IF NOT EXISTS collected_data (
    id UUID PRIMARY KEY,
    source VARCHAR(200) NOT NULL,
    source_url TEXT,
    title VARCHAR(500),
    content TEXT NOT NULL,
    data_category VARCHAR(50) NOT NULL,
    reliability_score DECIMAL(3,2) CHECK (reliability_score >= 0.0 AND reliability_score <= 1.0),
    published_at TIMESTAMP WITH TIME ZONE,
    collected_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    processed BOOLEAN DEFAULT FALSE,
    used_in_assessment UUID REFERENCES assessments(id)
);

CREATE INDEX IF NOT EXISTS idx_collected_data_category ON collected_data(data_category);
CREATE INDEX IF NOT EXISTS idx_collected_data_collected ON collected_data(collected_at DESC);
CREATE INDEX IF NOT EXISTS idx_collected_data_published ON collected_data(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_collected_data_processed ON collected_data(processed);

-- Full-text search index (PostgreSQL-specific)
CREATE INDEX IF NOT EXISTS idx_collected_data_content_fts
    ON collected_data USING GIN (to_tsvector('english', content));
