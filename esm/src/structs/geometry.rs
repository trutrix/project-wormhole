use crate::dev::*;





// OBND
#[derive(Debug, NomLE)]
pub struct ObjectBounds {
    pub x1: i16,
    pub y1: i16,
    pub z1: i16,
    pub x2: i16,
    pub y2: i16,
    pub z2: i16,
}


/// Location in world grid for cell
#[derive(Debug, Clone, NomLE)]
pub struct CellLoc {
    pub y: i16,
    pub x: i16
}


impl From<u32> for CellLoc {
    fn from(value: u32) -> Self {
        let b = value.to_le_bytes();

        let y = [b[0], b[1]];
        let x = [b[2], b[3]];

        let y = i16::from_le_bytes(y);
        let x = i16::from_le_bytes(x);
        CellLoc { y, x }
    }
}


#[derive(Debug, NomLE)]
pub struct LocationRotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
}


#[derive(Debug, PartialEq, Clone)]
/// A quaternion in the form of [x, y, z, w]
pub struct Quaternion(Vec4<f32>);

impl Quaternion {
    pub const fn x(&self) -> &f32 { &self.0.x }
    pub const fn y(&self) -> &f32 { &self.0.y }
    pub const fn z(&self) -> &f32 { &self.0.z }
    pub const fn w(&self) -> &f32 { &self.0.w }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion(Vec4{x, y, z, w})
    }
}


impl Default for Quaternion {
    fn default() -> Self {
        Quaternion(Vec4::empty())
    }
}

impl From<Quaternion> for gltf::json::scene::UnitQuaternion {
    fn from(q: Quaternion) -> Self {
        gltf::json::scene::UnitQuaternion([q.0.x, q.0.y, q.0.z, q.0.w])
    }
}