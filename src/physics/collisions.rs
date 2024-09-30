use core::f32;

use crate::v2;

use super::vec2::Vec2;

// w & h used to determine bounding box
// x, y is top left of box
#[derive(Debug)]
pub enum BoundingBox {
    Rect { w: f32, h: f32 },
}

// Enabled (this object will experience collision physics)
// Static (this object will impart but not experience physics itself)
// Passthrough (this object does not experience collision physics)
#[derive(Debug)]
pub enum CollisionParameters {
    Enabled(BoundingBox),
    // Static,
    Passthrough,
}

// If true the two entities colliding
// returns a position offset vector used to resolve collision
// the direction of this position offset vector can be used to nullify velocity as well
pub fn collision_test(
    a_col: &CollisionParameters,
    a_pos: &Vec2,
    b_col: &CollisionParameters,
    b_pos: &Vec2,
) -> Option<Vec2> {
    match (a_col, b_col) {
        (
            CollisionParameters::Enabled(BoundingBox::Rect { w: size1, .. }),
            CollisionParameters::Enabled(BoundingBox::Rect { w: size2, .. }),
        ) => {
            // Assume squares: use width as the size

            let half_size1 = size1 / 2.0;
            let half_size2 = size2 / 2.0;

            // Calculate centers
            let center1 = v2!(a_pos.x() + half_size1, a_pos.y() + half_size1);
            let center2 = v2!(b_pos.x() + half_size2, b_pos.y() + half_size2);

            // Calculate distance between centers
            let distance = center2 - center1;
            let abs_distance = distance.abs();

            // Check if there's an overlap
            let overlap_x = half_size1 + half_size2 - abs_distance.x();
            let overlap_y = half_size1 + half_size2 - abs_distance.y();

            if overlap_x > 0.0 && overlap_y > 0.0 {
                // There is a collision, determine the smaller overlap
                if overlap_x < overlap_y {
                    Some(v2!(overlap_x * distance.x().signum(), 0.0))
                } else {
                    Some(v2!(0.0, overlap_y * distance.y().signum()))
                }
            } else {
                None
            }
        }
        _ => None,
    }
}
