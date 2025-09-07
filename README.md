# Curvature 

- Holds raw curvature data and reconstruction logic
- Will support sparse sampling, Lomb-Scargle-like frequency estimation (or placeholder)
- Will expose a method like fn reconstruct(&self) -> Vec<f64>

### hotspot_detector.rs
- Detects high-curvature zones (semantic attractors)
- Uses percentile-based thresholding
- Exposes fn detect(&self, signal: &[f64]) -> Vec<usize>

### path_evaluator.rs
- Computes semantic path length vs. Manhattan distance
- Encodes curvature into trajectory
- Exposes fn evaluate(&self, signal: &[f64]) -> PathMetrics

This involves using the QFT-style constraints as informative priors within the Bayesian `BeliefTensor` itself.

