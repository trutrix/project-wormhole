use crate::dev::*;

define_record2! {
    b"ECZN",
    EncounterZone, [
        EditorId;
        b"DATA", Data, EncounterZoneData;
    ]
}


#[derive(Debug, NomLE)]
pub struct EncounterZoneData {
    pub owner: FormId,
    pub location: FormId,
    pub rank: u8,
    pub min_level: u8,
    pub flags: u8,
    pub max_level: u8
}