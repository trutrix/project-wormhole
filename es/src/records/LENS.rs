use crate::dev::*;

// TODO: Edge case - DNAM appears multiple times but the first is always f32 
// this may require custom parsing logic

define_record3! {
    "iden": b"LENS";
    "name": LensFlare;
    "fields": [
        EditorId;
        b"CNAM", Color, Color4;
        b"DNAM", FadeDistanceScale, LensFlareFadeDistanceScale;
        b"LFSD", Data, LensFlareData;
        b"FNAM", Texture, ESMString;
        b"LFSP", Count, u32;
    ]
}


#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct  LensFlareFadeDistanceScale {

}


#[derive(Debug, NomLE, PartialEq)]
pub struct LensFlareData {
    pub tint: Color3,
    pub width: f32,
    pub height: f32,
    pub position: f32,
    pub angular_fade: f32,
    pub opacity: f32,
    pub flags: u8,
    pub unknown: [u8; 12]
}