use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"AMMO",
    Ammo, [
        b"EDID", EditorId, ESMString;
        b"OBND", ObjectBounds, ObjectBounds;

        b"PTRN", PreviewTransform, FormId;
        b"FULL", FullName, ESMString; // LocalizedString?
        b"DESC", Description, ESMString; // LocalizedString?
        

        b"MODL", ModelPath, ModelPath;
        b"MODT", ModelTexture, ModelTexture;
        b"MODC", ModelColorMap, ModelColorMap;
        b"MODS", ModelMaterialSwap, ModelMaterialSwap;
        b"MODF", ModelFlags, ModelFlags;



    ]
}


// Field dump - {MODT, ZNAM, DESC, NAM1, MODL, KWDA, NAM2, OBND, PTRN, FULL, YNAM, ONAM, KSIZ, DATA}