use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{window, HtmlCanvasElement};

use crate::GAME_STATE;

// Loads key press handler
#[wasm_bindgen]
pub fn load_key_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let move_character = Box::new(|| {});
    // canvas.add_event_listener_with_callback("key", Closure::wrap().as_ref().unchecked_ref());
    Ok(())
}
