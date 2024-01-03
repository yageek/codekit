use wasm_bindgen::prelude::*;
use crate::{commons::Barcode, Codabar, Code39, Code93, I2of5, EAN13, EAN8};

fn codekit_create<B>(code: &str) -> Result<String, JsError> where B: Barcode {
    let code = B::make_descriptor(code)?;
    Ok(code)
}

macro_rules! wasm_call {
    ($t:ty) => {
        paste::item! {
        #[wasm_bindgen]
        pub fn [< codekit_create_$t:lower >](
            code: &str            
        ) -> Result<String, JsError> {
            codekit_create::<$t>(code)            
        }
        }
    };
}


wasm_call!(EAN8);
wasm_call!(EAN13);
wasm_call!(Codabar);
wasm_call!(Code39);
wasm_call!(Code93);
wasm_call!(I2of5);