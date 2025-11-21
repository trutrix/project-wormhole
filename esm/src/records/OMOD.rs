use crate::dev::*;

define_record2! {
    b"OMOD",
    ObjectModification, [
        EditorId;
        FullName;
        Description;
        ModelData;
        b"DATA", Data, ObjectModificationData;
        b"NAM1", Priority, u32;
        b"FNAM", FilterKeywords, Vec<FormId>;
        b"FLTR", Filter, EmptyParser; // Unknown format, only size known is 3 bytes
        b"LNAM", LooseMod, FormId;
        b"MNAM", TargetOMODKeywords, Vec<FormId>;
    ]
}


#[derive(Debug, NomLE)]
pub struct ObjectModificationData {
    // TODO: fill out - variable length
}