use crate::dev::*;

define_record2! {
    b"GDRY",
    Godray, [
        EditorId;
        b"DATA", Data, GodrayData;
    ]
}


// 60 bytes
#[derive(Debug, NomLE)]
pub struct GodrayData {
    
}