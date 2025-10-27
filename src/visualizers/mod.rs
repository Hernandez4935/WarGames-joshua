//! Visualization generators for risk assessments.

use crate::prelude::*;
use std::path::PathBuf;

/// Trait for visualization generation
pub trait Visualizer: Send + Sync {
    /// Generate visualization from assessment data
    fn visualize(&self, assessment: &Assessment) -> Result<Visualization>;

    /// Name of this visualization
    fn name(&self) -> &str;

    /// Output format(s) supported
    fn supported_formats(&self) -> &[VisualizationFormat];

    /// Default output format
    fn default_format(&self) -> VisualizationFormat {
        VisualizationFormat::Svg
    }
}

/// Generated visualization
#[derive(Debug, Clone)]
pub struct Visualization {
    /// Visualization name
    pub name: String,

    /// Output format
    pub format: VisualizationFormat,

    /// File path where visualization is saved
    pub file_path: PathBuf,

    /// Metadata about the visualization
    pub metadata: VisualizationMetadata,
}

/// Metadata about a visualization
#[derive(Debug, Clone)]
pub struct VisualizationMetadata {
    /// Width in pixels
    pub width: u32,

    /// Height in pixels
    pub height: u32,

    /// When generated
    pub generated_at: DateTime<Utc>,
}
