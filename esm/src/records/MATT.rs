use crate::dev::*;

define_record! {
    b"MATT",
    MaterialType, [
        EditorId;
        AllModelData; // Maybe only MODT?
        b"MNAM", MaterialName, ESMString;
        b"CNAM", HavokColor, [u32;3]; // TODO: 12 bytes, three u32s seems like overkill
        b"BNAM", Buoyancy, f32;
        b"FNAM", Flags, u32;
        b"HNAM", HavokImpactDataSet, FormId; // IPDS
        b"ANAM", BreakableEffect, ESMString;
        b"PNAM", MaterialParent, FormId; // MATT
    ]
}