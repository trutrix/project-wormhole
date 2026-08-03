use crate::{dev::*, es::es_group::{ESGroup, ESGroupHeader}};

// ====================================================================================================

#[derive(Debug)]
pub struct ESTopicChildren {
    pub header: ESGroupHeader
}

// ====================================================================================================

impl ESGroup for ESTopicChildren {
    fn group_label(&self) -> super::ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}