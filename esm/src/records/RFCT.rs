use crate::dev::*;

define_record3! {
    "iden": b"RFCT";
    "name": VisualEffect;
    "fields": [
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