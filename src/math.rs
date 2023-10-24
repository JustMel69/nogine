use std::ops::{Add, Neg, Sub, Mul, Div, AddAssign};

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

#[derive(Clone)]
pub struct Matrix3x3 {
    rows: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub const IDENTITY: Self = Self { rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    pub fn translate(&mut self, delta: Vector2) {
        self.rows[0][2] += delta.0;
        self.rows[1][2] -= delta.1;
    }

    pub fn scale(&mut self, mul: Vector2) {
        self.rows[0][0] *= mul.0;
        self.rows[1][0] *= mul.0;
        self.rows[0][1] *= mul.1;
        self.rows[1][1] *= mul.1;
    }

    pub fn rotate(&mut self, rot: f32) {
        let x = Vector2(self.rows[0][0], self.rows[1][0]).rotate(rot);
        let y = Vector2(self.rows[0][1], self.rows[1][1]).rotate(rot);

        self.rows[0][0] = x.0;
        self.rows[1][0] = x.1;

        self.rows[0][1] = y.0;
        self.rows[1][1] = y.1;
    }

    pub fn transform_matrix(pos: Vector2, rot: f32, scale: Vector2) -> Self {
        let mut mat = Self::IDENTITY.clone();
        mat.translate(pos);
        mat.scale(scale);
        mat.rotate(rot);
        return mat;
    }

    pub fn inv_transform_matrix(pos: Vector2, rot: f32, scale: Vector2) -> Self {
        let mut mat = Self::transform_matrix(pos, rot, scale);
        mat.invert();
        return mat;
    }

    pub fn cam_matrix(pos: Vector2, dims: Vector2) -> Self {
        let mut mat = Self::IDENTITY.clone();
        mat.rows[0][0] = 1.0 / dims.0;
        mat.rows[1][1] = -1.0 / dims.1;
        mat.rows[0][2] = -pos.0 / dims.0;
        mat.rows[1][2] = -pos.1 / dims.1;
        return mat;
    }

    pub fn determinant(&self) -> f32 {
        return self.rows[0][0] * (self.rows[1][1] * self.rows[2][2] - self.rows[2][1] * self.rows[1][2]) -
            self.rows[0][1] * (self.rows[1][0] * self.rows[2][2] - self.rows[2][0] * self.rows[1][2]) +
            self.rows[0][2] * (self.rows[1][0] * self.rows[2][1] - self.rows[2][0] * self.rows[1][1])
    }

    pub fn invert(&mut self) {
        let inv_det = 1.0 / self.determinant();
        let src = self.rows.clone();

        self.rows = [
            [
                (src[1][1] * src[2][2] - src[2][1] * src[1][2]) * inv_det,
                -(src[1][0] * src[2][2] - src[2][0] * src[1][2]) * inv_det,
                (src[1][0] * src[2][1] - src[2][0] * src[1][1]) * inv_det,
            ],
            [
                -(src[0][1] * src[2][2] - src[2][1] * src[0][2]) * inv_det,
                (src[0][0] * src[2][2] - src[2][0] * src[0][2]) * inv_det,
                -(src[0][0] * src[2][1] - src[2][0] * src[0][1]) * inv_det,
            ],
            [
                (src[0][1] * src[1][2] - src[1][1] * src[0][2]) * inv_det,
                -(src[0][0] * src[1][2] - src[1][0] * src[0][2]) * inv_det,
                (src[0][0] * src[1][1] - src[1][0] * src[0][1]) * inv_det,
            ],
        ]
    }

    pub fn inverse(&self) -> Self {
        let mut res = self.clone();
        res.invert();
        return res;
    }

    pub fn ptr(&self) -> *const f32 {
        return self.rows.as_ptr() as *const f32;
    }
}

pub fn lerp<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    let t = t.clamp(0.0, 1.0);
    return b * t + a * (1.0 - t);
}

pub fn lerp_unclamped<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    return b * t + a * (1.0 - t);
}

impl Mul for &Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix3x3 { rows: [[0.0; 3]; 3] };
        for i in 0..3 {
            let (a, b, c) = (self.rows[i][0], self.rows[i][1], self.rows[i][2]);
            for j in 0..3 {
                res.rows[i][j] = a * rhs.rows[0][j] + b * rhs.rows[1][j] + c * rhs.rows[2][j];
            }
        }
        return res;
    }
}


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