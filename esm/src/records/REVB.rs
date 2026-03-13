use crate::dev::*;

define_record3! {
    "iden": b"REVB";
    "name": Reverb;
    "fields": [
        EditorId;
        b"DATA", Data, ReverbData;
        b"ANAM", Class, u32; // TODO: enum?;
    ]
}

// todo: fill out
// size 14 bytes
#[derive(Debug, NomLE, PartialEq)]
pub struct ReverbData {

}