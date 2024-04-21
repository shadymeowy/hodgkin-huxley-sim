mod applet;
mod solver;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}
