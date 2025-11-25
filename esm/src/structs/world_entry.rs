use crate::{dev::*, records::all::{Cell, Worldspace}};


#[derive(Debug, NomLE)]
pub struct WorldEntry {
    pub worldspace: Worldspace,
    pub children: WorldChildren
}


#[derive(Debug)]
pub struct WorldChildren {
    pub header: GroupHeader,
    pub cell: Cell
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
        let (raw, cell) = Cell::parse(raw)?;


        let (_, next_id) = GroupHeader::parse(raw)?;
        println!("Next ID after WorldChildren Cell: {:?}", next_id);

        println!("{:?}", cell);
        Ok((i, Self { header, cell }) )
    }
}