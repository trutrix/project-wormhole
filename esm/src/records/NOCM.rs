use crate::dev::*;

define_record3! {
    "iden": b"NOCM";
    "name": NavObstacleManager;
    "fields": [
        // No EDID, single record
        b"INDX", Index, u32;
        b"DATA", Data, NavigationObstacleManagerData;
        b"INTV", Unknown1, u32; // Maybe related to INTV from TES4 (header) record?
        b"NAM1", ModelPath, ESMString;
    ]
}

#[derive(Debug, NomLE)]
pub struct NavigationObstacleManagerData {
    // TODO: fill out, always 8 bytes
}