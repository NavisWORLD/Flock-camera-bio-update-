use flock_signal_ffi::{
    fs_channel_frame, fs_engine, fs_engine_create, fs_engine_destroy, fs_engine_extract_template,
    fs_template, fs_template_copy_features, fs_template_copy_source_digest, fs_template_destroy,
    fs_template_feature_count, fs_template_quality, FS_ERR_INVALID, FS_ERR_NULL, FS_OK,
};
use std::ptr;

#[test]
fn create_rejects_null_output_pointer() {
    assert_eq!(unsafe { fs_engine_create(ptr::null_mut()) }, FS_ERR_NULL);
}

#[test]
fn engine_lifecycle_extraction_and_template_access_work() {
    let mut engine: *mut fs_engine = ptr::null_mut();
    assert_eq!(unsafe { fs_engine_create(&mut engine) }, FS_OK);
    assert!(!engine.is_null());

    let samples = [1.0_f32, -1.0, 1.0, -1.0];
    let frame = fs_channel_frame {
        samples: samples.as_ptr(),
        sample_count: samples.len(),
        sample_rate_hz: 4.0,
        timestamp_ns: 42,
    };
    let mut template: *mut fs_template = ptr::null_mut();
    assert_eq!(
        unsafe { fs_engine_extract_template(engine, &frame, 1, &mut template) },
        FS_OK
    );
    assert!(!template.is_null());
    assert!(unsafe { fs_template_quality(template) } > 0.0);

    let count = unsafe { fs_template_feature_count(template) };
    assert_eq!(count, 3);
    let mut features = vec![0.0_f32; count];
    assert_eq!(
        unsafe { fs_template_copy_features(template, features.as_mut_ptr(), features.len()) },
        FS_OK
    );
    assert!((features[0] - 1.0).abs() < 1e-6);

    let mut digest = [0_u8; 32];
    assert_eq!(
        unsafe { fs_template_copy_source_digest(template, digest.as_mut_ptr(), digest.len()) },
        FS_OK
    );
    assert_ne!(digest, [0_u8; 32]);

    unsafe {
        fs_template_destroy(template);
        fs_engine_destroy(engine);
    }
}

#[test]
fn copy_accessors_reject_too_small_buffers() {
    let mut engine: *mut fs_engine = ptr::null_mut();
    assert_eq!(unsafe { fs_engine_create(&mut engine) }, FS_OK);
    let samples = [1.0_f32, -1.0, 1.0, -1.0];
    let frame = fs_channel_frame {
        samples: samples.as_ptr(),
        sample_count: samples.len(),
        sample_rate_hz: 4.0,
        timestamp_ns: 42,
    };
    let mut template: *mut fs_template = ptr::null_mut();
    assert_eq!(
        unsafe { fs_engine_extract_template(engine, &frame, 1, &mut template) },
        FS_OK
    );

    let mut features = [0.0_f32; 2];
    assert_eq!(
        unsafe { fs_template_copy_features(template, features.as_mut_ptr(), features.len()) },
        FS_ERR_INVALID
    );
    let mut digest = [0_u8; 31];
    assert_eq!(
        unsafe { fs_template_copy_source_digest(template, digest.as_mut_ptr(), digest.len()) },
        FS_ERR_INVALID
    );

    unsafe {
        fs_template_destroy(template);
        fs_engine_destroy(engine);
    }
}

#[test]
fn extraction_rejects_nonempty_channel_with_null_samples_and_clears_output() {
    let mut engine: *mut fs_engine = ptr::null_mut();
    assert_eq!(unsafe { fs_engine_create(&mut engine) }, FS_OK);
    let frame = fs_channel_frame {
        samples: ptr::null(),
        sample_count: 4,
        sample_rate_hz: 4.0,
        timestamp_ns: 42,
    };
    let mut template = 1usize as *mut fs_template;
    assert_eq!(
        unsafe { fs_engine_extract_template(engine, &frame, 1, &mut template) },
        FS_ERR_INVALID
    );
    assert!(template.is_null());
    unsafe { fs_engine_destroy(engine) };
}
