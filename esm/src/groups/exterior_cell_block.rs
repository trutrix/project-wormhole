use std::collections::HashMap;

use crate::{dev::*, groups::prelude::ExteriorCellSubBlock, prelude::MapContents};

// ====================================================================================================

/// Top level World cells
#[derive(Debug)]
pub struct ExteriorCellBlock {
    pub header: GroupHeader,
    pub sub_blocks: Vec<ExteriorCellSubBlock>
}

// ====================================================================================================

impl Parse<&[u8]> for ExteriorCellBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::ExteriorCellBlock(_) => {
                let (raw, sub_blocks) = many0(ExteriorCellSubBlock::parse)(raw)?;

                #[cfg(debug_assertions)]
                if !raw.is_empty() {
                    let (_, next_id) = FourCC::parse(raw)?;
                    panic!("ExteriorCellBlock::parse found unexpected remaining data after parsing all ExteriorCellSubBlock items: {} bytes left. NextId: {:?}", raw.len(), next_id);
                }


                Ok((i, Self { header, sub_blocks }) )
            }
            _ => { panic!("ExteriorCellBlock::parse encountered wrong group type: {:?}", header.label) }
        }

    }
}

// ====================================================================================================

impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for RawExteriorCellBlock<'esm> {
    fn insert_into_one_map(self, map: &mut HashMap<FormId, RawRecord<'esm>>) {
        for sub_block in self.sub_blocks {
            for cell in sub_block.cells {
                cell.insert_into_one_map(map);
            }
        }
    }

    fn insert_into_two_maps(self, map1: &mut HashMap<FormId, RawRecord<'esm>>, map2: &mut HashMap<FormId, RawRecord<'esm>>) {
        for sub_block in self.sub_blocks {
            for cell in sub_block.cells {
                cell.insert_into_two_maps(map1, map2);
            }
        }
    }
}