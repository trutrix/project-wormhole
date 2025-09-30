use crate::dev::*;

define_record! {
    b"AECH",
    AudioEffectChain, [
        b"EDID", EditorId, ESMString;
        b"KNAM", Keyword, u32;
        b"DNAM", DNAM, ESMString;
    ]
}