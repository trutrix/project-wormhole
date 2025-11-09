use crate::dev::*;

define_record! {
    b"VTYP",
    VoiceType, [
        EditorId;
        b"DNAM", Flags, u8; // TODO: bitfields
    ]
}