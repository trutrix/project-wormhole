use proc::define_record2;

use crate::{dev::*, records::all::KeywordType};

define_record2! {
    b"AACT",
    Action, 
    // Fields - 'Common Name' or 'Iden, Name, Type'
    [
        EditorId;
        FullName;
        b"CNAM", Color, Color4;
        b"TNAM", Type, KeywordType; // TODO: Not sure if this is an override for the actual keyword record
        b"DNAM", Notes, ESMString;
        b"DATA", AttractionRule, [b"AORU"];
    ],
    // Flags - Position / Name
    [
        0x00080000, Restricted;
    ]
}