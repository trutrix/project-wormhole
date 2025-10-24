use crate::dev::*;

define_record! {
    b"LCRT",
    LocationReferenceType, [
        EditorId;
        b"CNAM", Color, Color4;
        b"TNAM", Unknown1, u32; // 4 bytes - sometimes present but the value is zero
    ]
}