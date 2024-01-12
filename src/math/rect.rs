use super::Vector2;

#[derive(Clone, Copy)]
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
}