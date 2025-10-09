use crate::dev::*;

define_record! {
    b"AACT",
    Action, [
        EditorId;
        b"CNAM", Color, Color4;
        b"TNAM", Type, u32; // TODO: Find correct type
        b"DNAM", Notes, ESMString;
        b"FULL", FullName, u8; // TODO: Find correct type
        b"DATA", AttractionRule, u8; // TODO: Find correct type
    ]
}