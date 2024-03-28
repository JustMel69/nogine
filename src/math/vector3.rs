use std::{fmt::Display, ops::{Add, AddAssign, Div, Mul, Neg, Sub}, simd::{f32x4, num::SimdFloat, StdFloat}};

use super::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3(pub f32, pub f32, pub f32);

impl Vector3 {
    pub const ZERO: Self = Self(0.0, 0.0, 0.0);
    pub const UP: Self = Self::up(1.0);
    pub const DOWN: Self = Self::down(1.0);
    pub const RIGHT: Self = Self::right(1.0);
    pub const LEFT: Self = Self::left(1.0);
    pub const ONE: Self = Self::one(1.0);
    pub const FORW: Self = Self::forw(1.0);
    pub const BACK: Self = Self::back(1.0);

    pub const fn up(fact: f32) -> Self {
        return Self(0.0, fact, 0.0);
    }

    pub const fn down(fact: f32) -> Self {
        return Self(0.0, -fact, 0.0);
    }

    pub const fn right(fact: f32) -> Self {
        return Self(fact, 0.0, 0.0);
    }

    pub const fn left(fact: f32) -> Self {
        return Self(-fact, 0.0, 0.0);
    }

    pub const fn forw(fact: f32) -> Self {
        return Self(0.0, 0.0, fact);
    }

    pub const fn back(fact: f32) -> Self {
        return Self(0.0, 0.0, -fact);
    }

    pub const fn one(fact: f32) -> Self {
        return Self(fact, fact, fact);
    }

    /// Creates a vector in the local space of a basis
    /*pub const fn local(x: f32, y: f32, basis: (Vector2, Vector2)) -> Self {
        return Vector2(x * basis.0.0 + y * basis.1.0, x * basis.0.1 + y * basis.1.1)
    }*/


    pub fn x(self) -> f32 {
        return self.0;
    }

    pub fn y(self) -> f32 {
        return self.1;
    }

    pub fn z(self) -> f32 {
        return self.2;
    }

    
    pub fn xy(self) -> Vector2 {
        return Vector2(self.0, self.1);
    }

    pub fn yz(self) -> Vector2 {
        return Vector2(self.1, self.2);
    }

    pub fn xz(self) -> Vector2 {
        return Vector2(self.0, self.2);
    }


    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    pub fn set_z(&mut self, z: f32) {
        self.2 = z;
    }

    pub fn xvec(self) -> Self {
        return Self(self.0, 0.0, 0.0);
    }

    pub fn yvec(self) -> Self {
        return Self(0.0, self.1, 0.0);
    }

    pub fn zvec(self) -> Self {
        return Self(0.0, 0.0, self.2);
    }

    /*pub fn cross(self) -> Self {
        return Self(self.1, -self.0);
    }

    /// Rotates the vector by the rotation provided in radians.<br>
    /// For rotations that need to match visuals, use `rotate_cw`
    pub fn rotate(self, rot: f32) -> Self {
        return Self(self.0 * rot.cos() + self.1 * rot.sin(), self.0 * rot.sin() - self.1 * rot.cos());
    }

    /// Rotates the vector by the specified rotation, but in a way that matches up with visual rotations.
    pub fn rotate_cw(self, rot: f32) -> Self {
        let rotated = self.rotate(rot);
        return Vector2(rotated.0, -rotated.1);
    }*/

    /// Performs the dot product between two vectors.
    pub fn dot(self, other: Self) -> f32 {
        return (self.to_simd() * other.to_simd()).reduce_sum();
    }

    pub fn sqr_magnitude(self) -> f32 {
        return self.dot(self);
    }

    /// Returns the magnitude of the vector.
    pub fn magnitude(self) -> f32 {
        if self == Self::ZERO {
            return 0.0;
        }

        return (self.to_simd() * self.to_simd()).reduce_sum().sqrt();
    }

