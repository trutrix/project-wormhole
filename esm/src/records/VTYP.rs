use crate::dev::*;

define_record2! {
    b"VTYP",
    VoiceType, [
        EditorId;
        b"DNAM", Flags, u8; // TODO: bitfields
    ]
}