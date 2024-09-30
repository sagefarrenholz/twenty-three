use core::net;
use std::error::Error;

use wasm_bindgen::JsValue;
use web_sys::console;

use super::{
    collisions::collision_test,
    constants::{DRAG_CONSTANT, FRICTION_CONSTANT, GRAVITY_ACCELERATION},
    vec2::Vec2,
};
use crate::foundation::{Entity, EntityPhysicalProperties};

// In place modifies entities using force vectors on
pub fn simulate(entities: &mut Vec<Entity>, dt: f32) -> Result<Option<bool>, Box<dyn Error>> {
    // Stage 1, calculate net force vector
    for entity in entities.iter_mut() {
        let EntityPhysicalProperties {
            force_vectors,
            mass,
            velocity,
            apply_gravity,
            max_velocity,
            grounded,
            ..
        } = entity.physical_properties();

        // // TODO make these seperate for each component and handle y case
        // if velocity.x() > max_velocity.x() {
        //     continue;
        // }

        // console::log_1(&JsValue::from_str(&format!("{}", mass)));

        let mut net_force: Vec2 = Default::default();

        // Net applied forces
        for force in force_vectors.drain(..) {
            net_force += force;
        }

        // Force due to drag
        // F_d_vec = Drag_cof *  v^2 * -norm(v_vec)
        net_force += velocity.clone().norm() * -1. * DRAG_CONSTANT * velocity.mag().powi(2);

        // Force due to gravity
        // F_g = gm
        if *apply_gravity {
            net_force += Vec2::new_y(-1. * GRAVITY_ACCELERATION) * *mass;
        }

        // Force due to friction
        // If we're grounded and we have downward net force
        // apply a force counter to our x direction porportion to our y force
        if *grounded && net_force.y() < 0. {
            if velocity.x().abs() < 0.01 {
                // braking
                // velocity.set_x(0.);d
            } else {
                net_force += Vec2::new_x(net_force.y() * FRICTION_CONSTANT * velocity.x().signum());
            }
            console::log_1(&JsValue::from_str(&format!("{} {:?}", grounded, net_force)));
        }
        let mut new_velocity = velocity.clone();
        new_velocity += net_force * dt * (1. / *mass);

        // Max speed checks
        // Constrains the velocity further than drag will
        if new_velocity.x().abs() > max_velocity.x() {
            new_velocity.set_x(
                max_velocity.x()
                    * (if new_velocity.x().is_sign_negative() {
                        -1.
                    } else {
                        1.
                    }),
            );
        }

        // Deadzone velocity to prevent very small values
        *velocity = new_velocity.deadzone_x(0.01);
    }

    // Stage 2, integrate velocity
    for entity in entities.iter_mut() {
        let EntityPhysicalProperties {
            velocity, position, ..
        } = entity.physical_properties();

        *position += velocity * dt;
    }

    // Stage 3, collision fixing
    let player = &mut entities[0];
    // Bottom center of player

    if let Some((player, entities)) = entities.split_first_mut() {
        if let Entity::Player {
            health: player_health,
            physical_properties:
                EntityPhysicalProperties {
                    position: player_pos,
                    collision_parameter: player_col,
                    grounded,
                    velocity: player_velocity,
                    ground_velocity,
                    jumping,
                    ..
                },
            ..
        } = player
        {
            *grounded = false;
            for non_player_entity in entities {
                match non_player_entity {
                    Entity::Block {
                        physical_properties:
                            EntityPhysicalProperties {
                                collision_parameter: block_col,
                                position: block_pos,
                                velocity: block_velocity,
                                ..
                            },
                        ..
                    } => {
                        // *player_health += *attack;
                        if let Some(collision_offset) =
                            collision_test(player_col, player_pos, block_col, block_pos)
                        {
                            // console::log_1(&JsValue::from_str(&format!(
                            //     "player pos {:?} player col {:?} block pos {:?} block col {:?} coll {:?} grounded {}",
                            //     player_pos, player_col, block_pos, block_col, collision_offset, grounded
                            // )));
                            console::log_1(&JsValue::from_str(&format!("test {:?}", grounded)));
                            *player_pos += collision_offset;
                            if collision_offset.x() != 0. {
                                player_velocity.set_x(0.);
                            } else {
                                player_velocity.set_y(0.);
                                // Move with block
                                if block_velocity.x().abs() > player_velocity.x().abs() - 1. {
                                    player_velocity.set_x(block_velocity.x());
                                }
                                if block_velocity.y().abs() > player_velocity.y().abs() - 1. {
                                    player_velocity.set_y(block_velocity.y());
                                }
                                *grounded = true;
                                *jumping = false;
                            }
                        }
                    }
                    Entity::Enemy {
                        physical_properties:
                            EntityPhysicalProperties {
                                collision_parameter: block_col,
                                position: block_pos,
                                ..
                            },
                        ..
                    } => {
                        if let Some(_) =
                            collision_test(player_col, player_pos, block_col, block_pos)
                        {
                            return Ok(Some(false));
                        }
                    }
                    Entity::Goal {
                        physical_properties:
                            EntityPhysicalProperties {
                                collision_parameter: block_col,
                                position: block_pos,
                                ..
                            },
                        ..
                    } => {
                        if let Some(_) =
                            collision_test(player_col, player_pos, block_col, block_pos)
                        {
                            return Ok(Some(true));
                        }
                    }
                    Entity::Player {
                        health,
                        physical_properties,
                    } => todo!(),
                }
            }
        } else {
            panic!("Player should be first entity")
        }
    }

    Ok(None)
}
