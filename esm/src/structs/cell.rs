use crate::{dev::*, records::all::{ActorReference, Cell, Landscape, NavigationMesh, Reference}};

#[derive(Debug)]
pub struct CellEntry {
    pub cell: Cell,
    pub children: Option<CellChildren>
}

impl Parse<&[u8]> for CellEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse the Cell record
        let (i, cell) = Cell::parse(i)?;    

        // Peek at the next FourCC to see if it's a GRUP
        let (_, next_id) = FourCC::parse(i)?;

        // If next iden is not GRUP, there are no children 
        // The groups themselves have pointers to parents, so in theory they could be out of order
        // In practice, they always seem to follow the Cell record directly.

        // Return early if no children group
        if &next_id.0 != GRUP {
            return Ok((i, Self { cell, children: None }) )
        }

        // Peek at the next group header to see if it's CellChildren
        let (_, next_header) = GroupHeader::parse(i)?;

        match next_header.label {
            GroupLabel::CellChildren(_) => {
                println!("  Parsing CellChildren for Cell: {:?}", cell.header.form_id);
                let (i, children) = CellChildren::parse(i)?;
                Ok((i, Self { cell, children: Some(children) }) )
            }
            _ => {
                println!("  No CellChildren group found for Cell: {:?}", cell.header.form_id);
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

        // Initialize optional children groups
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
                println!("    Found CellPersistentChildren in CellChildren group");
                let (raw, pc) = CellPersistentChildren::parse(raw)?;
                persistent_children = Some(pc);

                if raw.len() > 0 {
                    println!("      Found CellTemporaryChildren after CellPersistentChildren");
                    let (_, tc) = CellTemporaryChildren::parse(raw)?;
                    temporary_children = Some(tc);
                }
            }
            GroupLabel::CellTemporaryChildren(_) => {
                let (raw, tc) = CellTemporaryChildren::parse(raw)?;
                temporary_children = Some(tc);
                if raw.len() > 0 {
                    panic!("Warning: Extra data found after CellTemporaryChildren in CellChildren group");
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
    pub children: Vec<CellChildItem>,
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

        // Parse all child items from remaining raw data
        let (_, children) = many0(complete(CellChildItem::parse))(raw)?;

        Ok((i, Self { header, children }) )
    }
}

// ====================================================================================================


#[derive(Debug)]
pub struct CellTemporaryChildren {
    pub header: GroupHeader,
    pub children: Vec<CellChildItem>
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

        // Parse all child items from remaining raw data
        let (_, children) = many0(complete(CellChildItem::parse))(raw)?;

        Ok((i, Self { header, children }) )
    }
}


// ====================================================================================================

#[derive(Debug)]
pub enum CellChildItem {
    ItemReference(Reference),
    Landscape(Landscape),
    ActorReference(ActorReference),
    NavigationMesh(NavigationMesh)
}


impl Parse<&[u8]> for CellChildItem {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, header) = RecordHeader::parse(i)?;

        match &header.iden.0{
            b"REFR" => {
                let (i, reference) = Reference::parse(i)?;
                Ok((i, CellChildItem::ItemReference(reference)))
            }
            b"LAND" => {
                let (i, landscape) = Landscape::parse(i)?;
                Ok((i, CellChildItem::Landscape(landscape)))
            }
            b"ACHR" => {
                let (i, actor_ref) = ActorReference::parse(i)?;
                Ok((i, CellChildItem::ActorReference(actor_ref)))
            }
            b"NAVM" => {
                let (i, nav_mesh) = NavigationMesh::parse(i)?;
                Ok((i, CellChildItem::NavigationMesh(nav_mesh)))
            }
            _ => {
                panic!("CellChildItem::parse encountered unknown record type: {:?}", header.iden);
            }
        }
    }
}