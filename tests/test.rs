use std::ffi::{CStr};

use vtflib_sys::*;

unsafe fn get_error<'a>() -> &'a CStr {
    CStr::from_ptr(vlGetLastError())
}

macro_rules! assert_success {
    ($expr:expr) => {
        if $expr == vlFalse {
            panic!("failed with: {}", get_error().to_string_lossy());
        }
    };
}

#[test]
fn create_save_image() {
    unsafe {
        assert_success!(vlInitialize());

        let mut image_handle = 0;
        assert_success!(vlCreateImage(&mut image_handle));
        assert_eq!(vlImageIsBound(), vlFalse);
        assert_success!(vlBindImage(image_handle));
        assert_eq!(vlImageIsBound(), vlTrue);

        assert_eq!(vlImageIsLoaded(), vlFalse);
        assert_success!(vlImageCreate(512, 512, 1,1, 1, tagVTFImageFormat_IMAGE_FORMAT_RGBA8888, vlTrue, vlTrue, vlFalse));
        assert_eq!(vlImageIsLoaded(), vlTrue);

        let size = vlImageGetSize();
        let mut buffer = Vec::with_capacity(size as usize);
        let mut written_size = 0;
        assert_success!(vlImageSaveLump(buffer.as_mut_ptr(), size, &mut written_size));
        assert!(written_size <= size);
        buffer.set_len(written_size as usize);
        assert!(!buffer.is_empty());

        vlImageDestroy();
        assert_eq!(vlImageIsLoaded(), vlFalse);

        vlDeleteImage(image_handle);
        assert_eq!(vlImageIsBound(), vlFalse);

        vlShutdown();
    }
}
