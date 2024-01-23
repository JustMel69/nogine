use std::ops::Mul;

use crate::assert_expr;

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



#[derive(Clone, Copy, Debug)]
pub struct BColor4(pub u8, pub u8, pub u8, pub u8);

impl BColor4 {
    pub const CLEAR: Self = BColor4(0, 0, 0, 0);

    pub fn r(self) -> u8 {
        return self.0;
    }

    pub fn g(self) -> u8 {
        return self.1;
    }

    pub fn b(self) -> u8 {
        return self.2;
    }

    pub fn a(self) -> u8 {
        return self.3;
    }
}

impl Color for BColor4 {
    const RED: Self = BColor4(255, 0, 0, 255);
    const ORANGE: Self = BColor4(255, 128, 0, 255);
    const YELLOW: Self = BColor4(255, 255, 0, 255);
    const LIME: Self = BColor4(128, 255, 0, 255);
    const GREEN: Self = BColor4(0, 255, 0, 255);
    const CYAN: Self = BColor4(0, 255, 255, 255);
    const BLUE: Self = BColor4(0, 0, 255, 255);
    const PURPLE: Self = BColor4(128, 0, 255, 255);
    const PINK: Self = BColor4(255, 0, 255, 255);
    
    const WHITE: Self = BColor4(255, 255, 255, 255);
    const LIGHT_GRAY: Self = BColor4(191, 191, 191, 255);
    const GRAY: Self = BColor4(128, 128, 128, 255);
    const DARK_GRAY: Self = BColor4(64, 64, 64, 255);
    const BLACK: Self = BColor4(0, 0, 0, 255);
}



impl From<Color4> for BColor4 {
    fn from(value: Color4) -> Self {
        assert_expr!(
            value.0 >= 0.0 && value.0 <= 1.0 &&
            value.1 >= 0.0 && value.1 <= 1.0 &&
            value.2 >= 0.0 && value.2 <= 1.0 &&
            value.3 >= 0.0 && value.3 <= 1.0,
            "A color must be in the 0 to 1 range to be converted to a BColor4"
        );
        
        return Self(
            (value.0 * 255.0) as u8,
            (value.1 * 255.0) as u8,
            (value.2 * 255.0) as u8,
            (value.3 * 255.0) as u8,
        )
    }
}

impl From<BColor4> for Color4 {
    fn from(value: BColor4) -> Self {
        return Self(
            value.0 as f32 / 255.0,
            value.1 as f32 / 255.0,
            value.2 as f32 / 255.0,
            value.3 as f32 / 255.0,
        );
    }
}