mod codabar;
mod code39;
mod code93;
mod commons;
mod ean;
mod i2of5;

pub use commons::{Code, CodeOptions};

#[cfg(feature = "ffi-interface")]
/// The code contains for the FFI
pub mod ffi;

#[cfg(feature = "jni-interface")]
pub mod jvm;
