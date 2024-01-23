use super::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Rect(pub f32, pub f32, pub f32, pub f32);

impl Rect {
    pub const IDENT: Self = Rect(0.0, 0.0, 1.0, 1.0);

    pub fn up(&self) -> f32 {
        return self.1 + self.3;
    }

    pub fn down(&self) -> f32 {
        return self.1;
    }

    pub fn right(&self) -> f32 {
        return self.0 + self.2;
    }

    pub fn left(&self) -> f32 {
        return self.0;
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

    pub fn pos(&self) -> Vector2 {
        return Vector2(self.0, self.1)
    }

    pub fn size(&self) -> Vector2 {
        return Vector2(self.2, self.3);
    }

    pub fn sample(&self, pos: Vector2) -> Vector2 {
        return pos.scale(self.size()) + self.pos();
    }

    pub fn contains(&self, pos: Vector2) -> bool {
        return
            pos.0 >= self.left() && pos.0 <= self.right() &&
            pos.1 >= self.down() && pos.1 <= self.up();
    }

    pub fn expand(self, fact: f32) -> Self {
        return Self(self.0 - fact, self.1 - fact, self.2 + fact * 2.0, self.3 + fact * 2.0);
    }
}