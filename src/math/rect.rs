use super::vec2;

// Instead of setting pos and size, we use start and end to avoid floating-point fuckeries, specially regarding uvs.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub start: vec2,
    pub end: vec2,
}

impl Rect {
    pub const IDENT: Self = Self { start: vec2::ZERO, end: vec2::ONE };

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

    pub fn lu(&self) -> vec2 {
        return vec2(self.left(), self.up());
    }

    pub fn ru(&self) -> vec2 {
        return vec2(self.right(), self.up());
    }

    pub fn ld(&self) -> vec2 {
        return vec2(self.left(), self.down());
    }

    pub fn rd(&self) -> vec2 {
        return vec2(self.right(), self.down());
    }

    pub fn center(&self) -> vec2 {
        return (self.start + self.end) / 2.0;
    }

    pub fn size(&self) -> vec2 {
        return self.end - self.start;
    }

    pub fn sample(&self, pos: vec2) -> vec2 {
        return self.start.scale(vec2::ONE - pos) + self.end.scale(pos);
    }

    pub fn contains(&self, pos: vec2) -> bool {
        return
            pos.0 >= self.left() && pos.0 <= self.right() &&
            pos.1 >= self.down() && pos.1 <= self.up();
    }

    pub fn expand(self, fact: f32) -> Self {
        return Self { start: self.start - vec2::one(fact), end: self.end + vec2::one(fact) };
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