use flock_signal_ffi::{
    fs_channel_frame, fs_engine, fs_engine_create, fs_engine_destroy, fs_engine_extract_template,
    fs_template, fs_template_destroy, fs_template_quality, FS_ERR_INVALID, FS_ERR_NULL, FS_OK,
};
use std::ptr;

#[test]
fn create_rejects_null_output_pointer() {
    assert_eq!(unsafe { fs_engine_create(ptr::null_mut()) }, FS_ERR_NULL);
}

#[test]
fn engine_lifecycle_and_extraction_work() {
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
