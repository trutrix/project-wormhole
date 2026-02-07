use std::panic;

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

pub struct ActionRecord {
    pub header: RecordHeader,
    pub fields: Vec<ActionField>,
}

impl Identifier for ActionRecord {
    fn get_identifier(&self) -> &[u8;4] {
        b"AACT"
    }
}

impl EDID for Action {
    fn get_editor_id(&self) -> &ESMString {
        for field in &self.fields {
            if let ActionField::EditorId(editor_id) = field {
                return editor_id;
            }
        }
        panic!("EditorId field not found for Action record with FormId: {}", self.get_form_id());
    }
}

impl<T: EDID> Identifier for T {
    fn get_identifier(&self) -> &[u8;4] {
        b"EDID"
    }
}


pub trait Identifier {
    fn get_identifier(&self) -> &[u8;4];
}