use crate::{dev::*, traits::RecordParser};


define_record2! {
    b"TES4",
    FileHeader, [
        b"INCC", InteriorCellCount, u32;
        b"INTV", AvailableTags, u32;
        b"HEDR", Metadata, FileHeaderMetadata;
        b"CNAM", Author, ESMString;
        b"SNAM", TextDescription, ESMString; // TODO: Check if current
        b"ONAM", OverriddenForms, Vec<FormId>;
        b"TNAM", TransientItems, FileHeaderTransientItems;
    ]
}


impl RecordParser<FileHeaderField> for FileHeader {}

#[derive(Debug, NomLE)]
pub struct FileHeaderMetadata {
    pub version: f32,
    pub object_count: u32,
    pub next_object_id: FormId
}

#[derive(NomLE)]
pub struct FileHeaderTransientItems {
    pub type_: u32,
    pub ids: Vec<FormId>
}

impl std::fmt::Debug for FileHeaderTransientItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHeaderTransientItems {{ type_: {}, ids: {} items }}", self.type_, self.ids.len())
    }
}