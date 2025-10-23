use crate::dev::*;

define_record! {
    b"GRAS",
    Grass, [
        EditorId;
        ObjectBounds;
        AllModelData;
        b"DATA", Data, GrassData;
    ]
}


// 32 bytes
// TODO: best guess, needs confirmation
#[derive(Debug, NomLE)]
pub struct GrassData {
    pub density: u8,
    pub min_slope: u16,
    pub max_slope: u16,
    pub unknown1: u8,
    pub distance_from_water: u16,
    pub unknown2: u16,
    pub distance_from_water_type: u16,
    pub position_range: f32,
    pub height_range: f32,
    pub color_range: f32,
    pub wave_period: f32,
    pub flags: u8,
    pub unknown3: [u8;3] // Padding?
}