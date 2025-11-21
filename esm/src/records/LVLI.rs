use crate::dev::*;

define_record2! {
    b"LVLI",
    LeveledItem, [
        EditorId;
        ObjectBounds;
        b"COED", ExtraData, EmptyParser; // TODO: Size 12, may require special parsing
        b"LVLM", MaxCount, u8;
        b"LVLD", ChanceNone, u8;
        b"LVLF", Flags, u8;
        b"LLCT", Count, u8;

        b"ONAM", OverrideName, LocalizedString;

        b"LVSG", EpicLootChance, FormId; // GLOB
        b"LVLO", BaseData, EmptyParser; // TODO: 12 bytes - 4 values
        b"LLKC", KeywordChances, Vec<(FormId, u32)>; // TODO: verify
        b"LVLG", UseGlobal, FormId;
    ]
}