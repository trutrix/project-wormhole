use crate::{dev::*, groups::prelude::{ExteriorCellSubBlock, RawExteriorCellSubBlock}};

// ====================================================================================================

pub type ExteriorCellBlock = Group<ExteriorCellSubBlock>;

// ====================================================================================================

pub type RawExteriorCellBlock<'es> = Group<RawExteriorCellSubBlock<'es>>;

// ====================================================================================================


// Type aliases make this not work
// impl<'es> MapContents<HashMap<FormId, RawRecord<'es>>> for RawExteriorCellBlock<'es> {
//     fn insert_into_one_map(self, map: &mut HashMap<FormId, RawRecord<'es>>) {
//         for sub_block in self.data {
//             for cell in sub_block.data {
//                 cell.insert_into_one_map(map);
//             }
//         }
//     }
// }