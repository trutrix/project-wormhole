use crate::{dev::*, records::all::Worldspace};


#[derive(Debug, NomLE)]
#[repr(C)]
pub struct WorldEntry {
    pub worldspace: Worldspace,
    pub children: WorldChildren
}


#[derive(Debug)]
#[repr(C)]
pub struct WorldChildren {
    pub header: GroupHeader
}

impl Parse<&[u8]> for WorldChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, _raw)) = alloc_group(i)?;


        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::WorldChildren(_) => { 
                Ok((i, Self { header }))
            }
            _ => { panic!("WorldChildren::parse encountered wrong group type: {:?}", header.label) }
        } 

        #[cfg(not(debug_assertions))]
        {
            Ok((i, Self { header }))
        }
    }
}