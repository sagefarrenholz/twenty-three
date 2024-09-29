use web_time::{SystemTime, UNIX_EPOCH};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{console, Event, HtmlElement, KeyboardEvent};

use crate::HELD_KEYS;

// Loads key press handler
pub fn load_key_handlers(element: &HtmlElement) -> Result<Vec<Closure<dyn Fn(Event)>>, JsValue> {
    let cb = Closure::wrap(Box::new(|e: Event| {
        let kb_event = e.dyn_into::<KeyboardEvent>().unwrap();
        let code = kb_event.code();
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
