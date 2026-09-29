use std::collections::HashMap;

use crate::{dev::*, groups::prelude::{InteriorCellSubBlock, RawInteriorCellSubBlock}, prelude::MapContents};

// ====================================================================================================

pub type InteriorCellBlock = Group<InteriorCellSubBlock>;

// ====================================================================================================

pub type RawInteriorCellBlock<'es> = Group<RawInteriorCellSubBlock<'es>>;

// ====================================================================================================

// Works for Exterior as well because of aliases
impl<'es> MapContents<HashMap<FormId, RawRecord<'es>>> for RawInteriorCellBlock<'es> {
    fn insert_into_one_map(self, map: &mut HashMap<FormId, RawRecord<'es>>) {
        for sub_block in self.data {
            for cell in sub_block.data {
                cell.insert_into_one_map(map);
            }
        }
    }
}