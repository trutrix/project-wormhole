use crate::dev::*;

define_record! {
    b"DEBR",
    Debris, [
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