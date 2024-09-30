pub mod foundation;

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use foundation::{Entity, EntityPhysicalProperties};
use handlers::load_key_handlers;
use levels::{level::Level, levels::level_1};
use physics::{
    collisions::{BoundingBox, CollisionParameters},
    simulate::simulate,
    vec2::Vec2,
};
pub use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use web_sys::{console, CanvasRenderingContext2d, Event, HtmlCanvasElement};

mod handlers;
mod levels;
pub mod physics;

thread_local! {
    pub static GAME_STATE: RefCell<Level<'static>> = RefCell::new(level_1());
    pub static EVENT_HANDLERS: RefCell<Vec<Closure<dyn Fn(Event)>>> = RefCell::new(Vec::default());
    pub static HELD_KEYS: RefCell<HashMap<String, (bool, u128)>> = RefCell::new(HashMap::new());
    pub static MILESTONE: Cell<bool> = Cell::new(false);
}

// Game state object

// How much to scale coordinates and dimensions by
// We assume the level is ~32m tall by 32m wide
// So, if the canvas is 2048px tall, then 2048px = 32m, then our scaling factor is 64
// and person at 2m is 128px tall
pub const GAME_HEIGHT: f32 = 32.;

// How much force to apply on x force input
pub const PLAYER_X_FORCE: f32 = 10_000.;

#[wasm_bindgen]
pub fn render(canvas: HtmlCanvasElement, dt_ms: f32, tt_ms: f32) -> Result<bool, JsValue> {
    let mut context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

    GAME_STATE.with_borrow_mut(move |level| {
        let goal_reached = {
            let entities = &mut level.entities;
            simulate(entities, dt_ms / 1000.).map_err(|e| JsError::new(e.to_string().as_str()))?
        };

        let level_name = level.level_metadata.name.to_owned();

        if let Some(goal_reached) = goal_reached {
            if goal_reached {
                if let Some(next_level) = level.advance() {
                    *level = next_level
                } else {
                    // Game win
                    return Ok(true);
                }
            } else {
                level.reset();
            }
        }

        {
            let entities = &mut level.entities;

            let player = &mut entities[0];

            // if player has fallen below level
            if player.physical_properties().position.y() < -34. {
                level.reset();
            }
        }

        let entities = &mut level.entities;

        let player = &mut entities[0];

        // Continuous input handling (held keys)
        HELD_KEYS.with_borrow_mut(|h| {
            // X movement
            let (a_held, a_start) = *h.entry("KeyA".to_string()).or_insert((false, 0)); // left direction held
            let (d_held, d_start) = *h.entry("KeyD".to_string()).or_insert((false, 0)); // right direction held

            {
                let forces = &mut player.physical_properties().force_vectors;
                // If both held we only accelerate in direction of last pressed
                if a_held && d_held {
                    if a_start > d_start {
                        forces.push(Vec2::new_x(PLAYER_X_FORCE * -1.));
                    } else {
                        forces.push(Vec2::new_x(PLAYER_X_FORCE));
                    }
                } else if a_held {
                    forces.push(Vec2::new_x(PLAYER_X_FORCE * -1.));
                } else if d_held {
                    forces.push(Vec2::new_x(PLAYER_X_FORCE));
                }
            }
        });

        // Rendering
        // The canvas is square
        let scaling_factor: f32 = canvas.height() as f32 / GAME_HEIGHT;
        context.clear_rect(0., 0., canvas.width().into(), canvas.height().into());

        for entity in entities {
            let fill_color = match entity {
                Entity::Player { .. } => "#0000ff",
                Entity::Goal { .. } => "#00ff00",
                Entity::Block { .. } => "#000000",
                Entity::Enemy { .. } => "#ff0000",
            };
            // console::log_1(&JsValue::from_str(&format!("renderingdd {:?}", entity)));
            let EntityPhysicalProperties {
                collision_parameter,
                position,
                ..
            } = entity.physical_properties();

            context.set_fill_style(&JsValue::from_str(fill_color));
            if let CollisionParameters::Enabled(BoundingBox::Rect { w, h }) = collision_parameter {
                context.fill_rect(
                    (scaling_factor * position.x()).into(),
                    (-1. * scaling_factor * (position.y() + *h)).into(),
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
        }

        // Render level number
        context.set_fill_style(&JsValue::from_str("#00"));
        context.set_font("50px");
        context.fill_text_with_max_width(&level_name, (scaling_factor * 30.).into(), 10., 2000.)?;

        Ok((false))
    })
}

// Perform a render step
#[wasm_bindgen]
pub fn render_editor(canvas: HtmlCanvasElement, dt_ms: f32, tt_ms: f32) -> Result<(), JsValue> {
    let context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

    GAME_STATE.with_borrow_mut(
        move |Level {
                  ref mut entities, ..
              }| {
            simulate(entities, dt_ms / 1000.).map_err(|e| JsError::new(e.to_string().as_str()))?;

            let player = &mut entities[0];

            // Continuous input handling (held keys)
            HELD_KEYS.with_borrow_mut(|h| {
                // X movement
                let (a_held, a_start) = *h.entry("KeyA".to_string()).or_insert((false, 0)); // left direction held
                let (d_held, d_start) = *h.entry("KeyD".to_string()).or_insert((false, 0)); // right direction held

                {
                    let forces = &mut player.physical_properties().force_vectors;
                    // If both held we only accelerate in direction of last pressed
                    if a_held && d_held {
                        if a_start > d_start {
                            forces.push(Vec2::new_x(PLAYER_X_FORCE * -1.));
                        } else {
                            forces.push(Vec2::new_x(PLAYER_X_FORCE));
                        }
                    } else if a_held {
                        forces.push(Vec2::new_x(PLAYER_X_FORCE * -1.));
                    } else if d_held {
                        forces.push(Vec2::new_x(PLAYER_X_FORCE));
                    }
                }
            });

            // Rendering
            // The canvas is square
            let scaling_factor: f32 = canvas.height() as f32 / GAME_HEIGHT;
            context.clear_rect(0., 0., canvas.width().into(), canvas.height().into());

            // Simple player rendering code
            // if let Entity::Player {
            //     physical_properties:
            //         EntityPhysicalProperties {
            //             position,
            //             collision_parameter,
            //             ..
            //         },
            //     ..
            // } = player
            for entity in entities {
                let fill_color = match entity {
                    Entity::Player { .. } => "#0000ff",
                    Entity::Goal { .. } => "#00ff00",
                    Entity::Block { .. } => "#000000",
                    Entity::Enemy { .. } => "#ff0000",
                };
                // console::log_1(&JsValue::from_str(&format!("renderingdd {:?}", entity)));
                let EntityPhysicalProperties {
                    collision_parameter,
                    position,
                    ..
                } = entity.physical_properties();

                context.set_fill_style(&JsValue::from_str(fill_color));
                if let CollisionParameters::Enabled(BoundingBox::Rect { w, h }) =
                    collision_parameter
                {
                    context.fill_rect(
                        (scaling_factor * position.x()).into(),
                        (-1. * scaling_factor * (position.y() + *h)).into(),
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
