use crate::dev::*;

define_record3! {
    "iden": b"ANIO";
    "name": AnimatedObject;
    "fields": [
        EditorId;
        ModelData;
        b"BNAM", UnloadEvent, ESMString; // String?
    ]
}

// Field dump - {BNAM, MODC, MODS, MODL, MODT}