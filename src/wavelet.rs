use ndarray::ArrayViewMut1;
/// Wavelet transform and fusion module.
/// Provides traits and implementations for wavelet decomposition,
/// reconstruction, and fusion strategies.
use omni_wave::*;

/// Compute a Haar-transform approximation of the input signal.
pub fn haar_transform(signal: &[f64]) -> Vec<f64> {
    let wavelet = wavelet::HAAR;
    let signal_len = signal.len();
    let buffer_len = signal_len + wavelet.window_size() - 2;

    // Convert signal to f32
    let mut signal_f32: Vec<f32> = signal.iter().map(|&x| x as f32).collect();
    let mut buffer_f32 = vec![0f32; buffer_len];

    // Convert Vec<f32> to ArrayViewMut1<f32> as required by omni_wave
    let signal_view = ArrayViewMut1::from_shape(signal_f32.len(), &mut signal_f32[..]).unwrap();
    let buffer_view = ArrayViewMut1::from_shape(buffer_f32.len(), &mut buffer_f32[..]).unwrap();

    decompose(signal_view, buffer_view, wavelet);

    // Convert result back to f64
    signal_f32.iter().map(|&x| x as f64).collect()
}

/// A WaveletTransform must satisfy:
/// - Reversibility: reconstruct(decompose(s)) ≈ s
/// - Energy preservation: sum of squares of approximation + detail ≈ original signal energy
/// - Orthogonality: inner product of different wavelet functions is zero
///
/// Unified interface for wavelet transforms.
pub trait WaveletTransform {
    /// Signal type accepted by the transform.
    type Signal;
    /// Coefficient container returned by the transform.
    type Coefficients;
    /// Error type raised when transform invalidation occurs.
    type Error;

    /// Decompose a signal into coefficients.
    fn decompose(signal: &Self::Signal) -> Result<Self::Coefficients, Self::Error>;
    /// Reconstruct a signal from wavelet coefficients.
    fn reconstruct(coeffs: &Self::Coefficients) -> Result<Self::Signal, Self::Error>;
}

/// A simple signal wrapper around a vector of sample values.
#[derive(Debug, Clone)]
pub struct Signal(Vec<f64>);

/// Haar coefficients split into approximation and detail terms.
#[derive(Debug, Clone)]
pub struct Coefficients {
    /// Approximation coefficients.
    pub approximation: Vec<f64>,
    /// Detail coefficients.
    pub detail: Vec<f64>,
}

/// Failures returned by transform operations.
#[derive(Debug)]
pub enum TransformError {
    /// Signal length is incompatible with the transform.
    InvalidLength,
    /// Reconstruction could not be completed.
    ReconstructionFailed,
}

/// Haar wavelet implementation.
pub struct HaarWavelet;

impl WaveletTransform for HaarWavelet {
    type Signal = Signal;
    type Coefficients = Coefficients;
    type Error = TransformError;

    fn decompose(signal: &Self::Signal) -> Result<Self::Coefficients, Self::Error> {
        let data = &signal.0;
        if data.len() % 2 != 0 {
            return Err(TransformError::InvalidLength);
        }

        let mut approximation = Vec::new();
        let mut detail = Vec::new();

        for i in (0..data.len()).step_by(2) {
            let a = (data[i] + data[i + 1]) / 2.0;
            let d = (data[i] - data[i + 1]) / 2.0;
            approximation.push(a);
            detail.push(d);
        }

        Ok(Coefficients {
            approximation,
            detail,
        })
    }

    fn reconstruct(coeffs: &Self::Coefficients) -> Result<Self::Signal, Self::Error> {
        let mut signal = Vec::new();
        if coeffs.approximation.len() != coeffs.detail.len() {
            return Err(TransformError::ReconstructionFailed);
        }

        for (a, d) in coeffs.approximation.iter().zip(&coeffs.detail) {
            signal.push(a + d);
            signal.push(a - d);
        }

        Ok(Signal(signal))
    }
}

/// Represents the wavelet basis used for decomposition and reconstruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveletBasis {
    /// Standard Haar basis.
    Haar,
    /// Daubechies basis with the given filter order.
    Daubechies(u8),
    /// Biorthogonal basis using analysis and synthesis orders.
    Biorthogonal(u8, u8),
    /// User-defined or experimental basis name.
    Custom(String),
}

/// Trait for fusing wavelet coefficients from multiple bases.
pub trait WaveletFusionStrategy {
    /// Fuse multiple wavelet decompositions into a unified representation.
    fn fuse(
        decompositions: &[WaveletDecomposition],
        context: &FusionContext,
    ) -> WaveletDecomposition;

