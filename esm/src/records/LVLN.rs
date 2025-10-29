use crate::dev::*;

define_record! {
    b"LVLN",
    LeveledNPC, [
        EditorId;
        ObjectBounds;
        AllModelData;
        b"LVLO", LeveledObject, EmptyParser; // TODO: 12 bytes - 4 values (same as LVLI?)
        b"LVLD", ChanceNone, u8;
        b"LVLF", Flags, u8;
        b"LLCT", Count, u8;
        b"LVLG", UseGlobal, FormId;
        b"LLKC", KeywordChances, Vec<(FormId, u32)>; // TODO: verify
        b"LVLM", MaxCount, u8;
    ]
}