use crate::dev::*;

define_record2! {
    b"GLOB",
    Global, [
        EditorId;
        b"FNAM", Type, u8; // Changes type of value stored
        b"FLTV", Value, [u8;4];
    ]
}