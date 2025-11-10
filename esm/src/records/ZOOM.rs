use crate::dev::*;

define_record! {
    b"ZOOM",
    Zoom, [
        EditorId;
        b"GNAM", Data, ZoomData;
    ]
}

#[derive(Debug, NomLE)]
pub struct ZoomData {
    pub fov_multiplier: f32,
    pub overlay: u32, // TODO: enum?
    pub image_space: FormId,
    pub camera_offset: Vec3<f32>
}