use std::ops::{Div, DivAssign, Mul, MulAssign};

use crate::dev::*;


#[derive(Debug, PartialEq, Eq, NomLE, Clone, Copy)]
pub struct Matrix3<T>(pub [T; 9]);

#[derive(Debug, PartialEq, Eq, NomLE, Clone, Copy)]
pub struct Matrix4<T>(pub [T; 16]);


impl Default for Matrix3<f32> {
    fn default() -> Self {
        Self([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        ])
    }
}

impl Default for Matrix4<f32> {
    fn default() -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }
}


impl<T: MulAssign<T> + Copy> Mul<T> for Matrix3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        for i in 0..9 {
            result.0[i] *= rhs;
        }
        result
    }
}


impl<T: DivAssign<T> + Copy> Div<T> for Matrix3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let mut result = self;
        for i in 0..9 {
            result.0[i] /= rhs;
        }
        result
    }
}


impl<T: Copy> Matrix3<T> {
    pub fn to_col_major(&self) -> Self {
        Self([
            self.0[0], self.0[3], self.0[6],
            self.0[1], self.0[4], self.0[7],
            self.0[2], self.0[5], self.0[8]
        ])
    }

    pub fn to_row_major(&self) -> Self {
        Self([
            self.0[0], self.0[1], self.0[2],
            self.0[3], self.0[4], self.0[5],
            self.0[6], self.0[7], self.0[8]
        ])
    }
}

impl<T: Copy> Matrix4<T> {
    pub fn to_col_major(&self) -> Self {
        Self([
            self.0[0], self.0[4], self.0[8], self.0[12],
            self.0[1], self.0[5], self.0[9], self.0[13],
            self.0[2], self.0[6], self.0[10], self.0[14],
            self.0[3], self.0[7], self.0[11], self.0[15]
        ])
    }

    pub fn to_row_major(&self) -> Self {
        Self([
            self.0[0], self.0[1], self.0[2], self.0[3],
            self.0[4], self.0[5], self.0[6], self.0[7],
            self.0[8], self.0[9], self.0[10], self.0[11],
            self.0[12], self.0[13], self.0[14], self.0[15]
        ])
    }
}



impl From<Matrix3<f32>> for Quaternion {
    fn from(value: Matrix3<f32>) -> Self {

        let m00 = value.0[0];
        let m10 = value.0[1];
        let m20 = value.0[2];
        let m01 = value.0[3];
        let m11 = value.0[4];
        let m21 = value.0[5];
        let m02 = value.0[6];
        let m12 = value.0[7];
        let m22 = value.0[8];

        let qw = (1.0 + m00 + m11 + m22).max(0.0).sqrt() / 2.0;
        let mut qx = (1.0 + m00 - m11 - m22).max(0.0).sqrt() / 2.0;
        let mut qy = (1.0 - m00 + m11 - m22).max(0.0).sqrt() / 2.0;
        let mut qz = (1.0 - m00 - m11 + m22).max(0.0).sqrt() / 2.0;

        qx = qx.copysign(m21 - m12);
        qy = qy.copysign(m02 - m20);
        qz = qz.copysign(m10 - m01);

        Quaternion::new(qx, qy, qz, qw)

    }
}


impl Matrix4<f32> {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for value in &self.0 {
            data.extend_from_slice(&value.to_le_bytes());
        }
        data
    }
}