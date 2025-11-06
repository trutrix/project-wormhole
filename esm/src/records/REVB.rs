use crate::dev::*;

define_record! {
    b"REVB",
    Reverb, [
        EditorId;
        b"DATA", Data, ReverbData;
        b"ANAM", Class, u32; // TODO: enum?;
    ]
}

// todo: fill out
// size 14 bytes
#[derive(Debug, NomLE)]
pub struct ReverbData {

}