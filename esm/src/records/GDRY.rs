use crate::dev::*;

define_record3! {
    "iden": b"GDRY";
    "name": Godray;
    "fields": [
        EditorId;
        b"DATA", Data, GodrayData;
    ]
}


// 60 bytes
#[derive(Debug, NomLE, PartialEq)]
pub struct GodrayData {
    
}