use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use crate::{
    codabar::Codabar,
    code39::Code39,
    code93::Code93,
    commons::Barcode,
    ean::{EAN13, EAN8},
    i2of5::I2of5,
};

/// A descriptors holding all the
/// informations to draw a code
#[repr(C)]
pub struct CodeDescriptor {
    /// A pointer in memory to an array
    /// of byte where each one represent either a blank (0) or black (1) bar
    pub bars: *mut c_char,
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
    value: *mut CodeDescriptor,
) -> i8 {
    create_code_from_str::<EAN8>(content, value)
}

/// Create a descriptor for EAN8 code
#[no_mangle]
pub extern "C" fn codekit_code_create_ean13(
    content: *const c_char,
    value: *mut CodeDescriptor,
) -> i8 {
    create_code_from_str::<EAN13>(content, value)
}

/// Create a descriptor for a Code39 code.
#[no_mangle]
pub extern "C" fn codekit_code_create_code39(
    content: *const c_char,
    value: *mut CodeDescriptor,
) -> i8 {
    create_code_from_str::<Code39>(content, value)
}

/// Create a descriptor for a Code93 code.
#[no_mangle]
pub extern "C" fn codekit_code_create_code93(
    content: *const c_char,
    value: *mut CodeDescriptor,
) -> i8 {
    create_code_from_str::<Code93>(content, value)
}

/// Create a descriptor for a Codabar code.
#[no_mangle]
pub extern "C" fn codekit_code_create_codabar(
    content: *const c_char,
    value: *mut CodeDescriptor,
) -> i8 {
    create_code_from_str::<Codabar>(content, value)
}

/// Create a descriptor for a Interleaved code.
#[no_mangle]
pub extern "C" fn codekit_code_create_i2of5(
    content: *const c_char,
    value: *mut CodeDescriptor,
) -> i8 {
    create_code_from_str::<I2of5>(content, value)
}

/// Internal generic method
fn create_code_from_str<T>(content: *const c_char, value: *mut CodeDescriptor) -> i8
where
    T: Barcode,
{
    assert!(!content.is_null());
    // We need to convert a string from C world
    let input_string = unsafe { CStr::from_ptr(content) };

    let input = input_string.to_str().unwrap();

    match T::make_descriptor(input) {
        Ok(code) => {
            unsafe {
                (*value).bars_count = code.len();

                // Now we need to move the elements to the heap

                let code_value = CString::new(code).unwrap();
                (*value).bars = code_value.into_raw();
            }
            0
        }
        Err(_e) => -1,
    }
}
