/// Hotspot detection module for curvature signals.
/// Defines traits and implementations for identifying hotspots
/// in curvature data.
/// Detects signal indices whose values are above a percentile threshold.
pub trait HotspotDetector {
    /// Return the indices of the most significant hotspots in a signal.
    fn detect(&self, signal: &[f64]) -> Vec<usize>;
}

/// Simple hotspot detector based on a percentile threshold.
#[derive(Debug, Clone)]
pub struct PercentileHotspot {
    /// Percentile cut-off, such as 80.0 for the top 20% of samples.
    pub percentile: f64,
}

impl PercentileHotspot {
    /// Detect indices whose signal values exceed the percentile threshold.
    pub fn detect(&self, signal: &[f64]) -> Vec<usize> {
        if signal.is_empty() {
            return vec![];
        }

        let mut sorted = signal.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let index = ((self.percentile / 100.0) * sorted.len() as f64).floor() as usize;
        let threshold = sorted[index.min(sorted.len() - 1)];

        signal
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v >= threshold { Some(i) } else { None })
            .collect()
    }
}
