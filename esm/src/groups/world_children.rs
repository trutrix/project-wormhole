use crate::{dev::*, groups::prelude::ExteriorCellBlock, records::all::CellEntry};




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
        if !raw.is_empty() {
            panic!("WorldChildren::parse found unexpected remaining data after parsing all ExteriorCellBlock items: {} bytes left.", raw.len());
        }


        Ok((i, Self { header, cell, blocks }) )
    }
}

// ====================================================================================================

pub struct RawWorldChildren<'esm> {
    pub header: GroupHeader,
    pub cell: RawCellRecord<'esm>,
    pub blocks: Vec<RawExteriorCellBlock<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawWorldChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;

        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::WorldChildren(_) => { }
            _ => { panic!("RawWorldChildren::parse encountered wrong group type: {:?}", header.label) }
        }

        let (raw, cell) = RawCellRecord::parse(raw)?;
        // println!("Parsed world children cell: {:?}, {} bytes", cell.cell.header.form_id, raw.len());

        #[cfg(debug_assertions)]
        if cell.cell.header.iden.0 != *b"CELL" {
            panic!("WorldChildren tried to parse {:?} as CELL", cell.cell.header.form_id);
        }

        let (raw, blocks) = many0(RawExteriorCellBlock::parse)(raw)?;

        #[cfg(debug_assertions)]
        if !raw.is_empty() {
            panic!("Failed to consume RawWorldChildren")
        }

        Ok((i, Self { header, cell, blocks }))
    }
}


impl std::fmt::Debug for RawWorldChildren<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawWorldChildren {{ header: {:?}, cell: {:?}, blocks: {} }}", self.header, self.cell.cell.header.form_id, self.blocks.len())
    }
}