use super::{Vector2, Rect};

/// A rect rotated around its center point.
pub struct RotRect {
    pub center: Vector2,
    pub extents: Vector2,
    pub rot: f32,
}

impl RotRect {
    pub fn contains(&self, point: Vector2) -> bool {
        // Move point to rect local space
        let point = (point - self.center).rotate(-self.rot) + self.center;

        // Calculate regular containing
        let half_ext = self.extents * 0.5;
        return
            point.0 > self.center.0 - half_ext.0 && point.0 < self.center.0 + half_ext.0 &&
            point.1 > self.center.1 - half_ext.1 && point.1 < self.center.1 + half_ext.1;
    }
}

impl From<Rect> for RotRect {
    fn from(value: Rect) -> Self {
        let center = Vector2(
            (value.left() + value.right()) * 0.5,
            (value.up() + value.down()) * 0.5,
        );

        return Self { center, extents: value.size(), rot: 0.0 };
    }
}