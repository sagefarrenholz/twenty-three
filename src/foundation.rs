use super::physics::vec2::Vec2;
use crate::{physics::collisions::CollisionParameters, v2};

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct EntityPhysicalProperties {
    pub mass: f32,
    pub position: Vec2,
    pub collision_parameter: CollisionParameters,
    pub force_vectors: Vec<Vec2>, // Force vectors to apply on next physics time step
    pub acceleration_vector: Vec2, // Constant acceleration to apply
    pub apply_gravity: bool,
    pub velocity: Vec2,
    pub max_velocity: Vec2, // -1 = uncapped
}

impl EntityPhysicalProperties {
    pub fn new(collision_parameter: CollisionParameters) -> Self {
        Self {
            position: Default::default(),
            collision_parameter,
            apply_gravity: true,
            mass: 80f32, // kg
            velocity: Default::default(),
            force_vectors: Vec::new(),
            acceleration_vector: Vec2::default(),
            max_velocity: v2!(20., -1.),
        }
    }
}

#[derive(Debug)]
pub enum Entity {
    Player {
        health: i32,
        physical_properties: EntityPhysicalProperties,
    },
    Block {
        attack: i32, // Damage dealt when player touches block
        physical_properties: EntityPhysicalProperties,
    },
    Enemy {
        health: i32,
        attack: i32, // Damage dealt when player touches enemy
        physical_properties: EntityPhysicalProperties,
    },
}

impl Entity {
    pub fn physical_properties(&mut self) -> &mut EntityPhysicalProperties {
        match self {
            Entity::Player {
                physical_properties,
                ..
            } => physical_properties,
            Entity::Enemy {
                physical_properties,
                ..
            } => physical_properties,
            Entity::Block {
                physical_properties,
                ..
            } => physical_properties,
        }
    }
}
