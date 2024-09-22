pub mod foundation;

use std::sync::{LazyLock, RwLock};

use levels::level::Level;
pub use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

mod handlers;
mod levels;
mod physics;

#[derive(Default)]
struct Game {
    level_lock: RwLock<Level>, // Loaded level, if not set then present a menu
}

// Game state object
pub static GAME_STATE: LazyLock<Game> = LazyLock::new(|| Default::default());

// Perform a render step
#[wasm_bindgen]
pub fn render(canvas: HtmlCanvasElement, delta_time: i32) -> Result<(), JsValue> {
    let context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

    let mut entities = &mut GAME_STATE
        .level_lock
        .write()
        .map_err(|_| JsError::new("Failed to acquire write lock on game state"))?
        .entities;

    context.set_fill_style(&JsValue::from_str("#00"));
    context.fill_rect(0., 0., 100., 100.);

    console::log_1(&JsValue::from_str("test"));
    Ok(())
}

// Ensures all panics are reported in console
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Load all event handlers
    // load_key_handler();

    Ok(())
}
