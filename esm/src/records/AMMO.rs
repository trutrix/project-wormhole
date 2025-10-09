use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"AMMO",
    Ammo, [
        EditorId;
        ObjectBounds;
        AllModelData;

        b"PTRN", PreviewTransform, FormId; // Ref to TRNS
        b"FULL", FullName, ESMString; // LocalizedString?
        b"DESC", Description, ESMString; // LocalizedString?

        // Unsure if KSIZ defines the actual size of KWDA
        // Why wouldnt you be able to just divide the size of KWDA by 4?
        // Similar to XXXX field in cells?
        b"KSIZ", KeywordSize, u8; // Number of keywords
        b"KWDA", Keywords, Vec<FormId>; // Refs to KYWDs

        b"DATA", Data, (u32, f32); // Unknown struct
        b"DNAM", Damage, u8; // TODO create struct for this - refs a projectile or something
        b"ONAM", ShortName, ESMString; // LocalizedString?
        b"NAM1", CasingModel, FormId; // Ref?
        b"NAM2", ModelInfo, u8; // Unknown
        b"ZNAM", SoundPutDown, FormId; // Ref to SNDR
        b"YNAM", SoundPickUp, FormId; // Ref to SNDR
    ]
}


// Field dump - {MODT, ZNAM, DESC, NAM1, MODL, KWDA, NAM2, OBND, PTRN, FULL, YNAM, ONAM, KSIZ, DATA}