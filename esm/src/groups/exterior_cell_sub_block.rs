use crate::{dev::*, records::all::Cell};

// ====================================================================================================

#[derive(Debug)]
pub struct ExteriorCellSubBlock {
    pub header: GroupHeader,
    pub cells: Vec<Cell>
}

// ====================================================================================================

impl Parse<&[u8]> for ExteriorCellSubBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::ExteriorCellSubBlock(_) => {
                let (raw, cells) = many0(Cell::parse)(raw)?;

                #[cfg(debug_assertions)]
                if !raw.is_empty() {
                    let (_, next_id) = FourCC::parse(raw)?;
                    panic!("ExteriorCellSubBlock::parse found unexpected remaining data after parsing all CellEntry items: {} bytes left. NextId: {:?}", raw.len(), next_id);
                }

                Ok((i, Self { header, cells }) )
            }
            _ => { panic!("ExteriorCellSubBlock::parse encountered wrong group type: {:?}", header.label) }
        }

    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawExteriorCellSubBlock<'esm> {
    pub header: GroupHeader,
    pub cells: Vec<RawCellRecord<'esm>>
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for RawExteriorCellSubBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {

        let (i, (header, raw)) = alloc_group(i)?;

        if let GroupLabel::ExteriorCellSubBlock(_cell_coords) = header.label {
            let (raw, cells) = many0(RawCellRecord::parse)(raw)?;

            #[cfg(debug_assertions)]
            if !raw.is_empty() {
                panic!("Failed to consume RawExteriorCellSubBlock");
            }

            Ok((i, Self { header, cells }))
        } else {
            panic!("RawExteriorCellSubBlock::parse encountered wrong group type: {:?}", header.label)
        }
    }
}