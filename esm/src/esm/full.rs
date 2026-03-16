use rayon::prelude::*;

use crate::{dev::*, groups::prelude::TopGroup, records::all::FileHeader, structs::chunk::get_file_chunks};


#[derive(Debug)]
pub struct ESMFull {
    pub header: FileHeader,
    pub groups: Vec<TopGroup>,
}

impl ESMFull {
    pub fn parse_mt(i: &[u8]) -> IResult<&[u8], Self> {
        
        let (i, chunks) = get_file_chunks(i)?;

        let (_, header) = FileHeader::parse(chunks[0].data)?;

        let groups = chunks.par_iter().skip(1).map(|x| {
            let (_, header) = GroupHeader::parse(x.data).unwrap();
            
            if let Ok((_, g)) = TopGroup::parse(x.data) {
                g
            } else {
                panic!("Failed parsing group: {:?}", header);
            }
        }).collect();


        Ok((i, Self { header, groups}))

    }

    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, header) = FileHeader::parse(i)?;
        let (i, groups) = many0(TopGroup::parse)(i)?;
        Ok((i, Self { header, groups}))
    }

    
}