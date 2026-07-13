use crate::{es::{es_group::{ESGroupHeader, ESGroupLabel}, es_object::{ESHeader, ESObjectTraits}, es_record::ESRecordHeader}, records::AACT, traits::ParseAllocated};

#[derive(Debug)]
pub enum ESTop {
    Unhandled(ESGroupHeader)
}

// ====================================================================================================

impl ParseAllocated<ESGroupHeader, &[u8]> for ESTop {
    fn parse_allocated(header: ESGroupHeader, raw: &[u8]) -> Result<Self, nom_derive::nom::error::Error<&[u8]>> {
        match &header.label_value {
            b"AACT" => {
                todo!()
            }  
            _ => {
                todo!()
            }
        }
    }
}

// ====================================================================================================

impl ESObjectTraits for ESTop {
    fn object_size(&self) -> &u32 {
        match self {
            ESTop::Unhandled(esgroup_header) => &esgroup_header.size,
        }
    }
}

// ====================================================================================================

impl ESHeader<ESGroupHeader> for ESTop {
    fn header(&self) -> &ESGroupHeader {
        match self {
            ESTop::Unhandled(g) => g,
        }
    }
}