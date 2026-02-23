use project_wormhole_shared::prelude::FourCCTrait;

use crate::dev::*;


define_record3! {
    "iden": b"TES4";
    "name": FileHeader;
    "fields": [
        b"INCC", InteriorCellCount, u32;
        b"INTV", AvailableTags, u32;
        b"HEDR", Metadata, FileHeaderMetadata;
        b"CNAM", Author, ESMString;
        b"SNAM", TextDescription, ESMString; // TODO: Check if current
        b"ONAM", OverriddenForms, Vec<FormId>;
        b"TNAM", TransientItems, FileHeaderTransientItems;
    ]
}

#[derive(Debug, NomLE, PartialEq)]
pub struct FileHeaderMetadata {
    pub version: f32,
    pub object_count: u32,
    pub next_object_id: FormId
}

#[derive(NomLE, PartialEq)]
pub struct FileHeaderTransientItems {
    pub type_: u32,
    pub ids: Vec<FormId>
}

impl std::fmt::Debug for FileHeaderTransientItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHeaderTransientItems {{ type_: {}, ids: {} items }}", self.type_, self.ids.len())
    }
}


impl PartialEq for FileHeader {
    fn eq(&self, other: &Self) -> bool {
        if self.header != other.header { return false }
        else if self.data.len() != other.data.len() { return false }
        let mut matched = false;
        
        for self_item in &self.data {
            matched = false;

            for other_item in &other.data {
                if self_item == other_item { matched = true; break; }
            }

            if matched == false { return false; }
        }
        true
    }
}