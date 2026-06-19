use crate::dev::*;

define_record3! {
    "iden": b"TRNS";
    "name": Transform;
    "fields": [
        EditorId;
        b"DATA", Data, TransformData;
    ]
}

// Sizes: 28 and 36
#[derive(Debug, NomLE, PartialEq)]
pub struct TransformData {
    pub position: [f32; 3],
    pub rotation: [f32; 3], // TODO: Needs conversion
    pub scale: f32,
    pub zoom_min: Option<f32>,
    pub zoom_max: Option<f32>
}