    /// Optionally score each basis for its semantic fit.
    fn score_basis(basis: &WaveletBasis, signal: &[f64], context: &FusionContext) -> f64;
}

/// Contextual metadata for wavelet fusion.
#[derive(Debug, Clone)]
pub struct FusionContext {
    /// Entropy of the domain or signal.
    pub domain_entropy: f64,
    /// Optional resonance profile used as a weighting signal.
    pub resonance_profile: Option<Vec<f64>>,
    /// Tags describing the semantic domain.
    pub semantic_tags: Vec<String>,
    /// Optional local coherence map for the signal.
    pub coherence_map: Option<Vec<f64>>,
    /// Optional curvature profile used in semantic scoring.
    pub curvature_profile: Option<Vec<f64>>,
    /// Optional domain label such as "biological" or "quantum".
    pub domain_label: Option<String>,
}

// Implement Default for FusionContext
impl Default for FusionContext {
    fn default() -> Self {
        FusionContext {
            domain_entropy: 0.0,
            resonance_profile: None,
            semantic_tags: Vec::new(),
            coherence_map: None,
            curvature_profile: None,
            domain_label: None,
        }
    }
}
/// Fuses decompositions by inversely weighting each coefficient set by entropy.
pub struct EntropyWeightedFusion;

impl WaveletFusionStrategy for EntropyWeightedFusion {
    fn fuse(
        decompositions: &[WaveletDecomposition],
        _context: &FusionContext,
    ) -> WaveletDecomposition {
        if decompositions.is_empty() {
            return WaveletDecomposition {
                basis: WaveletBasis::Custom("EntropyFused".into()),
                coefficients: Vec::new(),
                level: 0,
            };
        }

        let target_len = decompositions
            .iter()
            .map(|decomp| decomp.coefficients.len())
            .max()
            .unwrap_or(0);
        let mut total_weight = vec![0.0; target_len];
        let mut fused_coeffs = vec![0.0; target_len];

        for decomp in decompositions {
            let entropy = compute_entropy(&decomp.coefficients);
            let weight = if entropy > 0.0 && entropy.is_finite() {
                1.0 / entropy
            } else {
                1.0
            };

            for (i, coeff) in decomp.coefficients.iter().enumerate() {
                fused_coeffs[i] += coeff * weight;
                total_weight[i] += weight;
            }
        }

        for (coeff, weight) in fused_coeffs.iter_mut().zip(total_weight.iter()) {
            if *weight > 0.0 {
                *coeff /= *weight;
            } else {
                *coeff = 0.0;
            }
        }

        let level = decompositions
            .iter()
            .map(|decomp| decomp.level)
            .max()
            .unwrap_or(0);

        WaveletDecomposition {
            basis: WaveletBasis::Custom("EntropyFused".into()),
            coefficients: fused_coeffs,
            level,
        }
    }

    fn score_basis(basis: &WaveletBasis, signal: &[f64], _context: &FusionContext) -> f64 {
        let coeffs = match basis {
            WaveletBasis::Haar => haar_transform(signal),
            WaveletBasis::Daubechies(order) => daubechies_transform(signal, *order),
            WaveletBasis::Biorthogonal(a, s) => biorthogonal_transform(signal, *a, *s),
            WaveletBasis::Custom(name) => custom_transform(signal, name),
        };
        let entropy = compute_entropy(&coeffs);
        1.0 / (entropy + 1e-6)
    }
}

/// Compute an entropy-like score for a coefficient vector.
pub fn compute_entropy(coeffs: &[f64]) -> f64 {
    if coeffs.is_empty() {
        return 0.0;
    }

    let norm: f64 = coeffs.iter().map(|c| c.abs()).sum();
    if !norm.is_finite() || norm <= 0.0 {
        return 0.0;
    }

    coeffs
        .iter()
        .map(|c| {
            let p = c.abs() / norm;
            if p > 0.0 && p.is_finite() {
                -p * p.log2()
            } else {
                0.0
            }
        })
        .sum()
}

/*
/// Decomposes a signal using the specified wavelet basis.
/// Returns the wavelet coefficients.
pub fn decompose(signal: &[f64], basis: &WaveletBasis) -> Vec<f64> {
    match basis {
        WaveletBasis::Haar => {
            // Placeholder: Haar decomposition
            haar_transform(signal)
        }
        WaveletBasis::Daubechies(order) => {
            // Placeholder: Daubechies decomposition
            daubechies_transform(signal, *order)
        }
        WaveletBasis::Biorthogonal(a, s) => {
            // Placeholder: Biorthogonal decomposition
            biorthogonal_transform(signal, *a, *s)
        }
        WaveletBasis::Custom(name) => {
            // Placeholder: Custom wavelet
            custom_transform(signal, name)
        }
    }
}
*/

