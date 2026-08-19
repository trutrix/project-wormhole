use crate::{dev::*, es::{es_group::{ESGroupTrait, ESGroupHeader}, es_object::ESObjectTrait}, traits::ParseAllocated};

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

// ====================================================================================================

impl ESGroupTrait for ESCellChildren {
    fn group_label(&self) -> super::ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}