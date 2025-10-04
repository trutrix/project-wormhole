use crate::dev::*;

define_record! {
    b"ADDN",
    AddonNode, [
        b"EDID", EditorId, ESMString;
        b"OBND", ObjectBounds, ObjectBounds;
        b"MODT", ModelTexture, ModelTexture;
        b"MODL", ModelPath, ModelPath;
        b"MODC", ModelColorMap, ModelColorMap;
        b"MODS", ModelMaterialSwap, ModelMaterialSwap;
        b"MODF", ModelFlags, ModelFlags;
        b"DATA", NodeIndex, u32; // TODO: Verify type
        b"SNAM", Sound, FormId;
        b"LNAM", Light, FormId;
        b"DNAM", AddonData, u8; // TODO: Define data structure
    ]
}

// Field dump
// "ADDN": [
//     "OBND",
//     "EDID",
//     "DNAM",
//     "SNAM",
//     "LNAM",
//     "DATA",
//     "MODL",
//     "MODT"
//   ]