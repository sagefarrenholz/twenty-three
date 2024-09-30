use wasm_bindgen::JsValue;
use web_sys::console;

use crate::{
    foundation::{Entity, EntityPhysicalProperties},
    physics::collisions::{BoundingBox, CollisionParameters},
    v2, Vec2,
};

#[derive(Debug)]
pub struct LevelMetadata<'a> {
    pub(crate) name: &'a str,
}

pub struct Level<'a> {
    pub level_metadata: LevelMetadata<'a>,
    pub entities: Vec<Entity>,
    pub player_start: Vec2,
    pub(crate) next: Option<Box<dyn Fn() -> Level<'static>>>,
}

// Levels are 64x64

// Used for managing game level
impl<'a> Level<'a> {
    // Inserts an entity in this level
    pub fn insert_entity(&mut self) {}

    // Returns a hash of the current level
    pub fn get_level_hash(&self) {}

    pub fn advance(&self) -> Option<Level<'static>> {
        (self.next.as_ref()).map(|f| f())
    }

    pub fn reset(&mut self) {
        let player = &mut self.entities[0];
        player.physical_properties().position = self.player_start;
    }
}
