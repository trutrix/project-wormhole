use crate::{dev::*, es::es_group::{ESGroupTrait, ESGroupHeader, interior_cell_sub_block::ESInteriorCellSubBlock}};

// ====================================================================================================

#[derive(Debug)]
pub struct ESInteriorCellBlock {
    pub header: ESGroupHeader,
    pub sub_blocks: Vec<ESInteriorCellSubBlock>
}

// ====================================================================================================

impl ESGroupTrait for ESInteriorCellBlock {
    fn group_label(&self) -> super::ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}