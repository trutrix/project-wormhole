use crate::dev::*;

define_record! {
    b"MGEF",
    MagicEffect, [
        EditorId;
        Keywords;
        FullName;
        VirtualMachineAdapter;
        Condition;
        b"DATA", Data, MagicEffectData;
        b"DNAM", ItemDescription, LocalizedString;
        b"SNDD", Sounds, EmptyParser; //(FormId, FormId); // TODO: Sometimes field is zero length to denote NULL
    ]
}


#[derive(Debug, NomLE)]
pub struct MagicEffectData {
    // TODO: fill out
}