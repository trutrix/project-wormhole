use crate::dev::*;

// This record forgoes the usual fields in favor of a single DATA structure

define_record! {
    b"AMDL",
    AimModel, [
        EditorId;
        b"DATA", AimModelData, AimModelData;
    ]
}


// TODO: Find actual structure
#[derive(Debug, NomLE)]
pub struct AimModelData {
    
}