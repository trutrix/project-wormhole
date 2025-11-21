use crate::dev::*;

define_record2! {
    b"STAG",
    SoundTag, [
        EditorId;
        b"TNAM", Sounds, (FormId, ESMString);
    ]
}