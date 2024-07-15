use std::ops::Mul;

use crate::assert_expr;

use super::vec2;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix3x3 {
    rows: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub const IDENTITY: Self = Self { rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    pub fn translate(&mut self, delta: vec2) {
        self.rows[0][2] += delta.0;
        self.rows[1][2] -= delta.1;
    }

    pub fn scale(&mut self, mul: vec2) {
        self.rows[0][0] *= mul.0;
        self.rows[1][0] *= mul.0;
        self.rows[0][1] *= mul.1;
        self.rows[1][1] *= mul.1;
    }

    pub fn rotate(&mut self, rot: f32) {
        let x = vec2(self.rows[0][0], self.rows[1][0]).rotate(rot);
        let y = vec2(self.rows[0][1], self.rows[1][1]).rotate(rot);

        self.rows[0][0] = x.0;
        self.rows[1][0] = x.1;

        self.rows[0][1] = y.0;
        self.rows[1][1] = y.1;
    }

    pub fn transform_matrix(pos: vec2, rot: f32, scale: vec2) -> Self {
        let mut mat = Self::IDENTITY.clone();
        mat.translate(pos);
        mat.scale(scale);
        mat.rotate(rot);
        return mat;
    }

    pub fn inv_transform_matrix(pos: vec2, rot: f32, scale: vec2) -> Self {
        let mut mat = Self::transform_matrix(pos, rot, scale);
        mat.invert();
        return mat;
    }

    pub fn cam_matrix(pos: vec2, dims: vec2) -> Self {
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
        let det = self.determinant();
        assert_expr!(det != 0.0, "Matrix is not inversible.");

        let inv_det = 1.0 / det;
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

impl Mul<vec2> for &Matrix3x3 {
    type Output = vec2;

    fn mul(self, rhs: vec2) -> Self::Output {
        return vec2(
            self.rows[0][0] * rhs.0 + self.rows[0][1] * rhs.1 + self.rows[0][2],
            self.rows[1][0] * rhs.0 + self.rows[1][1] * rhs.1 + self.rows[1][2]
        );
    }
}