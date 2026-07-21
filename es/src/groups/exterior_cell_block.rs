use crate::{dev::*, groups::prelude::{ExteriorCellSubBlock, RawExteriorCellSubBlock}};

// ====================================================================================================

pub type ExteriorCellBlock = GroupOld<ExteriorCellSubBlock>;

// ====================================================================================================

pub type RawExteriorCellBlock<'esm> = GroupOld<RawExteriorCellSubBlock<'esm>>;

// ====================================================================================================


// Type aliases make this not work
// impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for RawExteriorCellBlock<'esm> {
//     fn insert_into_one_map(self, map: &mut HashMap<FormId, RawRecord<'esm>>) {
//         for sub_block in self.data {
//             for cell in sub_block.data {
//                 cell.insert_into_one_map(map);
//             }
//         }
//     }
// }