use crate::{dev::*, records::all::{Cell, Reference}};

#[derive(Debug)]
pub struct CellEntry {
    pub cell: Cell,
    pub children: Option<CellChildren>
}

impl Parse<&[u8]> for CellEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        let (i, cell) = Cell::parse(i)?;

        println!("Parsed Cell: {:?}", cell.header.form_id);

        let (_, header) = GroupHeader::parse(i)?;

        match header.label {
            GroupLabel::CellChildren(_) => {
                println!("Parsing CellChildren for Cell: {:?}", cell.header.form_id);
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
    pub persistent_children: Option<CellPersistentChildren>,
    pub temporary_children: Option<CellTemporaryChildren>
}


// Implement nom_derive::Parse
impl Parse<&[u8]> for CellChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        let mut persistent_children = None;
        let mut temporary_children = None;

        // Ensure correct group type - debugging only
        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::CellChildren(_) => { }
            _ => { panic!("CellChildren::parse encountered wrong group type: {:?}", header.label) }
        }

        let (_, next_header) = GroupHeader::parse(raw)?;
        
        match next_header.label {
            GroupLabel::CellPersistentChildren(_) => {
                let (raw, pc) = CellPersistentChildren::parse(raw)?;
                persistent_children = Some(pc);

                if raw.len() > 0 {
                    let (_, tc) = CellTemporaryChildren::parse(raw)?;
                    temporary_children = Some(tc);
                }
            }
            GroupLabel::CellTemporaryChildren(_) => {
                let (raw, tc) = CellTemporaryChildren::parse(raw)?;
                temporary_children = Some(tc);
                if raw.len() > 0 {
                    println!("Warning: Extra data found after CellTemporaryChildren in CellChildren group");
                }
            }
            _ => { }
        }

        Ok((i, Self { header, persistent_children, temporary_children }) )
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct CellPersistentChildren {
    pub header: GroupHeader,
    pub children: Vec<Reference>
}

impl Parse<&[u8]> for CellPersistentChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        // Ensure correct group type - debugging only
        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::CellPersistentChildren(_) => { }
            _ => { panic!("CellPersistentChildren::parse encountered wrong group type: {:?}", header.label) }
        }

        let (_, children) = many0(complete(Reference::parse))(raw)?;

        //println!("Parsed {} persistent children", children.len());

        Ok((i, Self { header, children }) )
    }
}

// ====================================================================================================


#[derive(Debug)]
pub struct CellTemporaryChildren {
    pub header: GroupHeader,
    pub children: Vec<Reference>
}

impl Parse<&[u8]> for CellTemporaryChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        // Ensure correct group type - debugging only
        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::CellTemporaryChildren(_) => { }
            _ => { panic!("CellTemporaryChildren::parse encountered wrong group type: {:?}", header.label) }
        }

        let (_, children) = many0(complete(Reference::parse))(raw)?;

        //println!("Parsed {} temporary children", children.len());

        Ok((i, Self { header, children }) )
    }
}


// ====================================================================================================