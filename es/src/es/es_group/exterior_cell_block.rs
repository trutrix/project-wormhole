use crate::{dev::*, es::es_group::{ESGroup, ESGroupHeader, exterior_cell_sub_block::ESExteriorCellSubBlock}};

// ====================================================================================================

#[derive(Debug)]
pub struct ESExteriorCellBlock {
    pub header: ESGroupHeader,
    pub sub_blocks: Vec<ESExteriorCellSubBlock>
}

// ====================================================================================================

impl ESGroup for ESExteriorCellBlock {
    fn group_label(&self) -> super::ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}