use crate::dev::*;

define_record3! {
    "iden": b"SCSN";
    "name": AudioCategorySnapshot;
    "fields": [
        EditorId;
        b"PNAM", Priority, u16;
        b"CNAM", SoundCategoryMultiplier, (FormId, f32);
    ]
}