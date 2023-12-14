use wasm_bindgen::prelude::*;

use crate::{commons::Barcode, EAN8};


#[wasm_bindgen]
pub fn codekit_create_ean8(code: &str) -> String {
    EAN8::make_descriptor(code).unwrap()
}