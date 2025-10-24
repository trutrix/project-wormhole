use crate::dev::*;

// Only one record for this in all of FO4
// Probably for crafting is TES


define_record! {
    b"INGR",
    Ingredient, [
        EditorId;
        AllModelData;
        ObjectBounds;
        FullName;
        b"EFID", BaseEffect, FormId;
        b"ENIT", EffectData, EmptyParser;
        b"EFIT", EffectRadius, EmptyParser;
        b"DATA", Data, (FormId, f32);
        b"YNAM", SoundPickUp, FormId;
        b"ZNAM", SoundPutDown, FormId; // assumed - not present
        // TODO: find the rest of the fields
    ]
}