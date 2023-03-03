mod codabar;
mod code39;
mod code93;
mod commons;
mod ean;
mod i2of5;

pub use commons::{Code, CodeOptions};

#[cfg(feature = "ffi-interface")]
/// The code contains for the FFI
pub mod ffi {
    use std::{ffi::CStr, os::raw::c_char};

    use crate::{
        codabar::Codabar,
        code39::Code39,
        code93::Code93,
        commons::Barcode,
        ean::{EAN13, EAN8},
        i2of5::I2of5,
        CodeOptions,
    };

    /// A descriptors holding all the
    /// informations to draw a code
    #[repr(C)]
    pub struct CodeDescriptor {
        /// The options used to draw the code
        pub options: CodeOptions,
        /// A pointer in memory to an array
        /// of byte where each one represent either a blank (0) or black (1) bar
        pub bars: *mut u8,
        /// The total number of bars stored in memory
        pub bars_count: usize,
    }

    /// Free a code descriptor from a pointer
    /// to CodeDecriptor
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

    /// Create a descriptor for EAN8 code
    #[no_mangle]
    pub extern "C" fn codekit_code_create_ean8(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8 {
        create_code_from_str::<EAN8>(content, options, value)
    }

    /// Create a descriptor for EAN8 code
    #[no_mangle]
    pub extern "C" fn codekit_code_create_ean13(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8 {
        create_code_from_str::<EAN13>(content, options, value)
    }

    /// Create a descriptor for a Code39 code.
    #[no_mangle]
    pub extern "C" fn codekit_code_create_code39(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8 {
        create_code_from_str::<Code39>(content, options, value)
    }

    /// Create a descriptor for a Code93 code.
    #[no_mangle]
    pub extern "C" fn codekit_code_create_code93(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8 {
        create_code_from_str::<Code93>(content, options, value)
    }

    /// Create a descriptor for a Codabar code.
    #[no_mangle]
    pub extern "C" fn codekit_code_create_codabar(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8 {
        create_code_from_str::<Codabar>(content, options, value)
    }

    /// Create a descriptor for a Interleaved code.
    #[no_mangle]
    pub extern "C" fn codekit_code_create_i2of5(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8 {
        create_code_from_str::<I2of5>(content, options, value)
    }

    /// Internal generic method
    fn create_code_from_str<'a, T>(
        content: *const c_char,
        options: CodeOptions,
        value: *mut CodeDescriptor,
    ) -> i8
    where
        T: Barcode<Input = &'a str>,
    {
        assert!(!content.is_null());
        // We need to convert a string from C world
        let input_string = unsafe { CStr::from_ptr(content) };

        let input = input_string.to_str().unwrap();

        match T::make_descriptor(input, options) {
            Ok(code) => {
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
            Err(_e) => -1,
        }
    }
}
