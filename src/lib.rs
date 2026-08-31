#![warn(missing_docs)]

//! Curvature is a research-oriented Rust library for sparse-signal analysis,
//! wavelet-based smoothing, resonance fields, and curvature estimation.
//!
//! The crate is designed for experimentation with noisy or irregular data where
//! a robust signal-first workflow is more useful than a single hardcoded model.
//! It includes primitives for reconstructing curvature signals, fusing wavelet
//! coefficients, evaluating trajectories, and modeling resonance-like structures.
//!
//! # Example
//!
//! ```rust
//! use curvature::{CurvatureSignal, PercentileHotspot, TrajectoryPath};
//!
//! let signal = CurvatureSignal {
//!     positions: vec![0.0, 0.2, 0.5, 0.7, 1.0],
//!     values: vec![1.0, 1.5, 0.8, 2.0, 1.2],
//! };
//!
//! let reconstructed = signal.reconstruct();
//! let detector = PercentileHotspot { percentile: 80.0 };
//! let hotspots = detector.detect(&reconstructed);
//! let path = TrajectoryPath { dz_dt: 0.1 };
//! let metrics = path.evaluate(&reconstructed, 0.01);
//!
//! assert!(!hotspots.is_empty() || metrics.length >= 0.0);
//! ```

/// Coherence pulse primitives.
pub mod coherence;
/// Core evaluation traits.
pub mod core;
/// Curvature signal reconstruction utilities.
pub mod curvature_signal;
/// Semantic entanglement primitives and coupling maps.
pub mod entangle;
/// Graph kernel structures for resonance-like connectivity.
pub mod gkernel;
/// Hotspot detection and thresholding utilities.
pub mod hotspot_detector;
/// Trajectory and path evaluation metrics.
pub mod path_evaluator;
/// Curated, user-facing types for day-to-day usage.
pub mod prelude;
/// Resonance-field and field-propagation abstractions.
pub mod resonance;
/// Semantic-engine overlays and synthetic field representations.
pub mod sem_eng;
/// Wavelet transforms, fusion strategies, and entropy helpers.
pub mod wavelet;

pub use prelude::*;
