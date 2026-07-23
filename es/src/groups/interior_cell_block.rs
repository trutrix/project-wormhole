use std::collections::HashMap;

use crate::{dev::*, groups::prelude::{InteriorCellSubBlock, RawInteriorCellSubBlock}, prelude::MapContents};

// ====================================================================================================

pub type InteriorCellBlock = Group<InteriorCellSubBlock>;

// ====================================================================================================

pub type RawInteriorCellBlock<'esm> = Group<RawInteriorCellSubBlock<'esm>>;

// ====================================================================================================

// Works for Exterior as well because of aliases
impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for RawInteriorCellBlock<'esm> {
    fn insert_into_one_map(self, map: &mut HashMap<FormId, RawRecord<'esm>>) {
        for sub_block in self.data {
            for cell in sub_block.data {
                cell.insert_into_one_map(map);
            }
        }
    }
}