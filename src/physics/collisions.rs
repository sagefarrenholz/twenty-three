use crate::foundation::Position;

use super::vec2::Vec2;

// w & h used to determine bounding box
// x, y is top left of box
#[derive(Debug)]
pub enum BoundingBox {
    Square { w: f32, h: f32 },
}

// Enabled (this object will experience collision physics)
// Static (this object will impart but not experience physics itself)
// Passthrough (this object does not experience collision physics)
#[derive(Debug)]
pub enum CollisionParameters {
    Enabled(BoundingBox),
    Static,
    Passthrough,
}

// If true the two entities colliding
// returns the new position for a to resolve the collision
pub fn collision_test(
    a_col: &CollisionParameters,
    a_pos: &Position,
    b_col: &CollisionParameters,
    b_pos: &Position,
    iteration: usize, // In the rare case,
) -> Option<Vec2> {
    Some(Vec2::new((0., 0.)))
}
