use crate::dev::*;

define_record! {
    b"CPTH",
    CameraPath, [
        EditorId;
        Condition;
        b"ANAM", RelatedPaths, (FormId, FormId); // Parent / Sibling
        b"DATA", Flags, CameraPathFlags;
        b"SNAM", CameraShots, Vec<FormId>;
    ]
}


#[derive(Debug, NomLE)]
pub struct CameraPathFlags(pub u8);