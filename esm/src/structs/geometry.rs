use crate::dev::*;


pub type ModelPath = ESMString;
pub type ModelTexture = u8; // TODO: Define data structure
pub type ModelColorMap = u8; // TODO: Define data structure
pub type ModelMaterialSwap = u8; // TODO: Define data structure
pub type ModelFlags = u32; // TODO: Define flags



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