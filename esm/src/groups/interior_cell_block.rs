use std::collections::HashMap;

use crate::{dev::*, groups::prelude::{InteriorCellSubBlock, RawInteriorCellSubBlock}, prelude::MapContents};

// ====================================================================================================

#[derive(Debug)]
pub struct InteriorCellBlock(pub Group<InteriorCellSubBlock>);

impl Parse<&[u8]> for InteriorCellBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        //println!("  Parsing interior cell block...");
        let (i, items) = Group::<InteriorCellSubBlock>::parse(i)?;
        Ok((i, Self(items)))
    }
}

// ====================================================================================================

pub struct RawInteriorCellBlock<'esm> {
    pub header: GroupHeader,
    pub sub_blocks: Vec<RawInteriorCellSubBlock<'esm>>
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for RawInteriorCellBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, sub_blocks) = many0(RawInteriorCellSubBlock::parse)(data)?;
        Ok((i, Self { header, sub_blocks}))
    }
}

// ====================================================================================================

impl std::fmt::Debug for RawInteriorCellBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} bytes", self.header, self.sub_blocks.len())
    }
}

// ====================================================================================================

impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for RawInteriorCellBlock<'esm> {
    fn insert_into_one_map(self, map: &mut HashMap<FormId, RawRecord<'esm>>) {
        for sub_block in self.sub_blocks {
            for cell in sub_block.data {
                cell.insert_into_one_map(map);
            }
        }
    }

    fn insert_into_two_maps(self, map1: &mut HashMap<FormId, RawRecord<'esm>>, map2: &mut HashMap<FormId, RawRecord<'esm>>) {
        for sub_block in self.sub_blocks {
            for cell in sub_block.data {
                cell.insert_into_two_maps(map1, map2);
            }
        }
    }
}