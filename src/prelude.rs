//! A curated set of the most useful public types for day-to-day crate usage.

pub use crate::coherence::CoherencePulse;
pub use crate::curvature_signal::CurvatureSignal;
pub use crate::entangle::{Coupling, SemanticDomain, SimpleEntangleMap};
pub use crate::gkernel::{GraphKernel, ResonanceEdge, ResonanceNode};
pub use crate::hotspot_detector::{HotspotDetector, PercentileHotspot};
pub use crate::path_evaluator::{PathMetrics, TrajectoryPath};
pub use crate::resonance::{
    BiologicalField, EntangleMap, Gradient, GridField, LawSynthEngine, Position, Resonance,
    ResonanceField,
};
pub use crate::sem_eng::{
    EntanglementOverlay, Field, SemanticEngine, Synth, VisualEdge, VisualNode,
};
pub use crate::wavelet::{
    FusionContext, WaveletBasis, WaveletDecomposition, WaveletEngine, WaveletFusionStrategy,
    WaveletTransformStruct, compute_entropy,
};
