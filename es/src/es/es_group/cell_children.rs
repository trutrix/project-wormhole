use crate::{dev::*, es::es_group::ESGroupHeader, traits::ParseAllocated};

// ====================================================================================================

#[derive(Debug)]
pub struct ESCellChildren {
    pub header: ESGroupHeader
}

// ====================================================================================================

impl ParseAllocated<ESGroupHeader, &[u8]> for ESCellChildren {
    fn parse_allocated(header: ESGroupHeader, raw: &[u8]) -> Result<Self, nom::error::Error<&[u8]>> {
        todo!()
    }
}