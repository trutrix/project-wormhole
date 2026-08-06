use crate::{dev::*, es::es_group::{ESGroupTrait, ESGroupHeader}};

// ====================================================================================================

#[derive(Debug)]
pub struct ESExteriorCellSubBlock {
    pub header: ESGroupHeader
}

// ====================================================================================================

impl ESGroupTrait for ESExteriorCellSubBlock {
    fn group_label(&self) -> super::ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}