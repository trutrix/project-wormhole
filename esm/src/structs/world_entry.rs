use crate::{dev::*, records::all::{Cell, Worldspace}};


#[derive(Debug, NomLE)]
#[repr(C)]
pub struct WorldEntry {
    pub worldspace: Worldspace,
    pub children: WorldChildren
}


#[derive(Debug)]
#[repr(C)]
pub struct WorldChildren {
    pub header: GroupHeader,
    pub cell: Cell
}

impl Parse<&[u8]> for WorldChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;

        

        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::WorldChildren(_) => { }
            _ => { panic!("WorldChildren::parse encountered wrong group type: {:?}", header.label) }
        }
    
        let (raw, cell) = Cell::parse(raw)?;

        let (_, next_id) = FourCC::parse(raw)?;
        println!("Next ID after WorldChildren Cell: {:?}", next_id);

        println!("{:?}", cell);
        Ok((i, Self { header, cell }) )
    }
}