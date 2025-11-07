use crate::dev::*;

define_record! {
    b"SCSN",
    AudioCategorySnapshot, [
        EditorId;
        b"PNAM", Priority, u16;
        b"CNAM", SoundCategoryMultiplier, (FormId, f32);
    ]
}