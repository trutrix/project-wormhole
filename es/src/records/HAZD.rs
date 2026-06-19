use crate::dev::*;

define_record3! {
    "iden": b"HAZD";
    "name": Hazard;
    "fields": [
        EditorId;
        FullName;
        ModelData;
        ObjectBounds;
        b"DNAM", Data, HazardData;
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}


// TODO: 52 bytes
#[derive(Debug, NomLE, PartialEq)]
pub struct HazardData {
    
}