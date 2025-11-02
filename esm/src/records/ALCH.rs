use crate::{dev::*, structs::geometry::ObjectBounds};


// Also known as ingestible

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
        Keyword;
        PickUpPutDown;
        b"CUSD", CustomData, u8; // TODO: Find actual type
        b"DATA", Data, u8; // TODO: Find actual type
        b"ENIT", EffectData, u8; // TODO: Find actual type
        b"DNAM", Dynamic, u8; // TODO: Find actual type
        b"EFID", Effect, u32; // TODO: Find actual type
        b"EFIT", EffectData2, u8; // TODO: Find actual type
        b"DEST", DecalData, u8; // TODO: Find actual type
    ]
}