use crate::{dev::*, records::all::Cell};

#[derive(Debug)]
pub struct CellEntry {
    pub cell: Cell,
    pub children: Option<CellChildren>
}

impl Parse<&[u8]> for CellEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        let (i, cell) = Cell::parse(i)?;
        let (_, header) = GroupHeader::parse(i)?;

        match header.label {
            GroupLabel::CellChildren(_) => {
                let (i, children) = CellChildren::parse(i)?;
                Ok((i, Self { cell, children: Some(children) }) )
            }
            _ => {
                Ok((i, Self { cell, children: None }) )
            }
        }
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct CellChildren {
    pub header: GroupHeader,
}


// Implement nom_derive::Parse
impl Parse<&[u8]> for CellChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        // Ensure correct group type - debugging only
        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::CellChildren(_) => { }
            _ => { panic!("CellChildren::parse encountered wrong group type: {:?}", header.label) }
        }

        Ok((i, Self { header }) )
    }
}