/*
pub fn haar_transform(signal: &[f64]) -> Vec<f64> {
    let mut coeffs = Vec::new();
    let mut i = 0;
    while i + 1 < signal.len() {
        let avg = (signal[i] + signal[i + 1]) / 2.0;
        let diff = (signal[i] - signal[i + 1]) / 2.0;
        coeffs.push(avg);
        coeffs.push(diff);

       i += 2;
    }
    coeffs
}
*/

/// Apply a simple Daubechies-like moving-window transform as a pragmatic baseline.
pub fn daubechies_transform(signal: &[f64], order: u8) -> Vec<f64> {
    let window = order.max(2) as usize;
    let mut coeffs = Vec::new();

    for i in 0..(signal.len().saturating_sub(window)) {
        let slice = &signal[i..i + window];
        let weight = 1.0 / window as f64;
        let avg = slice.iter().map(|x| x * weight).sum::<f64>();
        coeffs.push(avg);
    }

    coeffs
}

/// Apply a biorthogonal-style transform using symmetric analysis and synthesis windows.
pub fn biorthogonal_transform(signal: &[f64], a: u8, s: u8) -> Vec<f64> {
    let analysis_window = a.max(2) as usize;
    let synthesis_window = s.max(2) as usize;
    let mut coeffs = Vec::new();

    for i in 0..(signal.len().saturating_sub(analysis_window)) {
        let slice = &signal[i..i + analysis_window];
        let analysis = slice.iter().sum::<f64>() / analysis_window as f64;

        let synth_start = i.saturating_sub(synthesis_window / 2);
        let synth_end = (synth_start + synthesis_window).min(signal.len());
        let synth_slice = &signal[synth_start..synth_end];
        let synthesis = synth_slice.iter().sum::<f64>() / synthesis_window as f64;

        coeffs.push((analysis + synthesis) / 2.0);
    }

    coeffs
}

/// Apply a simple experimental transform selected by name.
pub fn custom_transform(signal: &[f64], name: &str) -> Vec<f64> {
    match name {
        "identity" => signal.to_vec(),
        "reverse" => signal.iter().rev().cloned().collect(),
        "pulse" => signal.iter().map(|x| x.sin() * x).collect(),
        _ => signal.to_vec(), // fallback
    }
}

/// Fuses decompositions using a resonance-weighted coefficient average.
pub struct ResonanceWeightedFusion;

impl WaveletFusionStrategy for ResonanceWeightedFusion {
    fn fuse(
        decompositions: &[WaveletDecomposition],
        context: &FusionContext,
    ) -> WaveletDecomposition {
        if decompositions.is_empty() {
            return WaveletDecomposition {
                basis: WaveletBasis::Custom("ResonanceFused".into()),
                coefficients: Vec::new(),
                level: 0,
            };
        }

        let resonance = context.resonance_profile.as_ref();
        let len = decompositions
            .iter()
            .map(|decomp| decomp.coefficients.len())
            .max()
            .unwrap_or(0);
        let mut fused = vec![0.0; len];
        let mut total_weight = vec![0.0; len];

        for decomp in decompositions {
            for i in 0..len {
                let coeff = decomp.coefficients.get(i).copied().unwrap_or(0.0);
                let r = resonance.and_then(|rp| rp.get(i)).copied().unwrap_or(1.0);
                fused[i] += coeff * r;
                total_weight[i] += r;
            }
        }

        for i in 0..len {
            fused[i] = if total_weight[i].abs() > 1e-12 {
                fused[i] / total_weight[i]
            } else {
                0.0
            };
        }

        let level = decompositions
            .iter()
            .map(|decomp| decomp.level)
            .max()
            .unwrap_or(0);

        WaveletDecomposition {
            basis: WaveletBasis::Custom("ResonanceFused".into()),
            coefficients: fused,
            level,
        }
    }

    fn score_basis(basis: &WaveletBasis, signal: &[f64], context: &FusionContext) -> f64 {
        let coeffs = match basis {
            WaveletBasis::Haar => haar_transform(signal),
            WaveletBasis::Daubechies(order) => daubechies_transform(signal, *order),
            WaveletBasis::Biorthogonal(a, s) => biorthogonal_transform(signal, *a, *s),
            WaveletBasis::Custom(name) => custom_transform(signal, name),
        };
        let resonance = context.resonance_profile.as_ref();
        coeffs
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let r = resonance.map_or(1.0, |rp| rp[i]);
                r * c.abs()
            })
            .sum::<f64>()
    }
}

