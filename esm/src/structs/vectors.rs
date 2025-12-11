use std::ops::{Div, Mul};

use crate::dev::*;


// ====================================================================================================

#[derive(Debug, Clone, PartialEq, Copy, NomLE, serde::Serialize, serde::Deserialize)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: From<u8>> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.into(), y: 0.into() }
    }

    pub fn one() -> Self {
        Self { x: 1.into(), y: 1.into() }
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(v: Vec2<T>) -> [T; 2] {
        [v.x, v.y]
    }
}

// ====================================================================================================

#[derive(Debug, Clone, PartialEq, Eq, Copy, NomLE, serde::Serialize, serde::Deserialize)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: From<u8>> Vec3<T> {
    /// Create a new vector3  
    /// Easier to use than the struct constructor
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Get the length of the vector  
    /// Converts the vector type to `f32`   
    /// It it up to the caller to convert back if needed.
    pub fn length(&self) -> f32 
        where T: Mul<Output = f32> + Copy
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize the vector to a unit vector  
    /// Each value is divided by the length of the vector
    pub fn normalize(&self) -> Vec3<f32>
        where T: Mul<Output = f32> + Div<f32, Output = f32> + Copy
    {
        let length = self.length();
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }


    /// Create zero vector3 `Vec3 {x: 0, y: 0, z: 0}`
    pub fn zero() -> Self {
        Self { x: 0.into(), y: 0.into(), z: 0.into()}
    }

    /// Create one vector3 `Vec3 {x: 1, y: 1, z: 1}`
    pub fn one() -> Self {
        Self { x: 1.into(), y: 1.into(), z: 1.into() }
    }

}


// Multiplication for Vec3 againsts a number
impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
} 


// Multiplication for Vec3 againsts another Vec3
impl<T: Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}


// For converting from a structure to an array
impl<T> From<Vec3<T>> for [T; 3] {
    fn from(v: Vec3<T>) -> [T; 3] {
        [v.x, v.y, v.z]
    }
}

// For converting from an array to a structure
impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(v: [T; 3]) -> Vec3<T> {
        Vec3 { x: v[0], y: v[1], z: v[2] }
    }
}


// ====================================================================================================

#[derive(Debug, Clone, PartialEq, Copy, NomLE, serde::Serialize, serde::Deserialize)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T: From<u8>> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Create empty vector4 `Vec4 {x: 0, y: 0, z: 0, w: 1}`
    pub fn empty() -> Self {
        Self { x: 0.into(), y: 0.into(), z: 0.into(), w: 1.into() }
    }

    /// Create zero vector4 `Vec4 {x: 0, y: 0, z: 0, w: 0}`
    pub fn zero() -> Self {
        Self { x: 0.into(), y: 0.into(), z: 0.into(), w: 0.into() }
    }

    /// Create one vector4 `Vec4 {x: 1, y: 1, z: 1, w: 1}`
    pub fn one() -> Self {
        Self { x: 1.into(), y: 1.into(), z: 1.into(), w: 1.into() }
    }
}

// For converting from a structure to an array
impl<T> From<Vec4<T>> for [T; 4] {
    fn from(v: Vec4<T>) -> [T; 4] {
        [v.x, v.y, v.z, v.w]
    }
}

// For converting from an array to a structure
impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(v: [T; 4]) -> Vec4<T> {
        Vec4 { x: v[0], y: v[1], z: v[2], w: v[3] }
    }
}

impl Vec4<f32> {
    /// Manhatten length (absolute sum of all components)
    pub fn mlen(&self) -> f32 {
        self.x.abs() + self.y.abs() + self.z.abs() + self.w.abs()
    }

    /// Normalize the vector
    pub fn normalize_weights(&self) -> Self {
        let length = 1.0 / self.mlen();
        let x = self.x * length;
        let y = self.y * length;
        let z = self.z * length;
        let w = self.w * length;
        Self {x, y, z, w}
    }

    /// Length of the vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}