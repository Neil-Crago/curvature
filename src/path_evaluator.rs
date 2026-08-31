/// Path evaluator module: evaluates paths based on curvature signals.
/// Defines structures and methods for computing path metrics.
/// Aggregate path metrics computed from a curvature signal.
#[derive(Debug)]
pub struct PathMetrics {
    /// Total path length traversed.
    pub length: f64,
    /// Manhattan-style displacement of the trajectory.
    pub manhattan_distance: f64,
    /// X coordinates for the simulated path.
    pub x: Vec<f64>,
    /// Y coordinates for the simulated path.
    pub y: Vec<f64>,
}

/// Simple trajectory evaluator that projects curvature into a 2D path.
pub struct TrajectoryPath {
    /// Optional z-bias, exposed for future trajectory extensions.
    pub dz_dt: f64,
}

impl TrajectoryPath {
    /// Evaluate a curvature-driven trajectory over a time step.
    pub fn evaluate(&self, curvature: &[f64], dt: f64) -> PathMetrics {
        let mut theta = Vec::with_capacity(curvature.len());
        let mut x = Vec::with_capacity(curvature.len());
        let mut y = Vec::with_capacity(curvature.len());

        let mut angle = 0.0;
        let mut px = 0.0;
        let mut py = 0.0;

        for &k in curvature {
            angle += k * dt;
            px += angle.cos() * dt;
            py += angle.sin() * dt;

            theta.push(angle);
            x.push(px);
            y.push(py);
        }

        let length = curvature.len() as f64 * dt;
        let dx = x.last().unwrap_or(&0.0) - x.first().unwrap_or(&0.0);
        let dy = y.last().unwrap_or(&0.0) - y.first().unwrap_or(&0.0);
        let manhattan = dx.abs() + dy.abs();

        PathMetrics {
            length,
            manhattan_distance: manhattan,
            x,
            y,
        }
    }
}
