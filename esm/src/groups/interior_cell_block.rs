use crate::{dev::*, groups::prelude::{InteriorCellSubBlock, RawInteriorCellSubBlock}};


#[derive(Debug, NomLE)]
pub struct InteriorCellBlock(pub Group<InteriorCellSubBlock>);


pub struct RawInteriorCellBlock<'esm> {
    pub header: GroupHeader,
    pub sub_blocks: Vec<RawInteriorCellSubBlock<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawInteriorCellBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, sub_blocks) = many0(RawInteriorCellSubBlock::parse)(data)?;
        Ok((i, Self { header, sub_blocks}))
    }
}

impl std::fmt::Debug for RawInteriorCellBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} bytes", self.header, self.sub_blocks.len())
    }
}