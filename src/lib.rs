pub mod foundation;

use std::cell::{Cell, RefCell};

use foundation::{Entity, EntityPhysicalProperties};
use handlers::load_key_handlers;
use levels::level::Level;
use physics::{
    collisions::{BoundingBox, CollisionParameters},
    simulate::simulate,
};
pub use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use web_sys::{console, CanvasRenderingContext2d, Event, HtmlCanvasElement};

mod handlers;
mod levels;
pub mod physics;

thread_local! {
    pub static GAME_STATE: RefCell<Level> = RefCell::new(Level::default());
    pub static EVENT_HANDLERS: RefCell<Vec<Closure<dyn Fn(Event)>>> = RefCell::new(Vec::default());
    pub static MILESTONE: Cell<bool> = Cell::new(false);
}

// Game state object

// How much to scale coordinates and dimensions by
// We assume the level is ~32m tall by 32m wide
// So, if the canvas is 2048px tall, then 2048px = 32m, then our scaling factor is 64
// and person at 2m is 128px tall
pub const GAME_HEIGHT: f32 = 32.;

// Perform a render step
#[wasm_bindgen]
pub fn render(canvas: HtmlCanvasElement, dt_ms: f32, tt_ms: f32) -> Result<(), JsValue> {
    let context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

    GAME_STATE.with_borrow_mut(
        move |Level {
                  ref mut entities, ..
              }| {
            context.set_fill_style(&JsValue::from_str("#00"));
            simulate(entities, dt_ms / 1000.).map_err(|e| JsError::new(e.to_string().as_str()))?;

            // Rendering
            // The canvas is square
            // t
            let player = &entities[0];
            let scaling_factor: f32 = canvas.height() as f32 / GAME_HEIGHT;
            context.clear_rect(0., 0., canvas.width().into(), canvas.height().into());
            if let Entity::Player {
                physical_properties:
                    EntityPhysicalProperties {
                        position,
                        collision_parameter,
                        ..
                    },
                ..
            } = player
            {
                if let CollisionParameters::Enabled(BoundingBox::Square { w, h }) =
                    collision_parameter
                {
                    context.fill_rect(
                        (scaling_factor * position.x()).into(),
                        (-1. * scaling_factor * position.y()).into(),
                        (*w * scaling_factor).into(),
                        (*h * scaling_factor).into(),
                    );

                    if position.y() < -32. && !MILESTONE.get() {
                        MILESTONE.set(true);
                        console::log_1(&JsValue::from_str(&format!(
                            "Hit 32m at time {}",
                            tt_ms / 1000.
                        )));
                    }
                }
            } else {
                return Err(JsError::new("No player found at index 0").into());
            }
            Ok(())
        },
    )
}

// Loads all event handlers
#[wasm_bindgen]
pub fn load_handlers(element: HtmlElement) -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str(&format!("Loading keys")));
    for key_handler in load_key_handlers(&element)?.drain(..) {
        EVENT_HANDLERS.with(move |event_handlers| event_handlers.borrow_mut().push(key_handler));
    }
    console::log_1(&JsValue::from_str(&format!("Done")));
    Ok(())
}

// Ensures all panics are reported in console
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
