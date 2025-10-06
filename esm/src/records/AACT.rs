use crate::dev::*;

define_record! {
    b"AACT",
    Action, [
        EditorId;
        b"CNAM", Color, Color4;
        b"TNAM", Type, u32;
        b"DNAM", Notes, ESMString;
    ]
}