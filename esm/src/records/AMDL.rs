use crate::dev::*;

// This record forgoes the usual fields in favor of a single DATA structure

define_record3! {
    "iden": b"AMDL";
    "name": AimModel;
    "fields": [
        EditorId;
        b"DATA", AimModelData, AimModelData;
    ]
}


// TODO: Find actual structure
#[derive(Debug, NomLE, PartialEq)]
pub struct AimModelData {
    
}