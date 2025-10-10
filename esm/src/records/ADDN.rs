use crate::dev::*;

define_record! {
    b"ADDN",
    AddonNode, [
        EditorId;
        ObjectBounds;
        
        b"DATA", NodeIndex, u32; // TODO: Verify type
        b"SNAM", Sound, FormId;
        b"LNAM", Light, FormId;
        b"DNAM", AddonData, u8; // TODO: Define data structure
        
        b"MODL", ModelPath, ModelPath;
        b"MODT", ModelTexture, ModelTexture;
        b"MODC", ModelColorMap, ModelColorMap;
        b"MODS", ModelMaterialSwap, ModelMaterialSwap;
        b"MODF", ModelFlags, ModelFlags;
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