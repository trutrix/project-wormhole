use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"ALCH",
    Alchemy, [
        EditorId;
        ObjectBounds;
        PreviewTransform;
        FullName;
        Description;
        Condition;
        AllModelData;
        b"KSIZ", KeywordSize, u32;
        b"KWDA", Keywords, u32; // Make compound field work
        b"YNAM", Value, u32;
        b"CUSD", CustomData, u8; // TODO: Find actual type
        b"DATA", Data, u8; // TODO: Find actual type
        b"ENIT", EffectData, u8; // TODO: Find actual type
        b"DNAM", Dynamic, u8; // TODO: Find actual type
        b"EFID", Effect, u32; // TODO: Find actual type
        b"EFIT", EffectData2, u8; // TODO: Find actual type
        b"DEST", DecalData, u8; // TODO: Find actual type
        b"ZNAM", ZNAM, u8; // TODO: Find actual type
    ]
}