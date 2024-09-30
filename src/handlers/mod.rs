use web_time::{SystemTime, UNIX_EPOCH};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{console, Event, HtmlElement, KeyboardEvent};

use crate::{v2, GAME_STATE, HELD_KEYS};

// Loads key press handler
pub fn load_key_handlers(element: &HtmlElement) -> Result<Vec<Closure<dyn Fn(Event)>>, JsValue> {
    let cb = Closure::wrap(Box::new(|e: Event| {
        let kb_event = e.dyn_into::<KeyboardEvent>().unwrap();
        let code = kb_event.code();
        match code.as_str() {
            "Space" => GAME_STATE.with_borrow_mut(|s| {
                console::log_1(&JsValue::from_str(&format!("space",)));
                let player = &mut s.entities[0];
                let player_physics = player.physical_properties();
                console::log_1(&JsValue::from_str(&format!("test {:?}", player_physics)));
                if player_physics.grounded && !player_physics.jumping {
                    player_physics.velocity.set_y(8.);
                    player_physics.jumping = true;
                }
            }),
            _ => {}
        }
        HELD_KEYS.with_borrow_mut(|hk| {
            hk.insert(
                code,
                (
                    true,
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                ),
            );
        });
    }) as Box<dyn Fn(Event)>);

    let cb2 = Closure::wrap(Box::new(|e: Event| {
        let kb_event = e.dyn_into::<KeyboardEvent>().unwrap();
        let code = kb_event.code();

        HELD_KEYS.with_borrow_mut(|hk| {
            hk.insert(code, (false, 0));
        });
    }) as Box<dyn Fn(Event)>);

    element.add_event_listener_with_callback("keydown", &cb.as_ref().unchecked_ref())?;
    element.add_event_listener_with_callback("keyup", &cb2.as_ref().unchecked_ref())?;
    console::log_1(&JsValue::from_str(&format!("load")));
    Ok(vec![cb, cb2])
}
