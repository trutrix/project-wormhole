use crate::dev::*;

define_record3! {
    "iden": b"CPTH";
    "name": CameraPath;
    "fields": [
        EditorId;
        Condition;
        b"ANAM", RelatedPaths, (FormId, FormId); // Parent / Sibling
        b"DATA", Flags, CameraPathFlags;
        b"SNAM", CameraShots, Vec<FormId>;
    ]
}


#[derive(Debug, NomLE, PartialEq)]
pub struct CameraPathFlags(pub u8);