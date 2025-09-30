use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"ALCH",
    Alchemy, [
        b"EDID", EditorId, ESMString;
        b"OBND", ObjectBounds, ObjectBounds;
        b"PTRN", PTRN, ESMString; // TODO: Find actual type
        b"FULL", FullName, LocalizedString;
        b"KSIZ", KeywordSize, u32;
        b"KWDA", Keywords, u32; // Make compound field work
        b"MODL", Model, ModelPath;
        b"MODT", ModelTexture, ModelTexture; // TODO: Find actual type
        b"YNAM", Value, u32;
        b"DESC", Description, u32;
        b"CUSD", CustomData, u8; // TODO: Find actual type
        b"DATA", Data, u8; // TODO: Find actual type
        b"ENIT", EffectData, u8; // TODO: Find actual type
        b"DNAM", Dynamic, u8; // TODO: Find actual type
        b"EFID", Effect, u32; // TODO: Find actual type
        b"EFIT", EffectData2, u8; // TODO: Find actual type
        b"CTDA", Condition, u8; // TODO: Find actual types
        b"MODS", ModelSwap, u8; // TODO: Find actual type
        b"DEST", DecalData, u8; // TODO: Find actual type
        b"ZNAM", ZNAM, u8; // TODO: Find actual type
    ]
}