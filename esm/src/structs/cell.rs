use crate::{dev::*, groups::prelude::CellChildren, records::all::Cell};

#[derive(Debug)]
pub struct CellEntry {
    pub cell: Cell,
    pub children: Option<CellChildren>
}

impl Parse<&[u8]> for CellEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse the Cell record
        let (i, cell) = Cell::parse(i)?;

        // Check if buffer is consumed (usually at the end of groups)
        if i.len() < 4 {
            return Ok((i, Self { cell, children: None }) )
        }  

        // Peek at the next FourCC to see if it's a GRUP
        let  (_, next_id) = FourCC::parse(i)?;

        // If next iden is not GRUP, there are no children 
        // The groups themselves have pointers to parents, so in theory they could be out of order
        // In practice, they always seem to follow the Cell record directly.

        // Check if next item is a group, if not return with no children
        if &next_id.0 != GRUP {
            return Ok((i, Self { cell, children: None }) )
        }

        // Peek at the next group header to see if it's CellChildren
        let (_, next_header) = GroupHeader::parse(i)?;

        match next_header.label {
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



