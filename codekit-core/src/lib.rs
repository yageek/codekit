// #![deny(elided_lifetimes_in_paths)]

mod codabar;
mod code39;
mod code93;
mod commons;
mod ean;
mod i2of5;

pub use codabar::{Codabar, CodabarError};
pub use code39::{Code39, Code39Error};
pub use code93::{Code93, Code93Error};
pub use commons::{Code, CodeOptions};
pub use ean::{EANParseError, EAN13, EAN8};
pub use i2of5::{I2of5, I2of5Error};

#[cfg(feature = "ffi-interface")]
pub mod ffi;

#[cfg(feature = "jni-interface")]
pub mod jvm;
