/// Core traits and structures for semantic signal processing and wavelet fusion.
use crate::path_evaluator::PathMetrics;

/// Trait for evaluating paths based on signal characteristics.
pub trait PathEvaluator {
    /// Evaluate a signal and return the associated path metrics.
    fn evaluate(&self, signal: &[f64]) -> PathMetrics;
}
