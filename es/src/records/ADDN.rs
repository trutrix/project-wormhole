use crate::dev::*;

define_record3! {
    "iden": b"ADDN";
    "name": AddonNode;
    "fields": [
        EditorId;
        ObjectBounds;
        ModelData;
        b"DATA", NodeIndex, u32; // TODO: Verify type
        b"SNAM", Sound, FormId;
        b"LNAM", Light, FormId;
        b"DNAM", AddonData, u8; // TODO: Define data structure
    ]
}