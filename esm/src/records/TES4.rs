use crate::dev::*;

// ====================================================================================================

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
        b"MAST", MasterFile, ESMString;
        b"DATA", MasterFileSize, u64;
    ]
}

// ====================================================================================================

#[derive(Debug, NomLE, PartialEq)]
pub struct FileHeaderMetadata {
    pub version: f32,
    pub object_count: u32,
    pub next_object_id: FormId
}

// ====================================================================================================

#[derive(NomLE, PartialEq)]
pub struct FileHeaderTransientItems {
    pub type_: u32,
    pub ids: Vec<FormId>
}


// Custom debug to avoid printing an enormous list of FormIDs
impl std::fmt::Debug for FileHeaderTransientItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHeaderTransientItems {{ type_: {}, ids: {} items }}", self.type_, self.ids.len())
    }
}

// ====================================================================================================

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

            if !matched { return false; }
        }
        matched
    }
}

// ====================================================================================================

impl FileHeader {
    pub fn get_object_count(&self) -> Option<&u32> {
        
        for field in &self.data {
            if let FileHeaderField::Metadata(md) = field {
                return Some(&md.object_count)
            }
        }
        None
    }

    pub fn get_master_file(&self) -> Option<&ESMString> {
        for field in &self.data {
            if let FileHeaderField::MasterFile(md) = field {
                return Some(md)
            }
        }
        None
    }
}