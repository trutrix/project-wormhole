use crate::dev::*;

define_record! {
    b"PERK",
    Perk, [
        EditorId;
        VirtualMachineAdapter;
        FullName;
        Description;
        Condition;
        b"SNAM", Sound, FormId;
        b"FNAM", FlashFilePath, ESMString;
        b"DATA", Data, PerkData;
        b"NNAM", NextPerk, FormId;

        b"PRKF", EffectEnd, EmptyParser;
        b"PKRE", EffectHeader, PerkEffectHeader;
        b"PRKC", EffectRunOnIndex, u8;

        b"EPFT", EffectFlags, u8; // TODO: bitflags
        b"EPFD", EffectData, Vec<(FormId, f32)>; 
        b"EPF2", EffectButtonLabel, LocalizedString;
        b"EPF3", EffectScriptFlags, u16; // TODO: bitflags

    ]
}


#[derive(Debug, NomLE)]
pub struct PerkData {
    // TODO: shared with effects - lengths 3, 4, 5 and 6 observed
}

#[derive(Debug, NomLE)]
pub struct PerkEffectHeader {
    pub type_: u8, // TODO: enum
    pub rank: u8,
    pub priority: u8
}