use crate::dev::*;

define_record3! {
    "iden": b"GLOB";
    "name": Global;
    "fields": [
        EditorId;
        b"FNAM", Type, u8; // Changes type of value stored
        b"FLTV", Value, [u8;4];
    ]
}