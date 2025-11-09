use crate::dev::*;

define_record! {
    b"STAG",
    SoundTag, [
        EditorId;
        b"TNAM", Sounds, (FormId, ESMString);
    ]
}