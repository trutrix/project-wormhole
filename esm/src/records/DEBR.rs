use crate::dev::*;

define_record3! {
    "iden": b"DEBR";
    "name": Debris;
    "fields": [
        EditorId;
        ModelData;
        b"DATA", Data, u8;
    ]
}



// This is probably wrong
#[derive(Debug, NomLE)]
pub struct DebrisData {
    pub percentage: u32,
    pub model_path: ESMString,
    pub flags: u8
}