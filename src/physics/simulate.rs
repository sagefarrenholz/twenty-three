use std::error::Error;

use super::{
    constants::{DRAG_CONSTANT, GRAVITY_ACCELERATION},
    vec2::Vec2,
};
use crate::foundation::{Entity, EntityPhysicalProperties};

// In place modifies entities using force vectors on
pub fn simulate(entities: &mut Vec<Entity>, dt: f32) -> Result<(), Box<dyn Error>> {
    // Stage 1, apply net force vector
    for entity in entities.iter_mut() {
        let EntityPhysicalProperties {
            force_vectors,
            mass,
            velocity,
            apply_gravity,
            acceleration_vector,
            max_velocity,
            ..
        } = entity.physical_properties();

        let mut new_velocity = velocity.clone();
        // TODO make these seperate for each component and handle y case
        if velocity.x() > max_velocity.x() {
            continue;
        }

        // console::log_1(&JsValue::from_str(&format!("{:?}", velocity)));

        let mut net_force: Vec2 = Default::default();
        net_force.x();

        // Net applied forces
        for force in force_vectors.drain(..) {
            net_force += force;
        }
        // Resolve new velocity vector
        // Where a = dv / dt, so dv = a dt

        // Drag force
        // net_force += new_velocity.clone() * -1. * DRAG_CONSTANT;

        new_velocity += acceleration_vector * dt;

        if *apply_gravity {
            new_velocity += Vec2::new((0., -1. * GRAVITY_ACCELERATION * dt));
        }
        // console::log_1(&JsValue::from_str(&format!(
        //     "net 3 {:?} dt {}",
        //     net_force, dt
        // )));

        new_velocity += net_force * dt * (1. / *mass);

        if new_velocity.x().abs() > max_velocity.x() {
            new_velocity.set_x(velocity.x());
        }

        *velocity = new_velocity;

        // console::log_1(&JsValue::from_str("he"));
    }

    // Stage 2, integrate
    for entity in entities.iter_mut() {
        let EntityPhysicalProperties {
            velocity, position, ..
        } = entity.physical_properties();
        // console::log_1(&JsValue::from_str(&format!(
        //     "{:?} {:?}",
        //     velocity, position
        // )));
        *position += velocity * dt;
        // console::log_1(&JsValue::from_str(&format!("updated pos {:?}", position)));
    }

    // Stage 3, collision fixing
    let player = &mut entities[0];
    // Bottom center of player
    if let Some((player, entities)) = entities.split_first_mut() {
        if let Entity::Player {
            health: player_health,
            physical_properties,
        } = player
        {
            for non_player_entity in entities {
                match non_player_entity {
                    Entity::Block {
                        physical_properties,
                        attack,
                    } => {
                        *player_health += *attack;
                    }
                    Entity::Enemy {
                        physical_properties,
                        attack,
                        health,
                    } => {
                        // if collision_test(a_col, a_pos, b_col, b_pos) {
                        //     *player_health += *attack;
                        // }
                    }
                    _ => panic!("Found unimplemented collision type"),
                }
            }
        } else {
            panic!("Player should be first entity")
        }
    }

    Ok(())
}
