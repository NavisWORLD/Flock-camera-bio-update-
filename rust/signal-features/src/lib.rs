use rustfft::{num_complex::Complex32, FftPlanner};
use sha2::{Digest, Sha256};
use signal_core::{ObservationWindow, SignalTemplate};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FeatureError {
    #[error("observation contains no samples")]
    NoSamples,
    #[error("sample rate must be finite and positive")]
    InvalidSampleRate,
    #[error("observation could not be serialized for provenance hashing")]
    Serialization,
}

#[derive(Debug, Clone)]
pub struct FeatureExtractor {
    pub schema: String,
    pub version: String,
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self {
            schema: "flock-signal-basic-v1".into(),
            version: "1.0.0".into(),
        }
    }
}

impl FeatureExtractor {
    pub fn extract(&self, window: &ObservationWindow) -> Result<SignalTemplate, FeatureError> {
        let (sample_rate, samples) = first_channel(window).ok_or(FeatureError::NoSamples)?;
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Err(FeatureError::InvalidSampleRate);
        }

        let finite_count = samples.iter().filter(|v| v.is_finite()).count();
        if finite_count == 0 {
            return Err(FeatureError::NoSamples);
        }
        let quality = finite_count as f32 / samples.len() as f32;
        let clean = samples
            .iter()
            .map(|v| if v.is_finite() { *v } else { 0.0 })
            .collect::<Vec<_>>();

        let mean = clean.iter().sum::<f32>() / clean.len() as f32;
        let centered = clean.iter().map(|v| *v - mean).collect::<Vec<_>>();
        let rms = (centered.iter().map(|v| v * v).sum::<f32>() / centered.len() as f32).sqrt();
        let zcr = zero_crossing_rate(&centered);
        let (centroid_hz, low_energy, mid_energy, high_energy) =
            spectral_summary(&centered, sample_rate as f32);

        let serialized = serde_json::to_vec(window).map_err(|_| FeatureError::Serialization)?;
        let digest: [u8; 32] = Sha256::digest(serialized).into();

        Ok(SignalTemplate::new(
            self.schema.clone(),
            self.version.clone(),
            vec![rms, zcr, centroid_hz, low_energy, mid_energy, high_energy],
            quality,
            1.0 - quality,
            digest,
        ))
    }
}

fn first_channel(window: &ObservationWindow) -> Option<(f64, &[f32])> {
    window.frames.iter().find_map(|frame| {
        frame
            .channels
            .iter()
            .find(|channel| !channel.is_empty())
            .map(|channel| (frame.sample_rate_hz, channel.as_slice()))
    })
}

fn zero_crossing_rate(samples: &[f32]) -> f32 {
    if samples.len() < 2 {
        return 0.0;
    }
    let crossings = samples
        .windows(2)
        .filter(|pair| (pair[0] >= 0.0) != (pair[1] >= 0.0))
        .count();
    crossings as f32 / (samples.len() - 1) as f32
}

fn spectral_summary(samples: &[f32], sample_rate_hz: f32) -> (f32, f32, f32, f32) {
    let n = samples.len();
    if n == 0 {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let mut buffer = samples
        .iter()
        .enumerate()
        .map(|(i, sample)| {
            let weight = if n == 1 {
                1.0
            } else {
                0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos()
            };
            Complex32::new(sample * weight, 0.0)
        })
        .collect::<Vec<_>>();

    let mut planner = FftPlanner::<f32>::new();
    planner.plan_fft_forward(n).process(&mut buffer);

    let half = n / 2 + 1;
    let bin_hz = sample_rate_hz / n as f32;
    let nyquist = sample_rate_hz / 2.0;
    let mut magnitude_sum = 0.0;
    let mut weighted_sum = 0.0;
    let mut low = 0.0;
    let mut mid = 0.0;
    let mut high = 0.0;

    for (i, value) in buffer.iter().take(half).enumerate() {
        let magnitude = value.norm();
        let frequency = i as f32 * bin_hz;
        magnitude_sum += magnitude;
        weighted_sum += frequency * magnitude;
        let energy = magnitude * magnitude;
        if frequency < nyquist / 3.0 {
            low += energy;
        } else if frequency < 2.0 * nyquist / 3.0 {
            mid += energy;
        } else {
            high += energy;
        }
    }

    let centroid = if magnitude_sum > f32::EPSILON {
        weighted_sum / magnitude_sum
    } else {
        0.0
    };
    let total_energy = (low + mid + high).max(f32::EPSILON);
    (
        centroid,
        low / total_energy,
        mid / total_energy,
        high / total_energy,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_core::{ObservationWindow, SensorFrame, SourceKind};
    use std::collections::BTreeMap;

    fn sine_window() -> ObservationWindow {
        let sample_rate = 64.0;
        let samples = (0..64)
            .map(|i| (2.0 * std::f32::consts::PI * 8.0 * i as f32 / sample_rate as f32).sin())
            .collect::<Vec<_>>();
        ObservationWindow::new(
            0,
            1_000_000_000,
            vec![SensorFrame {
                sensor_id: "sim-1".into(),
                source_kind: SourceKind::Mechanical,
                timestamp_ns: 0,
                sample_rate_hz: sample_rate,
                channels: vec![samples],
                metadata: BTreeMap::new(),
            }],
        )
    }

    #[test]
    fn extracts_stable_features_for_known_sine_wave() {
        let window = sine_window();
        let a = FeatureExtractor::default()
            .extract(&window)
            .expect("extract");
        let b = FeatureExtractor::default()
            .extract(&window)
            .expect("extract");
        assert_eq!(a.feature_schema, "flock-signal-basic-v1");
        assert_eq!(a.features, b.features);
        assert_eq!(a.source_digest, b.source_digest);
        assert!(
            (a.features[2] - 8.0).abs() < 0.75,
            "centroid should be near 8 Hz"
        );
    }

    #[test]
    fn rejects_empty_observation_windows() {
        let window = ObservationWindow::new(0, 1, Vec::new());
        assert!(matches!(
            FeatureExtractor::default().extract(&window),
            Err(FeatureError::NoSamples)
        ));
    }

    #[test]
    fn quality_and_uncertainty_are_bounded() {
        let template = FeatureExtractor::default()
            .extract(&sine_window())
            .expect("extract");
        assert!((0.0..=1.0).contains(&template.quality));
        assert!((0.0..=1.0).contains(&template.uncertainty));
    }
}
