use crate::{es::es_group::{ESGroupHeader, ESGroupLabel}, traits::ParseAllocated};

#[derive(Debug)]
pub enum ESTop {
    Unhandled(ESGroupHeader)
}

impl ParseAllocated<ESGroupHeader, &[u8]> for ESTop {
    fn parse_allocated(header: ESGroupHeader, raw: &[u8]) -> Result<Self, nom_derive::nom::error::Error<&[u8]>> {
        todo!()        
    }
}