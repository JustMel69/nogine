use std::ops::Mul;

pub trait Color {
    const RED: Self;
    const ORANGE: Self;
    const YELLOW: Self;
    const LIME: Self;
    const GREEN: Self;
    const CYAN: Self;
    const BLUE: Self;
    const PURPLE: Self;
    const PINK: Self;
    
    const WHITE: Self;
    const LIGHT_GRAY: Self;
    const GRAY: Self;
    const DARK_GRAY: Self;
    const BLACK: Self;
}


#[derive(Clone, Copy, Debug)]
pub struct Color4(pub f32, pub f32, pub f32, pub f32);

impl Color4 {
    pub const CLEAR: Self = Color4(0.0, 0.0, 0.0, 0.0);

    pub fn r(self) -> f32 {
        return self.0;
    }

    pub fn g(self) -> f32 {
        return self.1;
    }

    pub fn b(self) -> f32 {
        return self.2;
    }

    pub fn a(self) -> f32 {
        return self.3;
    }

    pub const fn mix(self, other: Self, fact: f32) -> Self {
        return Color4(
            other.0 * fact + self.0 * (1.0 - fact),
            other.1 * fact + self.1 * (1.0 - fact),
            other.2 * fact + self.2 * (1.0 - fact),
            other.3 * fact + self.3 * (1.0 - fact),
        );
    }
}

impl Color for Color4 {
    const RED: Self = Color4(1.0, 0.0, 0.0, 1.0);
    const ORANGE: Self = Color4(1.0, 0.5, 0.0, 1.0);
    const YELLOW: Self = Color4(1.0, 1.0, 0.0, 1.0);
    const LIME: Self = Color4(0.5, 1.0, 0.0, 1.0);
    const GREEN: Self = Color4(0.0, 1.0, 0.0, 1.0);
    const CYAN: Self = Color4(0.0, 1.0, 1.0, 1.0);
    const BLUE: Self = Color4(0.0, 0.0, 1.0, 1.0);
    const PURPLE: Self = Color4(0.5, 0.0, 1.0, 1.0);
    const PINK: Self = Color4(1.0, 0.0, 1.0, 1.0);
    
    const WHITE: Self = Color4(1.0, 1.0, 1.0, 1.0);
    const LIGHT_GRAY: Self = Color4(0.75, 0.75, 0.75, 1.0);
    const GRAY: Self = Color4(0.5, 0.5, 0.5, 1.0);
    const DARK_GRAY: Self = Color4(0.25, 0.25, 0.25, 1.0);
    const BLACK: Self = Color4(0.0, 0.0, 0.0, 1.0);
}

impl Mul for Color4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2, self.3 * rhs.3);
    }
}