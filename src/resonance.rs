/// Resonance module: defines resonance fields, gradients, and related traits.
/// This module provides abstractions for resonance fields,
/// entanglement maps, and law synthesis engines.
use crate::wavelet::{
    FusionContext, WaveletBasis, WaveletDecomposition, WaveletEngine, WaveletFusionStrategy,
    compute_entropy,
};
use coheron::traits::BeliefTensor;

/// A local resonance estimate with amplitude and spectral frequency.
#[derive(Debug, Clone)]
pub struct Resonance {
    /// Energy or magnitude of the resonance.
    pub amplitude: f64,
    /// Dominant oscillatory frequency associated with the field.
    pub frequency: f64,
}

/// Gradient information derived from a resonance field.
#[derive(Debug, Clone)]
pub struct Gradient {
    /// Directional derivative in x/y space.
    pub direction: [f64; 2],
    /// Magnitude of the local gradient.
    pub magnitude: f64,
}

/// A 2D position in the resonance field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// Horizontal coordinate.
    pub x: f64,
    /// Vertical coordinate.
    pub y: f64,
}

/// A dense 2D field storing coherence values over a grid.
pub struct GridField {
    /// Matrix of coherence values indexed by row/column.
    pub coherence_map: Vec<Vec<f64>>, // 2D grid
    /// Grid width in cells.
    pub width: usize,
    /// Grid height in cells.
    pub height: usize,
}

/// Trait for systems that produce a local resonance response from a position.
pub trait ResonanceField {
    /// Position type used by the field.
    type Position;
    /// Gradient-like measurement emitted by the field.
    type Gradient;
    /// Resonance descriptor produced by the field.
    type Resonance;

    /// Observe the local gradient or signal at a position.
    fn observe(&self, position: &Self::Position) -> Self::Gradient;
    /// Compute the local resonance amplitude and frequency.
    fn compute_resonance(&self, position: &Self::Position) -> Self::Resonance;
    /// Propagate field influence into the region around a position.
    fn propagate(&mut self, position: &Self::Position, influence: &Self::Resonance);

    /// Returns the raw signal representing the resonance field.
    fn signal(&self) -> &[f64];

    /// Returns the semantic domain label (e.g. "quantum", "biological").
    fn domain_label(&self) -> &str;

    /// Returns the fusion context for spectral analysis.
    fn fusion_context(&self) -> FusionContext;

    /// Performs wavelet fusion and returns the fused decomposition.
    fn fused_spectrum<F: WaveletFusionStrategy>(
        &self,
        engine: &WaveletEngine<F>,
        level: usize,
    ) -> WaveletDecomposition {
        engine.fuse(self.signal(), &self.fusion_context(), level)
    }

    /// Optionally returns the dominant basis for this field.
    fn dominant_basis<F: WaveletFusionStrategy>(
        &self,
        engine: &WaveletEngine<F>,
    ) -> Option<WaveletBasis> {
        let mut best = None;
        for (basis, score) in engine.score_bases(self.signal(), &self.fusion_context()) {
            if !score.is_finite() {
                continue;
            }
            match best {
                None => best = Some((basis, score)),
                Some((_, best_score)) if score > best_score => best = Some((basis, score)),
                _ => {}
            }
        }
        best.map(|(basis, _)| basis)
    }
}

/// Trait for entangling different semantic domains.
/// A mapping from domain pairs to coupling values and updates.
pub trait EntangleMap {
    /// Semantic domain label used by the map.
    type Domain;
    /// Coupling representation for the relationship.
    type Coupling;

    /// Construct a new empty coupling map.
    fn new() -> Self;
    /// Fetch the coupling value between two domains.
    fn get_coupling(&self, domain_a: &Self::Domain, domain_b: &Self::Domain) -> Self::Coupling;
    /// Update the coupling between two domains by a delta.
    fn update_coupling(
        &mut self,
        domain_a: &Self::Domain,
        domain_b: &Self::Domain,
        delta: Self::Coupling,
    );
}

/// Synthesizes a control law from belief, resonance, and entanglement state.
pub trait LawSynthEngine<B, R, E>
where
    B: BeliefTensor,
    R: ResonanceField,
    E: EntangleMap,
{
    /// Output type for synthesized control.
    type ControlLaw;

    /// Produce a control law based on the current system state.
    fn synthesize(
        &self,
        belief: &B::Posterior,
        resonance: &R::Resonance,
        entanglement: &E,
    ) -> Self::ControlLaw;
}

/// A pulse that can trigger coherence updates when a threshold is reached.
pub trait CoherencePulse<B, E>
where
    B: BeliefTensor,
    E: EntangleMap,
{
    /// Trigger a coherence update for the current belief and entanglement map.
    fn trigger(&mut self, belief: &mut B, entanglement: &mut E);
}

