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
    pub jumping: bool,
    pub collision_parameter: CollisionParameters,
    pub force_vectors: Vec<Vec2>, // Force vectors to apply on next physics time step
    pub apply_gravity: bool,
    pub ground_velocity: Option<Vec2>, // Additional Velocity to add
    pub grounded: bool,                //If base of this entity is colliding with another
    pub velocity: Vec2,
    pub start_position: Vec2,
    pub max_velocity: Vec2, // -1 = uncapped
}

impl EntityPhysicalProperties {
    pub fn new(position: Vec2, collision_parameter: CollisionParameters) -> Self {
        Self {
            position,
            collision_parameter,
            apply_gravity: true,
            start_position: position,
            mass: 70f32, // kg
            jumping: false,
            grounded: false,
            ground_velocity: None,
            velocity: Default::default(),
            force_vectors: Vec::new(),
            max_velocity: v2!(10., -1.),
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
    Goal {
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
            Entity::Goal {
                physical_properties,
            } => physical_properties,
        }
    }

    pub fn default_goal(position: Vec2) -> Entity {
        Entity::Goal {
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 2., h: 2. },
                ),
                apply_gravity: false,
                mass: 700., // kg
                ground_velocity: None,
                start_position: position,
                jumping: false,
                velocity: Default::default(),
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }

    pub fn small_block(position: Vec2) -> Entity {
        Entity::Block {
            attack: 0,
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 1., h: 1. },
                ),
                apply_gravity: false,
                ground_velocity: None,
                start_position: position,
                mass: 700., // kg
                jumping: false,
                velocity: Default::default(),
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }

    pub fn small_enemy(position: Vec2) -> Entity {
        Entity::Enemy {
            health: 0,
            attack: 0,
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 1., h: 1. },
                ),
                apply_gravity: false,
                mass: 700., // kg
                ground_velocity: None,
                start_position: position,
                jumping: false,
                velocity: Default::default(),
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }
    pub fn default_enemy(position: Vec2) -> Entity {
        Entity::Enemy {
            health: 0,
            attack: 0,
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 2., h: 2. },
                ),
                apply_gravity: false,
                ground_velocity: None,
                start_position: position,
                mass: 700., // kg
                jumping: false,
                velocity: Default::default(),
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }

    pub fn default_block(position: Vec2) -> Entity {
        Entity::Block {
            attack: 0,
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 2., h: 2. },
                ),
                apply_gravity: false,
                mass: 700., // kg
                jumping: false,
                start_position: position,
                ground_velocity: None,
                velocity: Default::default(),
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }
    pub fn default_goal_moving(position: Vec2, velocity: Vec2) -> Entity {
        Entity::Goal {
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 2., h: 2. },
                ),
                apply_gravity: false,
                mass: 700., // kg
                ground_velocity: None,
                start_position: position,
                jumping: false,
                velocity,
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }

    pub fn default_block_moving(position: Vec2, velocity: Vec2) -> Entity {
        Entity::Block {
            attack: 0,
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 2., h: 2. },
                ),
                apply_gravity: false,
                mass: 700., // kg
                ground_velocity: None,
                start_position: position,
                jumping: false,
                velocity,
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }

    pub fn default_enemy_moving(position: Vec2, velocity: Vec2) -> Entity {
        Entity::Enemy {
            health: 0,
            attack: 0,
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w: 2., h: 2. },
                ),
                apply_gravity: false,
                mass: 700., // kg
                ground_velocity: None,
                start_position: position,
                jumping: false,
                velocity,
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }

    pub fn custom_block(position: Vec2, w: f32, h: f32) -> Entity {
        Entity::Block {
            attack: 0,
            physical_properties: EntityPhysicalProperties {
                position,
                collision_parameter: CollisionParameters::Enabled(
                    crate::physics::collisions::BoundingBox::Rect { w, h },
                ),
                apply_gravity: false,
                mass: 700., // kg
                ground_velocity: None,
                start_position: position,
                jumping: false,
                velocity: Default::default(),
                force_vectors: Vec::new(),
                max_velocity: v2!(20., -1.),
                grounded: false,
            },
        }
    }
}
