use crate::foundation::Position;

// w & h used to determine bounding box
// x, y is top left of box
#[derive(Debug)]
pub enum BoundingBox {
    Square { w: i32, h: i32 },
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

// If true the two entities
pub fn collision_test(
    a_col: &CollisionParameters,
    a_pos: &Position,
    b_col: &CollisionParameters,
    b_pos: &Position,
) -> bool {
    false
}
