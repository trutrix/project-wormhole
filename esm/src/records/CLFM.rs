use crate::dev::*;

define_record3! {
    "iden": b"CLFM";
    "name": Color;
    "fields": [
        EditorId;
        FullName;
        Condition;
        b"CNAM", RGBA, Color4;
        b"FNAM", Flags, u8; // TODO: find flags
    ]
}