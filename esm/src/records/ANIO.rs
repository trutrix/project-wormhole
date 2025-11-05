use crate::dev::*;

define_record! {
    b"ANIO",
    AnimatedObject, [
        EditorId;
        ModelData;
        b"BNAM", UnloadEvent, ESMString; // String?
    ]
}

// Field dump - {BNAM, MODC, MODS, MODL, MODT}