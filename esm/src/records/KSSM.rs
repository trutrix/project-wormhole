use crate::dev::*;

define_record! {
    b"KSSM",
    KeywordSoundMapping, [
        EditorId;
        b"DNAM", PrimaryDesc, FormId;
        b"RNAM", SoundPair, (u32, FormId); // Type / Sound
        b"TNAM", VatsDesc, FormId;
        b"VNAM", VatsThreshold, f32;
        b"KNAM", Keywords, Vec<FormId>;
        b"ENAM", ExteriorTailSound, FormId;
    ]
}