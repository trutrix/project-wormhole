use crate::dev::*;

define_record3! {
    "iden": b"STAG";
    "name": SoundTag;
    "fields": [
        EditorId;
        b"TNAM", Sounds, (FormId, ESMString);
    ]
}