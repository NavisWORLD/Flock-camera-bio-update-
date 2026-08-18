#![allow(non_camel_case_types)]

use signal_core::{ObservationWindow, SensorFrame, SignalTemplate, SourceKind};
use signal_features::FeatureExtractor;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

pub const FS_OK: i32 = 0;
pub const FS_ERR_NULL: i32 = 1;
pub const FS_ERR_INPUT: i32 = 2;
pub const FS_ERR_FEATURE: i32 = 3;
pub const FS_ERR_PANIC: i32 = 255;

pub struct fs_engine {
    extractor: FeatureExtractor,
}

pub struct fs_template {
    inner: SignalTemplate,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct fs_channel_frame {
    pub samples: *const f32,
    pub sample_count: usize,
    pub sample_rate_hz: f64,
    pub timestamp_ns: i64,
}

#[no_mangle]
pub unsafe extern "C" fn fs_engine_create(out_engine: *mut *mut fs_engine) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        if out_engine.is_null() {
            return FS_ERR_NULL;
        }
        let engine = Box::new(fs_engine {
            extractor: FeatureExtractor::default(),
        });
        unsafe {
            *out_engine = Box::into_raw(engine);
        }
        FS_OK
    }))
    .unwrap_or(FS_ERR_PANIC)
}

#[no_mangle]
pub unsafe extern "C" fn fs_engine_destroy(engine: *mut fs_engine) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !engine.is_null() {
            unsafe {
                drop(Box::from_raw(engine));
            }
        }
    }));
}

#[no_mangle]
pub unsafe extern "C" fn fs_engine_extract_template(
    engine: *mut fs_engine,
    channels: *const fs_channel_frame,
    channel_count: usize,
    out_template: *mut *mut fs_template,
) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        if engine.is_null() || channels.is_null() || out_template.is_null() {
            return FS_ERR_NULL;
        }
        if channel_count == 0 {
            return FS_ERR_INPUT;
        }

        let channel_slice = unsafe { std::slice::from_raw_parts(channels, channel_count) };
        let mut frames = Vec::with_capacity(channel_count);
        for (index, channel) in channel_slice.iter().enumerate() {
            if channel.samples.is_null()
                || channel.sample_count == 0
                || !channel.sample_rate_hz.is_finite()
                || channel.sample_rate_hz <= 0.0
            {
                return FS_ERR_INPUT;
            }
            let samples = unsafe { std::slice::from_raw_parts(channel.samples, channel.sample_count) }.to_vec();
            frames.push(SensorFrame {
                sensor_id: format!("ffi-channel-{index}"),
                source_kind: SourceKind::Other,
                timestamp_ns: channel.timestamp_ns as i128,
                sample_rate_hz: channel.sample_rate_hz,
                channels: vec![samples],
                metadata: BTreeMap::new(),
            });
        }

        let start_ns = frames.iter().map(|f| f.timestamp_ns).min().unwrap_or(0);
        let end_ns = frames.iter().map(|f| f.timestamp_ns).max().unwrap_or(start_ns);
        let window = ObservationWindow::new(start_ns, end_ns, frames);
        let extractor = unsafe { &(*engine).extractor };
        let template = match extractor.extract(&window) {
            Ok(value) => value,
            Err(_) => return FS_ERR_FEATURE,
        };
        unsafe {
            *out_template = Box::into_raw(Box::new(fs_template { inner: template }));
        }
        FS_OK
    }))
    .unwrap_or(FS_ERR_PANIC)
}

#[no_mangle]
pub unsafe extern "C" fn fs_template_destroy(value: *mut fs_template) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !value.is_null() {
            unsafe {
                drop(Box::from_raw(value));
            }
        }
    }));
}

#[no_mangle]
pub unsafe extern "C" fn fs_template_quality(value: *const fs_template, out_quality: *mut f32) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        if value.is_null() || out_quality.is_null() {
            return FS_ERR_NULL;
        }
        unsafe {
            *out_quality = (*value).inner.quality;
        }
        FS_OK
    }))
    .unwrap_or(FS_ERR_PANIC)
}

#[no_mangle]
pub unsafe extern "C" fn fs_template_uncertainty(
    value: *const fs_template,
    out_uncertainty: *mut f32,
) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        if value.is_null() || out_uncertainty.is_null() {
            return FS_ERR_NULL;
        }
        unsafe {
            *out_uncertainty = (*value).inner.uncertainty;
        }
        FS_OK
    }))
    .unwrap_or(FS_ERR_PANIC)
}

#[no_mangle]
pub unsafe extern "C" fn fs_template_free_and_null(value: *mut *mut fs_template) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        if value.is_null() {
            return FS_ERR_NULL;
        }
        unsafe {
            let inner = *value;
            if !inner.is_null() {
                drop(Box::from_raw(inner));
            }
            *value = ptr::null_mut();
        }
        FS_OK
    }))
    .unwrap_or(FS_ERR_PANIC)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_destroy_engine() {
        let mut engine: *mut fs_engine = ptr::null_mut();
        unsafe {
            assert_eq!(fs_engine_create(&mut engine), FS_OK);
            assert!(!engine.is_null());
            fs_engine_destroy(engine);
        }
    }

    #[test]
    fn null_engine_output_is_rejected() {
        unsafe {
            assert_eq!(fs_engine_create(ptr::null_mut()), FS_ERR_NULL);
        }
    }

    #[test]
    fn synthetic_channel_extracts_bounded_template() {
        let samples = [0.0_f32, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0];
        let frame = fs_channel_frame {
            samples: samples.as_ptr(),
            sample_count: samples.len(),
            sample_rate_hz: 8.0,
            timestamp_ns: 0,
        };
        let mut engine: *mut fs_engine = ptr::null_mut();
        let mut template: *mut fs_template = ptr::null_mut();
        unsafe {
            assert_eq!(fs_engine_create(&mut engine), FS_OK);
            assert_eq!(fs_engine_extract_template(engine, &frame, 1, &mut template), FS_OK);
            let mut quality = -1.0;
            assert_eq!(fs_template_quality(template, &mut quality), FS_OK);
            assert!((0.0..=1.0).contains(&quality));
            fs_template_destroy(template);
            fs_engine_destroy(engine);
        }
    }
}
