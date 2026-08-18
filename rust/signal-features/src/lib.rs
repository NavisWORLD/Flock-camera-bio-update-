#![forbid(unsafe_code)]

use rustfft::{num_complex::Complex, FftPlanner};
use signal_core::{digest_observation, ObservationWindow, SignalTemplate};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum FeatureError {
    #[error("observation contains no frames")]
    EmptyObservation,
    #[error("observation contains no samples")]
    EmptyChannel,
    #[error("sample rate must be finite and greater than zero")]
    InvalidSampleRate,
    #[error("samples must be finite")]
    NonFiniteSample,
}

#[derive(Clone, Debug, Default)]
pub struct FeatureExtractor;

impl FeatureExtractor {
    pub fn extract(&self, window: &ObservationWindow) -> Result<SignalTemplate, FeatureError> {
        let frame = window.frames.first().ok_or(FeatureError::EmptyObservation)?;
        if !frame.sample_rate_hz.is_finite() || frame.sample_rate_hz <= 0.0 {
            return Err(FeatureError::InvalidSampleRate);
        }
        let samples = frame.channels.first().ok_or(FeatureError::EmptyChannel)?;
        if samples.is_empty() {
            return Err(FeatureError::EmptyChannel);
        }
        if samples.iter().any(|sample| !sample.is_finite()) {
            return Err(FeatureError::NonFiniteSample);
        }

        let rms = rms(samples);
        let zcr = zero_crossing_rate(samples);
        let centroid = spectral_centroid(samples, frame.sample_rate_hz);
        let quality = ((samples.len() as f32) / 16.0).clamp(0.05, 1.0);

        Ok(SignalTemplate {
            template_id: Uuid::new_v4(),
            feature_schema: "fs-basic".to_string(),
            feature_version: "1".to_string(),
            features: vec![rms, zcr, centroid],
            quality,
            uncertainty: 1.0 - quality,
            source_digest: digest_observation(window),
        })
    }
}

fn rms(samples: &[f32]) -> f32 {
    let sum = samples
        .iter()
        .map(|sample| (*sample as f64) * (*sample as f64))
        .sum::<f64>();
    (sum / samples.len() as f64).sqrt() as f32
}

fn zero_crossing_rate(samples: &[f32]) -> f32 {
    if samples.len() < 2 {
        return 0.0;
    }
    let crossings = samples
        .windows(2)
        .filter(|pair| {
            (pair[0] < 0.0 && pair[1] >= 0.0) || (pair[0] >= 0.0 && pair[1] < 0.0)
        })
        .count();
    crossings as f32 / (samples.len() - 1) as f32
}

fn spectral_centroid(samples: &[f32], sample_rate_hz: f64) -> f32 {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(samples.len());
    let mut buffer: Vec<Complex<f32>> = samples
        .iter()
        .map(|sample| Complex::new(*sample, 0.0))
        .collect();
    fft.process(&mut buffer);

    let bins = samples.len() / 2 + 1;
    let bin_hz = sample_rate_hz as f32 / samples.len() as f32;
    let mut magnitude_sum = 0.0_f32;
    let mut weighted_sum = 0.0_f32;

    for (index, value) in buffer.iter().take(bins).enumerate() {
        let magnitude = value.norm();
        magnitude_sum += magnitude;
        weighted_sum += magnitude * (index as f32 * bin_hz);
    }

    if magnitude_sum <= f32::EPSILON {
        0.0
    } else {
        weighted_sum / magnitude_sum
    }
}
