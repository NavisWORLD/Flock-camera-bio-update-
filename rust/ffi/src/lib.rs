#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn create_and_destroy_engine() {
        let mut engine: *mut fs_engine = ptr::null_mut();
        assert_eq!(fs_engine_create(&mut engine), FS_OK);
        assert!(!engine.is_null());
        fs_engine_destroy(engine);
    }

    #[test]
    fn null_engine_output_is_rejected() {
        assert_eq!(fs_engine_create(ptr::null_mut()), FS_ERR_NULL);
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
        assert_eq!(fs_engine_create(&mut engine), FS_OK);
        assert_eq!(fs_engine_extract_template(engine, &frame, 1, &mut template), FS_OK);
        let mut quality = -1.0;
        assert_eq!(fs_template_quality(template, &mut quality), FS_OK);
        assert!((0.0..=1.0).contains(&quality));
        fs_template_destroy(template);
        fs_engine_destroy(engine);
    }
}