/// Engine used to decompose, score, and fuse signals across multiple wavelet bases.
pub struct WaveletEngine<F: WaveletFusionStrategy> {
    /// Set of wavelet bases used for decomposition.
    pub basis_set: Vec<WaveletBasis>,
    /// Strategy used to fuse coefficient sets.
    pub fusion_strategy: F,
}

impl<F: WaveletFusionStrategy> WaveletEngine<F> {
    /// Construct a new engine configured with a basis set and fusion strategy.
    pub fn new(basis_set: Vec<WaveletBasis>, fusion_strategy: F) -> Self {
        Self {
            basis_set,
            fusion_strategy,
        }
    }

    /// Decompose a signal using all bases in the set.
    pub fn decompose_all(&self, signal: &[f64], level: usize) -> Vec<WaveletDecomposition> {
        self.basis_set
            .iter()
            .map(|basis| {
                let coeffs = match basis {
                    WaveletBasis::Haar => haar_transform(signal),
                    WaveletBasis::Daubechies(order) => daubechies_transform(signal, *order),
                    WaveletBasis::Biorthogonal(a, s) => biorthogonal_transform(signal, *a, *s),
                    WaveletBasis::Custom(name) => custom_transform(signal, name),
                };
                WaveletDecomposition {
                    basis: basis.clone(),
                    coefficients: coeffs,
                    level,
                }
            })
            .collect()
    }

    /// Fuse decompositions using the selected strategy.
    pub fn fuse(
        &self,
        signal: &[f64],
        context: &FusionContext,
        level: usize,
    ) -> WaveletDecomposition {
        let decompositions = self.decompose_all(signal, level);
        F::fuse(&decompositions, context)
    }

    /// Score each basis for semantic fit.
    pub fn score_bases(&self, signal: &[f64], context: &FusionContext) -> Vec<(WaveletBasis, f64)> {
        self.basis_set
            .iter()
            .map(|basis| {
                let score = F::score_basis(basis, signal, context);
                (basis.clone(), score)
            })
            .collect()
    }
}

/// Holds wavelet coefficients and metadata.
pub struct WaveletDecomposition {
    /// Wavelet basis used to create the coefficients.
    pub basis: WaveletBasis,
    /// Raw decomposition coefficients.
    pub coefficients: Vec<f64>,
    /// Decomposition level.
    pub level: usize,
}

/// A simple smoothing transform built from repeated pairwise averaging and thresholding.
#[derive(Debug, Clone)]
pub struct WaveletTransformStruct {
    /// Number of smoothing passes to apply.
    pub levels: usize,
    /// Threshold used to zero small differences.
    pub threshold: f64,
}

impl WaveletTransformStruct {
    /// Smooth a signal by repeatedly averaging pairs and thresholding residual detail.
    pub fn smooth(&self, signal: &[f64]) -> Vec<f64> {
        let mut data = signal.to_vec();
        let mut temp = vec![0.0; data.len()];

        for _ in 0..self.levels {
            let mut i = 0;
            while i + 1 < data.len() {
                let avg = (data[i] + data[i + 1]) / 2.0;
                let diff = (data[i] - data[i + 1]) / 2.0;

                temp[i / 2] = avg;
                temp[data.len() / 2 + i / 2] = if diff.abs() > self.threshold {
                    diff
                } else {
                    0.0
                };
                i += 2;
            }
            data = temp.clone();
        }

        // Reconstruct smoothed signal
        let mut recon = vec![0.0; signal.len()];
        let mut i = 0;
        while i + 1 < recon.len() {
            let avg = data[i / 2];
            let diff = data[recon.len() / 2 + i / 2];
            recon[i] = avg + diff;
            recon[i + 1] = avg - diff;
            i += 2;
        }

        recon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_fusion_input_is_safe() {
        let context = FusionContext::default();
        let fused = EntropyWeightedFusion::fuse(&[], &context);

        assert!(fused.coefficients.is_empty());
        assert_eq!(fused.level, 0);
    }

    #[test]
    fn zero_energy_signal_has_zero_entropy() {
        assert_eq!(compute_entropy(&[]), 0.0);
        assert_eq!(compute_entropy(&[0.0, 0.0, 0.0]), 0.0);
    }
}
