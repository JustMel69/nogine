use std::ops::{Add, Neg, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct Vector2(pub f32, pub f32);

impl Vector2 {
    pub const ZERO: Self = Self(0.0, 0.0);
    pub const UP: Self = Self(0.0, 1.0);
    pub const DOWN: Self = Self(0.0, -1.0);
    pub const RIGHT: Self = Self(1.0, 0.0);
    pub const LEFT: Self = Self(-1.0, 0.0);
    pub const ONE: Self = Self(1.0, 1.0);

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
        return lerp(self, other, fact);
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self(self.0 + rhs.0, self.1 + rhs.1);
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

#[derive(Clone)]
pub struct Matrix3x3 {
    val: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub const IDENTITY: Self = Self { val: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    pub fn translate(&mut self, delta: Vector2) {
        self.val[0][2] += delta.0;
        self.val[1][2] += delta.1;
    }

    pub fn scale(&mut self, mul: Vector2) {
        self.val[0][0] *= mul.0;
        self.val[1][0] *= mul.0;
        self.val[0][1] *= mul.1;
        self.val[1][1] *= mul.1;
    }

    pub fn rotate(&mut self, rot: f32) {
        let x = Vector2(self.val[0][0], self.val[1][0]).rotate(rot);
        let y = Vector2(self.val[0][1], self.val[1][1]).rotate(rot);

        self.val[0][0] = x.0;
        self.val[1][0] = x.1;

        self.val[0][1] = y.0;
        self.val[1][1] = y.1;
    }

    pub fn transform_matrix(pos: Vector2, rot: f32, scale: Vector2) -> Self {
        let mut mat = Self::IDENTITY.clone();
        mat.translate(pos);
        mat.scale(scale);
        mat.rotate(rot);
        return mat;
    }

    pub fn inv_transform_matrix(pos: Vector2, rot: f32, scale: Vector2) -> Self {
        let mut mat = Self::IDENTITY.clone();
        mat.rotate(-rot);
        mat.scale(scale.inv_dims());
        mat.translate(-pos);
        return mat;
    }

    pub fn ptr(&self) -> *const f32 {
        return self.val.as_ptr() as *const f32;
    }
}

pub fn lerp<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    let t = t.clamp(0.0, 1.0);
    return b * t + a * (1.0 - t);
}

pub fn lerp_unclamped<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    return b * t + a * (1.0 - t);
}