    /// Returns the vector with a magnitude of 1.
    pub fn normalized(self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            return self;
        }

        return self / mag;
    }

    /// Returns the vector divided by the squared magnitude.
    pub fn inverted(self) -> Self {
        let sqr_mag = self.dot(self);
        if sqr_mag == 0.0 {
            return self;
        }

        return self / sqr_mag;
    }

    /// Multiplies the vectors component-wise
    pub fn scale(self, other: Self) -> Self {
        return Self::from_simd(self.to_simd() * other.to_simd());
    }

    /// Divides the vectors component-wise
    pub fn inv_scale(self, other: Self) -> Self {
        return Self::from_simd(self.to_simd() / other.to_simd());
    }

    /// Inverts each component
    pub fn inv_dims(self) -> Self {
        return Self::from_simd(f32x4::splat(1.0) / self.to_simd());
    }

    pub fn lerp(self, other: Self, fact: f32) -> Self {
        return super::lerp(self, other, fact);
    }

    pub fn average(slice: &[Self]) -> Self {
        let mut res = Self::ZERO;
        for v in slice {
            res += *v;
        }
        return res / slice.len() as f32;
    }

    pub fn dist_to(self, other: Self) -> f32 {
        (self - other).magnitude()
    }

    pub fn sqr_dist_to(self, other: Self) -> f32 {
        (self - other).sqr_magnitude()
    }


    pub fn min(self, other: Self) -> Self {
        Self::from_simd(self.to_simd().simd_min(other.to_simd()))
    }

    pub fn max(self, other: Self) -> Self {
        Self::from_simd(self.to_simd().simd_max(other.to_simd()))
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    
    pub fn min_mag(self, other: f32) -> Self {
        let mag = self.magnitude();
        return self / mag * mag.min(other);
    }

    pub fn max_mag(self, other: f32) -> Self {
        let mag = self.magnitude();
        return self / mag * mag.max(other);
    }

    pub fn clamp_mag(self, min: f32, max: f32) -> Self {
        let mag = self.magnitude();
        return self / mag * mag.clamp(min, max);
    }


    pub fn max_axis(self) -> f32 {
        return self.to_simd_ext(f32::NEG_INFINITY).reduce_max();
    }

    pub fn min_axis(self) -> f32 {
        return self.to_simd_ext(f32::INFINITY).reduce_min();
    }


    pub fn floor(self) -> Self {
        return Self::from_simd(self.to_simd().floor());
    }

    pub fn round(self) -> Self {
        return Self::from_simd(self.to_simd().round());
    }

    pub fn ceil(self) -> Self {
        return Self::from_simd(self.to_simd().ceil());
    }


    pub fn to_simd(self) -> f32x4 {
        f32x4::from_array([self.0, self.1, self.2, 0.0])
    }

    pub fn to_simd_ext(self, w: f32) -> f32x4 {
        f32x4::from_array([self.0, self.1, self.2, w])
    }

    pub fn from_simd(simd: f32x4) -> Self {
        Self(simd[0], simd[1], simd[2])
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::from_simd(self.to_simd() + rhs.to_simd());
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::from_simd(self.to_simd() - rhs.to_simd());

    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self::from_simd(-self.to_simd());
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        return Self::from_simd(self.to_simd() * f32x4::splat(rhs));
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        return Self::from_simd(self.to_simd() / f32x4::splat(rhs));
    }
}

impl Into<(f32, f32, f32)> for Vector3 {
    fn into(self) -> (f32, f32, f32) {
        return (self.0, self.1, self.2);
    }
}

impl Into<Vector3> for (f32, f32, f32) {
    fn into(self) -> Vector3 {
        return Vector3(self.0, self.1, self.2);
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl From<(u32, u32, u32)> for Vector3 {
    fn from(value: (u32, u32, u32)) -> Self {
        return Vector3(value.0 as f32, value.1 as f32, value.2 as f32);
    }
}