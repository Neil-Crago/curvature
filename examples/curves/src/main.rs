use curvature::curvature_signal::CurvatureSignal;
use curvature::hotspot_detector::PercentileHotspot;
use curvature::path_evaluator::{TrajectoryPath};
use curvature::wavelet::WaveletTransformStruct;

/// Demonstrate Wavelet Transform smoothing
fn wvt() {
    let raw_signal = vec![1.0, 1.5, 0.8, 2.0, 1.2, 0.9, 1.8, 2.2];
    let wavelet = WaveletTransformStruct { levels: 2, threshold: 0.1 };
    let smoothed = wavelet.smooth(&raw_signal);
    println!("Smoothed signal: {:?}", smoothed);
}


fn main() {
    println!();

    // Simulate sparse curvature
    let positions = vec![0.0, 0.2, 0.5, 0.7, 1.0];
    let values = vec![1.0, 1.5, 0.8, 2.0, 1.2];
    let signal = CurvatureSignal { positions, values };

    let recon = signal.reconstruct();
    println!("Reconstructed signal: {:?}", recon);

    let detector = PercentileHotspot { percentile: 80.0 };
    let hotspots = detector.detect(&recon);
    println!("Hotspot indices: {:?}", hotspots);

    let evaluator = TrajectoryPath { dz_dt: 0.1 };
    let metrics = evaluator.evaluate(&recon, 0.01);
    println!(
        "Path length: {:.2}, Manhattan distance: {:.2}",
        metrics.length, metrics.manhattan_distance
    );

    println!();
    
    // Demonstrate Wavelet Transform smoothing
    wvt();

    println!();
}