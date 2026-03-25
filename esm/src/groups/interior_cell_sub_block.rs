use crate::{dev::*, records::all::Cell};

// ====================================================================================================

#[derive(Debug)]
pub struct InteriorCellSubBlock(pub Group<Cell>);

impl Parse<&[u8]> for InteriorCellSubBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, items) = Group::<Cell>::parse(i)?;
        Ok((i, Self(items)))
    }
}

// ====================================================================================================

pub struct RawInteriorCellSubBlock<'esm> {
    pub header: GroupHeader,
    pub data: Vec<RawCellRecord<'esm>>
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for RawInteriorCellSubBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        // println!("  Parsing InteriorCellSubBlock: {:?}, {} bytes", header, data.len());
        let (_, records) = many0(RawCellRecord::parse)(data)?;
        // println!("  Finished parsing InteriorCellSubBlock: {:?}, {} records", header, records.len());
        Ok((i, Self { header, data: records }))
    }
}

// ====================================================================================================

impl std::fmt::Display for RawInteriorCellSubBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} records", self.header, self.data.len())
    }
}

// ====================================================================================================

impl std::fmt::Debug for RawInteriorCellSubBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} records", self.header, self.data.len())
    }
}