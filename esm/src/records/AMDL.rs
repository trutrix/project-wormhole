use crate::dev::*;

define_record! {
    b"AMDL",
    AimModel, [
        b"EDID", EditorId, ESMString;
        b"DATA", AimModelData, AimModelData;
    ]
}

#[derive(Debug, NomLE)]
pub struct AimModelData {
    
}