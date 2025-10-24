
pub mod curvature_signal;
pub mod entangle;
pub mod gkernel;
pub mod resonance;
pub mod sem_eng;
pub mod wavelet;
pub mod hotspot_detector;
pub mod path_evaluator;
pub mod core;
pub mod coherence;

pub use core::PathEvaluator;
pub use coherence::CoherencePulse;
pub use curvature_signal::CurvatureSignal;
pub use entangle::{Coupling, SemanticDomain, SimpleEntangleMap};
pub use gkernel::{ResonanceNode, ResonanceEdge, GraphKernel};
pub use hotspot_detector::{HotspotDetector, PercentileHotspot};
pub use path_evaluator::{PathMetrics, TrajectoryPath};
pub use resonance::{
    Resonance, 
    Position, 
    Gradient, 
    GridField, 
    BiologicalField,
    EntangleMap,
    LawSynthEngine,
    ResonanceField,
};
pub use sem_eng::{
    SemanticEngine, 
    VisualEdge, 
    VisualNode, 
    EntanglementOverlay, 
    Synth, 
    Field};
pub use wavelet::{
    FusionContext, 
    WaveletBasis, 
    WaveletDecomposition, 
    WaveletEngine, 
    WaveletFusionStrategy, 
    compute_entropy,
};