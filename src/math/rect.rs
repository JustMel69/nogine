use super::Vector2;

// Instead of setting pos and size, we use start and end to avoid floating-point fuckeries, specially regarding uvs.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub start: Vector2,
    pub end: Vector2,
}

impl Rect {
    pub const IDENT: Self = Self { start: Vector2::ZERO, end: Vector2::ONE };

    pub fn up(&self) -> f32 {
        return self.end.1;
    }

    pub fn down(&self) -> f32 {
        return self.start.1;
    }

    pub fn right(&self) -> f32 {
        return self.end.0;
    }

    pub fn left(&self) -> f32 {
        return self.start.0;
    }

    pub fn lu(&self) -> Vector2 {
        return Vector2(self.left(), self.up());
    }

    pub fn ru(&self) -> Vector2 {
        return Vector2(self.right(), self.up());
    }

    pub fn ld(&self) -> Vector2 {
        return Vector2(self.left(), self.down());
    }

    pub fn rd(&self) -> Vector2 {
        return Vector2(self.right(), self.down());
    }

    pub fn center(&self) -> Vector2 {
        return (self.start + self.end) / 2.0;
    }

    pub fn size(&self) -> Vector2 {
        return self.end - self.start;
    }

    pub fn sample(&self, pos: Vector2) -> Vector2 {
        return self.start.scale(Vector2::ONE - pos) + self.end.scale(pos);
    }

    pub fn contains(&self, pos: Vector2) -> bool {
        return
            pos.0 >= self.left() && pos.0 <= self.right() &&
            pos.1 >= self.down() && pos.1 <= self.up();
    }

    pub fn expand(self, fact: f32) -> Self {
        return Self { start: self.start - Vector2::one(fact), end: self.end + Vector2::one(fact) };
    }
}


#[derive(Debug, Clone, Copy)]
pub struct URect(pub u32, pub u32, pub u32, pub u32);

impl URect {
    pub const IDENT: Self = Self(0, 0, 1, 1);

    pub fn up(&self) -> u32 {
        return self.1 + self.3;
    }

    pub fn down(&self) -> u32 {
        return self.1;
    }

    pub fn right(&self) -> u32 {
        return self.0 + self.2;
    }

    pub fn left(&self) -> u32 {
        return self.0;
    }
}