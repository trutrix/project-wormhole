use crate::dev::*;

// Unknown structs here appear to just be f32 lists of varying lengths
// Maybe make wrapper structs for them that pick specific indices and fail gracefully?

define_record2! {
    b"CSTY",
    CombatStyle, [
        EditorId;
        b"CSFL", Flight, Vec<f32>; // Unknown Struct - Always 32
        b"CSME", Melee, Vec<f32>; // Unknown Struct - Always 36 and 40
        b"DATA", Flags, CombatStyleFlags;
        b"CSGD", General, Vec<f32>; // Unknown Struct - Always 48
        b"CSCV", CoverSearchDistanceMultiplier, f32;
        b"CSCR", CloseRange, Vec<f32>; // Unknown Struct - Always 44
        b"CSLR", LongRange, Vec<f32>; // Unknown Struct - Always 12, 16, 20
        b"CSRA", RangedAccuracyMultiplier, f32;
    ]
}


#[derive(Debug, NomLE)]
pub struct CombatStyleFlags(pub u32);