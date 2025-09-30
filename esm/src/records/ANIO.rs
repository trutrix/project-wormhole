use crate::dev::*;

define_record! {
    b"ANIO",
    AnimatedObject, [
        b"EDID", EditorId, ESMString;
        b"MODL", ModelMesh, ESMString;
        b"MODT", ModelTexture, ESMString;
        
    ]
}

// Field dump - {BNAM, MODC, MODS, MODL, MODT}