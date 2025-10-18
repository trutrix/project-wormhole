use crate::dev::*;

define_record! {
    b"CLFM",
    Color, [
        EditorId;
        FullName;
        Condition;
        b"CNAM", RGBA, Color4;
        b"FNAM", Flags, u8; // TODO: find flags
    ]
}