# Curvature

[![Crates.io](https://img.shields.io/crates/v/curvature.svg?style=flat-square)](https://crates.io/crates/curvature)
[![Docs.rs](https://img.shields.io/docsrs/curvature?style=flat-square)](https://docs.rs/curvature)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/neil-crago/curvature/actions/workflows/rust.yml/badge.svg)](https://github.com/neil-crago/curvature/actions/workflows/rust.yml)

Curvature is a Rust crate for exploratory signal and geometry analysis, with an emphasis on sparse or noisy data. The library includes wavelet-based smoothing and fusion, curvature signal reconstruction, hotspot detection, resonance-field primitives, and lightweight graph-like semantic coupling utilities.

## Status

The crate is actively being revived and hardened. The current focus is on correctness, safe edge handling, and a cleaner public API so the project is usable as a real research and experimentation toolkit rather than a collection of unfinished prototypes.

## Highlights

- Curvature signal reconstruction from sparse measurements
- Wavelet smoothing and coefficient fusion strategies
- Hotspot detection for high-curvature regions
- Path metrics and trajectory evaluation helpers
- Resonance field abstractions and semantic coupling primitives

## Quick start

```rust
use curvature::{CurvatureSignal, PercentileHotspot, TrajectoryPath, WaveletTransformStruct};

let positions = vec![0.0, 0.2, 0.5, 0.7, 1.0];
let values = vec![1.0, 1.5, 0.8, 2.0, 1.2];
let signal = CurvatureSignal { positions, values };

let reconstructed = signal.reconstruct();
let detector = PercentileHotspot { percentile: 80.0 };
let hotspots = detector.detect(&reconstructed);

let evaluator = TrajectoryPath { dz_dt: 0.1 };
let metrics = evaluator.evaluate(&reconstructed, 0.01);

let smoother = WaveletTransformStruct { levels: 2, threshold: 0.1 };
let smoothed = smoother.smooth(&reconstructed);

println!("hotspots: {:?}", hotspots);
println!("path length: {:.2}", metrics.length);
println!("smoothed: {:?}", smoothed);
```

## Examples

See the example under [examples/curves/src/main.rs](examples/curves/src/main.rs) for a runnable signal-processing demonstration.

## License

This project is licensed under the MIT license. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

## Author

Neil Crago
