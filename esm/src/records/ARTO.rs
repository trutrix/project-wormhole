use crate::dev::*;

define_record3! {
    "iden": b"ARTO";
    "name": ArtObject;
    "fields": [
        EditorId;
        ModelData;
        ObjectBounds;
        PreviewTransform;
        Keyword;
        b"DNAM", ArtType, u32;
    ]
}

// Field dump - {MODL, MODC, PTRN, KWDA, MODS, MODT, KSIZ, OBND}