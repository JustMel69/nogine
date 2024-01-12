use std::{ops::{Add, AddAssign, Sub, Neg, Mul, Div}, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2(pub f32, pub f32);

impl Vector2 {
    pub const ZERO: Self = Self(0.0, 0.0);
    pub const UP: Self = Self::up(1.0);
    pub const DOWN: Self = Self::down(1.0);
    pub const RIGHT: Self = Self::right(1.0);
    pub const LEFT: Self = Self::left(1.0);
    pub const ONE: Self = Self::one(1.0);

    pub const fn up(fact: f32) -> Self {
        return Self(0.0, fact);
    }

    pub const fn down(fact: f32) -> Self {
        return Self(0.0, -fact);
    }

    pub const fn right(fact: f32) -> Self {
        return Self(fact, 0.0);
    }

    pub const fn left(fact: f32) -> Self {
        return Self(-fact, 0.0);
    }

    pub const fn one(fact: f32) -> Self {
        return Self(fact, fact);
    }


    pub fn x(self) -> f32 {
        return self.0;
    }

    pub fn y(self) -> f32 {
        return self.1;
    }

    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    pub fn xvec(self) -> Self {
        return Self(self.0, 0.0);
    }

    pub fn yvec(self) -> Self {
        return Self(0.0, self.1);
    }

    pub fn cross(self) -> Self {
        return Self(self.1, -self.0);
    }

    /// Rotates the vector by the rotation provided in radians.
    pub fn rotate(self, rot: f32) -> Self {
        return Self(self.0 * rot.cos() + self.1 * rot.sin(), self.0 * rot.sin() - self.1 * rot.cos());
    }

    /// Performs the dot product between two vectors.
    pub fn dot(self, other: Self) -> f32 {
        return self.0 * other.0 + self.1 * other.1;
    }

    /// Returns the magnitude of the vector.
    pub fn magnitude(self) -> f32 {
        return (self.0 * self.0 + self.1 * self.1).sqrt();
    }

    /// Returns the vector with a magnitude of 1.
    pub fn normalized(self) -> Self {
        let mag = self.magnitude();
        return Self(self.0 / mag, self.1 / mag);
    }

    /// Returns the vector divided by the squared magnitude.
    pub fn inverted(self) -> Self {
        let sqr_mag = self.dot(self);
        return Self(self.0 / sqr_mag, self.1 / sqr_mag);
    }

    /// Multiplies the vectors component-wise
    pub fn scale(self, other: Self) -> Self {
        return Self(self.0 * other.0, self.1 * other.1);
    }

    /// Divides the vectors component-wise
    pub fn inv_scale(self, other: Self) -> Self {
        return Self(self.0 / other.0, self.1 / other.1);
    }

    /// Inverts each component
    pub fn inv_dims(self) -> Self {
        return Self(1.0 / self.0, 1.0 / self.1);
    }

    pub fn lerp(self, other: Self, fact: f32) -> Self {
        return super::lerp(self, other, fact);
    }

    pub fn average(slice: &[Self]) -> Self {
        let mut res = Vector2::ZERO;
        for v in slice {
            res += *v;
        }
        return res / slice.len() as f32;
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self(self.0 - rhs.0, self.1 - rhs.1);
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self(-self.0, -self.1);
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        return Self(self.0 * rhs, self.1 * rhs);
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        return Self(self.0 / rhs, self.1 / rhs);
    }
}

impl Into<(f32, f32)> for Vector2 {
    fn into(self) -> (f32, f32) {
        return (self.0, self.1);
    }
}

impl Into<Vector2> for (f32, f32) {
    fn into(self) -> Vector2 {
        return Vector2(self.0, self.1);
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}