use crate::{dev::*, records::all::CellEntry};

// ====================================================================================================

#[derive(Debug)]
pub struct ExteriorCellSubBlock {
    pub header: GroupHeader,
    pub cells: Vec<CellEntry>
}

// ====================================================================================================

impl Parse<&[u8]> for ExteriorCellSubBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::ExteriorCellSubBlock(_) => {
                let (raw, cells) = many0(CellEntry::parse)(raw)?;

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