use proc::define_record;

use crate::{dev::*, structs::colors::Color4};


define_record! {
    b"KYWD", Keyword, [
        b"EDID", EditorId, ESMString;
        b"CNAM", Color, Color4;
        b"FULL", FullName, u32;
        b"DNAM", Notes, ESMString;
        b"TNAM", Type, u32;
        b"DATA", AttractionRule, u32;
        b"NNAM", DisplayName, ESMString;
    ]
}