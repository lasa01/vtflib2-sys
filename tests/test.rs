use std::ffi::CStr;

use vtflib2_sys::*;

unsafe fn get_error<'a>(error: *const VTFLibError) -> &'a CStr {
    CStr::from_ptr(vlGetErrorMessage(error))
}

macro_rules! assert_success {
    ($expr:expr, $error:expr) => {
        if $expr == vlFalse {
            panic!("failed with: {}", get_error($error).to_string_lossy());
        }
    };
}

macro_rules! assert_failure {
    ($expr:expr, $error:expr) => {
        if $expr == vlTrue {
            panic!("succeeded, expected failure");
        } else {
            eprintln!("failed with: {}", get_error($error).to_string_lossy());
        }
    };
}

#[test]
fn create_save_image() {
    unsafe {
        let error = vlCreateVTFLibError();
        let vtf = vlCreateVTFFile();

        assert_eq!(vlImageIsLoaded(vtf), vlFalse);
        assert_success!(
            vlImageCreate(
                vtf,
                512,
                512,
                1,
                1,
                1,
                tagVTFImageFormat_IMAGE_FORMAT_RGBA8888,
                vlTrue,
                vlTrue,
                vlFalse,
                error,
            ),
            error
        );
        assert_eq!(vlImageIsLoaded(vtf), vlTrue);

        let size = vlImageGetSize(vtf);
        let mut buffer = Vec::with_capacity(size as usize);
        let mut written_size = 0;
        assert_success!(
            vlImageSaveLump(vtf, buffer.as_mut_ptr(), size, &mut written_size, error),
            error
        );
        assert!(written_size <= size);
        buffer.set_len(written_size as usize);
        assert!(!buffer.is_empty());

        vlDestroyVTFFile(vtf);
        vlDestroyVTFLibError(error);
    }
}

#[test]
fn create_image_error() {
    unsafe {
        let error = vlCreateVTFLibError();
        let vtf = vlCreateVTFFile();

        assert_failure!(
            vlImageCreate(
                vtf,
                0,
                1024,
                1,
                1,
                1,
                tagVTFImageFormat_IMAGE_FORMAT_RGBA8888,
                vlTrue,
                vlTrue,
                vlFalse,
                error,
            ),
            error
        );

        vlDestroyVTFFile(vtf);
        vlDestroyVTFLibError(error);
    }
}
