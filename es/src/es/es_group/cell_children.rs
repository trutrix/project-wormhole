use crate::{dev::*, es::{es_group::ESGroupHeader, es_object::ESObject}, traits::ParseAllocated};

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

impl ESObject for ESCellChildren {
    fn object_count(&self) -> &usize {
        todo!()
    }
    fn object_size(&self) -> &u32 { &self.header.size }
    fn is_group(&self) -> bool { true }
    fn try_get_form_id(&self) -> Option<&FormId> { None }
}