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
    

    pub fn ld(&self) -> Vector2 {
        return (-self.extents * 0.5).rotate(self.rot) + self.center;
    }

    pub fn rd(&self) -> Vector2 {
        return Vector2(self.extents.0 * 0.5, -self.extents.1 * 0.5).rotate(self.rot) + self.center;
    }

    pub fn lu(&self) -> Vector2 {
        return Vector2(-self.extents.0 * 0.5, self.extents.1 * 0.5).rotate(self.rot) + self.center;
    }

    pub fn ru(&self) -> Vector2 {
        return (self.extents * 0.5).rotate(self.rot) + self.center;
    }


    pub fn left(&self) -> Vector2 {
        return (-self.extents.xvec() * 0.5).rotate(self.rot) + self.center;
    }

    pub fn right(&self) -> Vector2 {
        return (self.extents.xvec() * 0.5).rotate(self.rot) + self.center;
    }

    pub fn up(&self) -> Vector2 {
        return (self.extents.yvec() * 0.5).rotate(self.rot) + self.center;
    }

    pub fn down(&self) -> Vector2 {
        return (-self.extents.yvec() * 0.5).rotate(self.rot) + self.center;
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