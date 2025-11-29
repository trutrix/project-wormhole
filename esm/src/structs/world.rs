use crate::{dev::*, records::all::{Cell, Worldspace}, structs::cell::CellEntry};


#[derive(Debug, NomLE)]
pub struct WorldEntry {
    pub worldspace: Worldspace,
    pub children: WorldChildren
}

// ====================================================================================================

#[derive(Debug)]
pub struct WorldChildren {
    pub header: GroupHeader,
    pub cell: CellEntry,
    pub blocks: Vec<ExteriorCellBlock>
}


// Implement nom_derive::Parse
impl Parse<&[u8]> for WorldChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        // Ensure correct group type - debugging only
        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::WorldChildren(_) => { }
            _ => { panic!("WorldChildren::parse encountered wrong group type: {:?}", header.label) }
        }
    
        // Parse the Cell record inside the WorldChildren group
        let (raw, cell) = CellEntry::parse(raw)?;
        let (raw, blocks) = many0(ExteriorCellBlock::parse)(raw)?;

        #[cfg(debug_assertions)]
        if raw.len() > 0 {
            panic!("WorldChildren::parse found unexpected remaining data after parsing all ExteriorCellBlock items: {} bytes left.", raw.len());
        }


        Ok((i, Self { header, cell, blocks }) )
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct ExteriorCellBlock {
    pub header: GroupHeader,
    pub sub_blocks: Vec<ExteriorCellSubBlock>
}

impl Parse<&[u8]> for ExteriorCellBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::ExteriorCellBlock(_) => {
                let (raw, sub_blocks) = many0(ExteriorCellSubBlock::parse)(raw)?;

                #[cfg(debug_assertions)]
                if raw.len() > 0 {
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

#[derive(Debug)]
pub struct ExteriorCellSubBlock {
    pub header: GroupHeader,
    pub cells: Vec<CellEntry>
}

impl Parse<&[u8]> for ExteriorCellSubBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::ExteriorCellSubBlock(_) => {
                let (raw, cells) = many0(CellEntry::parse)(raw)?;

                #[cfg(debug_assertions)]
                if raw.len() > 0 {
                    let (_, next_id) = FourCC::parse(raw)?;
                    panic!("ExteriorCellSubBlock::parse found unexpected remaining data after parsing all CellEntry items: {} bytes left. NextId: {:?}", raw.len(), next_id);
                }

                Ok((i, Self { header, cells }) )
            }
            _ => { panic!("ExteriorCellSubBlock::parse encountered wrong group type: {:?}", header.label) }
        }

    }
}