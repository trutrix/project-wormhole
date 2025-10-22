use crate::dev::*;

define_record! {
    b"FACT",
    Faction, [
        EditorId;
        FullName;
        Condition;
        b"DATA", Flags, FactionFlags;
        b"XNAM", FactionRelation, FactionRelation; // More than one can appear per record
        b"CRGR", SharedCrimeList, FormId;
        b"VEND", VendorItemList, FormId;
        b"VENC", VendorContainer, FormId; // Refers to a REFR usually
        b"VENV", VendorValues, VendorValues;
        b"CITC", ConditionCount, u32;
        b"CRVA", CrimeValues, CrimeValues;
        b"RNAM", Ranks, Vec<FormId>; // Unsure if list or single
        b"PLVD", Location, FactionLocation;
    ]
}


#[derive(Debug, NomLE)]
pub struct FactionFlags(pub u32);



#[derive(Debug, NomLE)]
pub struct FactionRelation {
    pub faction: FormId,
    pub modifier: u32,
    pub reaction: u32
}

// 12 bytes - 8 values
// TODO: Verify structure - these are guesses
#[derive(Debug, NomLE)]
pub struct VendorValues {
    pub start_hour: u8,
    pub end_hour: u8,
    pub radius: u16,
    pub unknown1: u32,
    pub buy_stolen: u8,
    pub buy_non_listed: u8,
    pub buy_non_stolen: u8,
    pub unknown2: u8,
}

// 20 bytes - 10 values
// TODO: Verify structure - these are guesses
#[derive(Debug, NomLE)]
pub struct CrimeValues {
    pub arrest: u8,
    pub attack: u8,
    pub murder: u16,
    pub assault: u16,
    pub trespass: u16,
    pub pick_pocket: u16,
    pub unknown: u16,
    pub steal_multiplier: f32,
    pub escape: u16,
    pub werewolf: u16 // Skyrim only
}

// 16 bytes - 4 values
#[derive(Debug, NomLE)]
pub struct FactionLocation {
    pub type_: u32,
    pub unknown: u32,
    pub radius: u32,
    pub collection_index: u32
}