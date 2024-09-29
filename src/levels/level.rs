use crate::{
    foundation::{Entity, EntityPhysicalProperties, Position},
    physics::{
        collisions::{BoundingBox, CollisionParameters},
        vec2::Vec2,
    },
};

#[derive(Debug)]
pub struct LevelMetadata {
    name: &'static str,
}

#[derive(Debug)]
pub struct Level {
    pub level_metadata: Option<LevelMetadata>,
    pub entities: Vec<Entity>,
}

impl Default for Level {
    fn default() -> Self {
        let player = Entity::Player {
            health: 100,
            physical_properties: EntityPhysicalProperties::new(CollisionParameters::Enabled(
                BoundingBox::Square { w: 1., h: 1. },
            )),
        };
        Self {
            entities: vec![player],
            level_metadata: Some(LevelMetadata { name: "TEST" }),
        }
    }
}

// Levels are 64x64

// Used for managing game level
impl Level {
    pub fn new() -> Self {
        Level::default()
    }

    pub fn from_hash(hash: &'static str) -> Self {
        Level::default()
    }

    // Inserts an entity in this level
    pub fn insert_entity(&mut self) {}

    // Returns a hash of the current level
    pub fn get_level_hash(&self) {}
}
