mod commons;
mod ean;

pub use commons::{Code, CodeOptions};

#[cfg(feature = "ffi-interface")]
/// The code contains for the FFI
pub mod ffi {
    use std::{ffi::CString, os::raw::c_char};

    use crate::{commons::Barcode, ean::EAN8, CodeOptions};

    #[repr(C)]
    pub struct CodeDescriptor {
        pub options: CodeOptions,
        pub bars: *const u8,
        pub bars_count: usize,
        inner: *mut Vec<u8>,
    }

    #[no_mangle]
    pub extern "C" fn codekit_free_descriptor(ptr: *mut CodeDescriptor) {
        assert!(!ptr.is_null());
        unsafe {
            Box::from_raw((*ptr).inner);
        }
    }

    #[no_mangle]
    pub extern "C" fn codekit_code_create_EAN8(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8 {
        assert!(!content.is_null());
        // We need to convert a string from C world
        let input_string = unsafe { CString::from_raw(content as *mut c_char) };
        let input = input_string.to_str().unwrap();

        match EAN8::make_descriptor(input, options) {
            Ok(code) => {
                unsafe {
                    (*value).options = code.options();
                    (*value).bars_count = code.bars().len();

                    // Now we need to move the elements to the heap
                    let vec = code.get_bars();
                    (*value).bars = vec.as_ptr();

                    Box::into_raw(Box::new(vec));
                }
                0
            }
            Err(_e) => -1,
        }
    }
}
