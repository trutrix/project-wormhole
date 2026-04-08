use std::collections::HashMap;
use crate::{dev::*, groups::prelude::ExteriorCellSubBlock, prelude::MapContents};

// ====================================================================================================

pub type ExteriorCellBlock = Group<ExteriorCellSubBlock>;

// ====================================================================================================

pub type RawExteriorCellBlock<'esm> = Group<Group<RawCellRecord<'esm>>>;

// ====================================================================================================

impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for RawExteriorCellBlock<'esm> {
    fn insert_into_one_map(self, map: &mut HashMap<FormId, RawRecord<'esm>>) {
        for sub_block in self.data {
            for cell in sub_block.data {
                cell.insert_into_one_map(map);
            }
        }
    }
}