use wasm_bindgen::prelude::*;

use crate::{commons::Barcode, Codabar, Code39, Code93, I2of5, EAN13, EAN8};
use std::f64;
use web_sys::console;

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

#[wasm_bindgen]
pub fn codekit_draw_ean8(canvas_id: &str, code: String, bar_width: u32, height: u32) -> Result<(), JsError> {

    // We first need to get the document canvas from the id
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = match document.get_element_by_id(canvas_id) {
        Some(canvas) => canvas,
        None => return Err(JsError::new(&format!("impossible to get the canvas with {}", canvas_id))),
    };
    // Once the node is here, we start the canvas API
    // See: https://rustwasm.github.io/docs/wasm-bindgen/examples/2d-canvas.html
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_e| JsError::new("can not find the canvas"))?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    
    // Compute the code
    let ean8_code = EAN8::make_descriptor(&code).map_err(|_e| JsError::new("invalid code"))?;
    
    // Now we need to ajust the canvas size to the computed size
    let required_width = (ean8_code.len() as u32) * bar_width;
    canvas.set_width(required_width);
    canvas.set_height(height);

    // Now we draw the code
    for (index, bar_text) in ean8_code.chars().enumerate() {
        console::log_2(&"index:".into(), &index.into()); 
         match bar_text {            
            '1' => {
                let index = index as u32;
                let idx = (index as f64) * (bar_width as f64);     
                console::log_2(&"x:".into(), &idx.into());       
                context.fill_rect(idx, 0.0, bar_width as f64, height as f64);
            },
            _ => {},
        };
    }
;
    Ok(())
}