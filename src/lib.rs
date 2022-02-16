mod app;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start(
    canvas_id: &str,
    width: u32,
    height: u32,
) -> Result<(), eframe::wasm_bindgen::JsValue> {
    utils::set_panic_hook();
    eframe::start_web(canvas_id, Box::new(app::App::new(width, height)))
}