impl ResonanceField for GridField {
    type Position = Position;
    type Gradient = Gradient;
    type Resonance = Resonance;

    fn observe(&self, pos: &Position) -> Gradient {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let x = x.min(self.width.saturating_sub(1));
        let y = y.min(self.height.saturating_sub(1));

        let center = self
            .coherence_map
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(0.0);
        let left = self
            .coherence_map
            .get(y)
            .and_then(|row| row.get(x.saturating_sub(1)))
            .copied()
            .unwrap_or(center);
        let up = self
            .coherence_map
            .get(y.saturating_sub(1))
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(center);
        let dx = left - center;
        let dy = up - center;

        Gradient {
            direction: [dx, dy],
            magnitude: (dx.powi(2) + dy.powi(2)).sqrt(),
        }
    }

    fn compute_resonance(&self, pos: &Position) -> Resonance {
        let grad = self.observe(pos);
        Resonance {
            amplitude: grad.magnitude,
            frequency: grad.direction[0].abs() + grad.direction[1].abs(),
        }
    }

    fn propagate(&mut self, pos: &Position, influence: &Resonance) {
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let x = x.min(self.width.saturating_sub(1));
        let y = y.min(self.height.saturating_sub(1));
        let delta = influence.amplitude * 0.01;

        if let Some(row) = self.coherence_map.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                *cell += delta;
            }
        }
    }

    fn signal(&self) -> &[f64] {
        // Flatten the 2D coherence_map into a 1D slice for signal
        // This is a simple implementation; you may want to adjust as needed
        // For now, return the first row as a slice
        self.coherence_map
            .first()
            .map(|row| row.as_slice())
            .unwrap_or(&[])
    }

    fn domain_label(&self) -> &str {
        "GridField"
    }

    fn fusion_context(&self) -> crate::wavelet::FusionContext {
        crate::wavelet::FusionContext::default()
    }
}

fn _init_field(width: usize, height: usize) -> GridField {
    let coherence_map = vec![vec![0.5; width]; height];
    GridField {
        coherence_map,
        width,
        height,
    }
}

/// A biologically flavored resonance field over a scalar signal.
pub struct BiologicalField {
    /// Input signal values.
    pub signal: Vec<f64>,
    /// Semantic tags associated with the signal.
    pub tags: Vec<String>,
    /// Per-sample resonance amplitude.
    pub resonance: Vec<f64>,
    /// Per-sample curvature estimate.
    pub curvature: Vec<f64>,
}

impl ResonanceField for BiologicalField {
    type Position = usize;
    type Gradient = f64;
    type Resonance = f64;

    fn signal(&self) -> &[f64] {
        &self.signal
    }

    fn domain_label(&self) -> &str {
        "biological"
    }

    fn fusion_context(&self) -> FusionContext {
        FusionContext {
            domain_entropy: compute_entropy(&self.signal),
            resonance_profile: Some(self.resonance.clone()),
            semantic_tags: self.tags.clone(),
            coherence_map: None,
            curvature_profile: Some(self.curvature.clone()),
            domain_label: Some("biological".into()),
        }
    }

    fn observe(&self, position: &Self::Position) -> Self::Gradient {
        self.signal.get(*position).copied().unwrap_or(0.0)
    }

    fn compute_resonance(&self, position: &Self::Position) -> Self::Resonance {
        self.resonance.get(*position).copied().unwrap_or(0.0)
    }

    fn propagate(&mut self, position: &Self::Position, influence: &Self::Resonance) {
        if let Some(r) = self.resonance.get_mut(*position) {
            *r += *influence;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wavelet::{ResonanceWeightedFusion, WaveletEngine};

    #[test]
    fn dominant_basis_returns_none_when_scores_are_empty_or_invalid() {
        let field = BiologicalField {
            signal: vec![1.0, 2.0, 3.0],
            tags: vec!["test".into()],
            resonance: vec![0.5, 0.5, 0.5],
            curvature: vec![1.0, 1.0, 1.0],
        };

        let engine = WaveletEngine::new(Vec::new(), ResonanceWeightedFusion);
        assert!(field.dominant_basis(&engine).is_none());
    }

    #[test]
    fn grid_field_observe_handles_boundaries_safely() {
        let field = GridField {
            coherence_map: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            width: 2,
            height: 2,
        };

        let gradient = field.observe(&Position { x: 0.0, y: 0.0 });
        assert_eq!(gradient.direction, [0.0, 0.0]);
        assert_eq!(gradient.magnitude, 0.0);
    }
}
