use crate::dev::*;

define_record3! {
    "iden": b"SOUN";
    "name": SoundMarker;
    "fields": [
        EditorId;
        ObjectBounds;
        b"SDSC", SoundDescriptor, FormId;
        b"REPT", Repeat, SoundMarkerRepeat;
    ]
}


// Min 8 bytes, Sometimes 9 bytes
#[derive(Debug, NomLE)]
pub struct SoundMarkerRepeat {
    pub min_time: f32,
    pub max_time: f32,
    pub stackable: Option<u8>
}