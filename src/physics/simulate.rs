use std::error::Error;

use super::{
    constants::{DRAG_CONSTANT, GRAVITY_ACCELERATION},
    vec2::Vec2,
};
use crate::foundation::{Entity, EntityPhysicalProperties};

// In place modifies entities using force vectors on
pub fn simulate(entities: &mut Vec<Entity>, dt: f32) -> Result<(), Box<dyn Error>> {
    // Stage 1, calculate net force vector
    for entity in entities.iter_mut() {
        let EntityPhysicalProperties {
            force_vectors,
            mass,
            velocity,
            apply_gravity,
            max_velocity,
            ..
        } = entity.physical_properties();

        // TODO make these seperate for each component and handle y case
        if velocity.x() > max_velocity.x() {
            continue;
        }

        let mut net_force: Vec2 = Default::default();
        net_force.x();

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

        *velocity = new_velocity;
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
            ..
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
