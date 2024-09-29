use std::vec;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{console, Event, HtmlElement, KeyboardEvent};

use crate::{levels::level::Level, physics::vec2::Vec2, GAME_STATE};

// Loads key press handler
pub fn load_key_handlers(element: &HtmlElement) -> Result<Vec<Closure<dyn Fn(Event)>>, JsValue> {
    let cb = Closure::wrap(Box::new(|e: Event| {
        let kb_event = e.dyn_into::<KeyboardEvent>().unwrap();
        let code = kb_event.code();
        GAME_STATE.with_borrow_mut(
            |Level {
                 ref mut entities, ..
             }| match code.as_str() {
                "KeyD" => {
                    let player = &mut entities[0];
                    let physics = player.physical_properties();
                    physics.velocity.set_x(5.); // Snappier feeling
                    physics.acceleration_vector.set_x(5_00.);
                }
                "KeyA" => {
                    let player = &mut entities[0];
                    let physics = player.physical_properties();
                    physics.velocity.set_x(-5.);
                    physics.acceleration_vector.set_x(-5_00.);
                }
                _ => (),
            },
        );
    }) as Box<dyn Fn(Event)>);

    let cb2 = Closure::wrap(Box::new(|e: Event| {
        let kb_event = e.dyn_into::<KeyboardEvent>().unwrap();
        let code = kb_event.code();
        GAME_STATE.with_borrow_mut(
            |Level {
                 ref mut entities, ..
             }| match code.as_str() {
                "KeyD" => {
                    let player = &mut entities[0];
                    let physics = player.physical_properties();
                    physics.acceleration_vector.set_x(0.);
                }
                "KeyA" => {
                    let player = &mut entities[0];
                    let physics = player.physical_properties();
                    physics.acceleration_vector.set_x(0.);
                }
                _ => (),
            },
        );
    }) as Box<dyn Fn(Event)>);

    element.add_event_listener_with_callback("keydown", &cb.as_ref().unchecked_ref())?;
    element.add_event_listener_with_callback("keyup", &cb2.as_ref().unchecked_ref())?;
    console::log_1(&JsValue::from_str(&format!("load")));
    Ok(vec![cb, cb2])
}
