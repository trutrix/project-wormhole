use crate::{dev::*, es::es_group::{ESGroupTrait, ESGroupHeader}};

// ====================================================================================================

#[derive(Debug)]
pub struct ESCellPersistentChildren {
    pub header: ESGroupHeader
}

// ====================================================================================================

impl ESGroupTrait for ESCellPersistentChildren {
    fn group_label(&self) -> super::ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}