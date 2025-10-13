use crate::dev::*;

define_record! {
    b"CLFM",
    Color, [
        EditorId;
        FullName;
        b"CNAM", RGBA, Color4;
        b"FNAM", Flags, u8; // TODO: find flags
        b"CTDA", Condition, u8; // Unknown
    ]
}