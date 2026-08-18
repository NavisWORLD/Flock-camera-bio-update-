use signal_core::{ObservationWindow, SensorFrame, SignalTemplate, SourceKind};
use signal_features::FeatureExtractor;
use std::{
    collections::BTreeMap,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr,
    slice,
};
use uuid::Uuid;

pub const FS_OK: i32 = 0;
pub const FS_ERR_NULL: i32 = -1;
pub const FS_ERR_INVALID: i32 = -2;
pub const FS_ERR_INTERNAL: i32 = -3;

#[repr(C)]
pub struct fs_channel_frame {
    pub samples: *const f32,
    pub sample_count: usize,
    pub sample_rate_hz: f64,
    pub timestamp_ns: i64,
}

#[repr(C)]
pub struct fs_engine {
    extractor: FeatureExtractor,
}

#[repr(C)]
pub struct fs_template {
    inner: SignalTemplate,
}

#[no_mangle]
pub unsafe extern "C" fn fs_engine_create(out_engine: *mut *mut fs_engine) -> i32 {
    if out_engine.is_null() {
        return FS_ERR_NULL;
    }
    match catch_unwind(AssertUnwindSafe(|| {
        let engine = Box::new(fs_engine {
            extractor: FeatureExtractor::default(),
        });
        unsafe { ptr::write(out_engine, Box::into_raw(engine)) };
    })) {
        Ok(()) => FS_OK,
        Err(_) => FS_ERR_INTERNAL,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fs_engine_destroy(engine: *mut fs_engine) {
    if engine.is_null() {
        return;
    }
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        drop(Box::from_raw(engine));
    }));
}

#[no_mangle]
pub unsafe extern "C" fn fs_engine_extract_template(
    engine: *mut fs_engine,
    channels: *const fs_channel_frame,
    channel_count: usize,
    out_template: *mut *mut fs_template,
) -> i32 {
    if engine.is_null() || channels.is_null() || out_template.is_null() {
        return FS_ERR_NULL;
    }
    if channel_count == 0 {
        return FS_ERR_INVALID;
    }

    match catch_unwind(AssertUnwindSafe(|| {
        let engine_ref = unsafe { &*engine };
        let channel_frames = unsafe { slice::from_raw_parts(channels, channel_count) };
        let first = &channel_frames[0];
        if !first.sample_rate_hz.is_finite() || first.sample_rate_hz <= 0.0 {
            return Err(FS_ERR_INVALID);
        }

        let mut copied_channels = Vec::with_capacity(channel_count);
        for frame in channel_frames {
            if frame.sample_count == 0 || frame.samples.is_null() {
                return Err(FS_ERR_INVALID);
            }
            if !frame.sample_rate_hz.is_finite()
                || frame.sample_rate_hz <= 0.0
                || (frame.sample_rate_hz - first.sample_rate_hz).abs() > f64::EPSILON
            {
                return Err(FS_ERR_INVALID);
            }
            let samples = unsafe { slice::from_raw_parts(frame.samples, frame.sample_count) };
            if samples.iter().any(|sample| !sample.is_finite()) {
                return Err(FS_ERR_INVALID);
            }
            copied_channels.push(samples.to_vec());
        }

        let observation = ObservationWindow {
            window_id: Uuid::new_v4(),
            start_ns: first.timestamp_ns as i128,
            end_ns: first.timestamp_ns as i128,
            frames: vec![SensorFrame {
                sensor_id: "native-ffi".into(),
                source_kind: SourceKind::Generic,
                timestamp_ns: first.timestamp_ns as i128,
                sample_rate_hz: first.sample_rate_hz,
                channels: copied_channels,
                metadata: BTreeMap::new(),
            }],
        };
        let template = engine_ref
            .extractor
            .extract(&observation)
            .map_err(|_| FS_ERR_INVALID)?;
        let boxed = Box::new(fs_template { inner: template });
        unsafe { ptr::write(out_template, Box::into_raw(boxed)) };
        Ok(FS_OK)
    })) {
        Ok(Ok(code)) => code,
        Ok(Err(code)) => code,
        Err(_) => FS_ERR_INTERNAL,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fs_template_quality(value: *const fs_template) -> f32 {
    if value.is_null() {
        return 0.0;
    }
    catch_unwind(AssertUnwindSafe(|| unsafe { (&*value).inner.quality })).unwrap_or(0.0)
}

#[no_mangle]
pub unsafe extern "C" fn fs_template_destroy(value: *mut fs_template) {
    if value.is_null() {
        return;
    }
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        drop(Box::from_raw(value));
    }));
}
