use crate::dev::*;

define_record! {
    b"HAZD",
    Hazard, [
        EditorId;
        FullName;
        ModelData;
        ObjectBounds;
        b"DNAM", Data, HazardData;
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}


// TODO: 52 bytes
#[derive(Debug, NomLE)]
pub struct HazardData {
    
}