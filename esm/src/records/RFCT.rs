use crate::dev::*;

define_record! {
    b"RFCT",
    VisualEffect, [
        EditorId;
        b"DATA", Data, VisualEffectData;
    ]
}


#[derive(Debug, NomLE)]
pub struct VisualEffectData {
    pub effect: FormId,
    pub art: FormId,
    pub flags: u32
}