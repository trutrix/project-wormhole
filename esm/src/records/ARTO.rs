use crate::dev::*;

define_record! {
    b"ARTO",
    ArtObject, [
        EditorId;
        ModelData;
        ObjectBounds;
        PreviewTransform;
        Keyword;
        b"DNAM", ArtType, u32;
    ]
}

// Field dump - {MODL, MODC, PTRN, KWDA, MODS, MODT, KSIZ, OBND}