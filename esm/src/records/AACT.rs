use crate::{dev::*, records::all::KeywordType};

define_record3! {
    "iden": b"AACT";
    "name": Action;
    "fields": [
        EditorId;
        FullName;
        b"CNAM", Color, Color4;
        b"TNAM", Type, KeywordType; // TODO: Not sure if this is an override for the actual keyword record
        b"DNAM", Notes, ESMString;
        b"DATA", AttractionRule, [b"AORU"];
    ];
    "flags": [
        0x00080000, Restricted;
    ];
}