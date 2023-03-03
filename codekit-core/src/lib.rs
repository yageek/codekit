mod commons;
mod ean;

pub use commons::{Code, CodeOptions};

#[cfg(feature = "ffi-interface")]
/// The code contains for the FFI
pub mod ffi {
    use std::{ffi::CStr, os::raw::c_char};

    use crate::{commons::Barcode, ean::EAN8, CodeOptions};

    #[repr(C)]
    pub struct CodeDescriptor {
        pub options: CodeOptions,
        pub bars: *mut u8,
        pub bars_count: usize,
    }

    #[no_mangle]
    pub extern "C" fn codekit_free_descriptor(ptr: *mut CodeDescriptor) {
        assert!(!ptr.is_null());
        unsafe {
            drop(Vec::from_raw_parts(
                (*ptr).bars,
                (*ptr).bars_count,
                (*ptr).bars_count,
            ));
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
        let input_string = unsafe { CStr::from_ptr(content) };

        let input = input_string.to_str().unwrap();

        match EAN8::make_descriptor(input, options) {
            Ok(code) => {
                println!("Total bars: {}", code.bars().len());
                unsafe {
                    (*value).options = code.options();
                    (*value).bars_count = code.bars().len();

                    // Now we need to move the elements to the heap
                    let mut vec = code.get_bars();
                    vec.shrink_to_fit();
                    (*value).bars = vec.as_mut_ptr();
                    std::mem::forget(vec);
                }
                0
            }
            Err(e) => {
                println!("Error: {}", e);
                -1
            }
        }
    }
}
