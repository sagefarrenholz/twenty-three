use crate::physics::collisions::CollisionParameters;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct EntityPhysicalProperties {
    position: Position,
    collision_parameter: CollisionParameters,
}

impl EntityPhysicalProperties {
    pub fn new(position: Position, collision_parameter: CollisionParameters) -> Self {
        Self {
            position,
            collision_parameter,
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
        physical_properties: EntityPhysicalProperties,
    },
}
