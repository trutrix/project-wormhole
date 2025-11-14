use crate::dev::*;

define_record! {
    b"ADDN",
    AddonNode, [
        EditorId;
        ObjectBounds;
        ModelData;


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