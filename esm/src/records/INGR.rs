use crate::dev::*;

// Only one record for this in all of FO4
// Probably for crafting is TES


define_record! {
    b"INGR",
    Ingredient, [
        EditorId;
        ModelData;
        ObjectBounds;
        FullName;
        PickUpPutDown;
        b"EFID", BaseEffect, FormId;
        b"ENIT", EffectData, EmptyParser;
        b"EFIT", EffectRadius, EmptyParser;
        b"DATA", Data, (FormId, f32);
        // TODO: find the rest of the fields
    ]
}