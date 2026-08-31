use curvature::prelude::*;

fn smooth_signal(signal: &[f64]) -> Vec<f64> {
    let wavelet = WaveletTransformStruct {
        levels: 2,
        threshold: 0.1,
    };
    wavelet.smooth(signal)
}

fn main() {
    println!("Curvature example");
    println!("-----------------");

    let positions = vec![0.0, 0.2, 0.5, 0.7, 1.0];
    let values = vec![1.0, 1.5, 0.8, 2.0, 1.2];
    let signal = CurvatureSignal { positions, values };

    let reconstructed = signal.reconstruct();
    println!("Reconstructed signal: {:?}", reconstructed);

    let detector = PercentileHotspot { percentile: 80.0 };
    let hotspots = detector.detect(&reconstructed);
    println!("Hotspot indices: {:?}", hotspots);

    let evaluator = TrajectoryPath { dz_dt: 0.1 };
    let metrics = evaluator.evaluate(&reconstructed, 0.01);
    println!(
        "Path length: {:.2}, Manhattan distance: {:.2}",
        metrics.length, metrics.manhattan_distance
    );

    let smoothed = smooth_signal(&reconstructed);
    println!("Smoothed signal: {:?}", smoothed);
}