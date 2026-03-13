use crate::dev::*;

define_record3! {
    "iden": b"VTYP";
    "name": VoiceType;
    "fields": [
        EditorId;
        b"DNAM", Flags, u8; // TODO: bitfields
    ]
}