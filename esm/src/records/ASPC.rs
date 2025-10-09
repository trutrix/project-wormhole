use crate::dev::*;


// Contains refs

define_record! {
    b"ASPC",
    AcousticSpace, [
        EditorId;
        ObjectBounds;
        b"WNAM", WeatherAttenuation, u8; // TODO: Find correct type
        b"XTRI", IsInterior, u8; // TODO: Find correct type
        b"RDAT", UseSoundFromRegion, FormId; // Ref
        b"SMAM", LoopingSound, FormId; // Ref
        b"BNAM", EnviromentType, u8; // TODO: Find correct type
    ]
}

// Field dump - {OBND, XTRI, WNAM, RDAT, BNAM, SNAM}