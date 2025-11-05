use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"AMMO",
    Ammo, [
        EditorId;
        ObjectBounds;
        ModelData;
        PreviewTransform;
        
        FullName;
        Description;
        PickUpPutDown;
        Keyword;

        b"DATA", Data, (u32, f32); // Unknown struct
        b"DNAM", Damage, u8; // TODO create struct for this - refs a projectile or something
        b"ONAM", ShortName, ESMString; // LocalizedString?
        b"NAM1", CasingModel, FormId; // Ref?
        b"NAM2", ModelInfo, u8; // Unknown
        
    ]
}


// Field dump - {MODT, ZNAM, DESC, NAM1, MODL, KWDA, NAM2, OBND, PTRN, FULL, YNAM, ONAM, KSIZ, DATA}