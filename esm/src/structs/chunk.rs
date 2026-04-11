use crate::{dev::*, records::all::FileHeader};


// ====================================================================================================

#[derive(Debug)]
pub struct ESMChunk<'esm> {
    pub data: &'esm[u8]
}


impl<'esm> Parse<&'esm[u8]> for ESMChunk<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, chunk) = alloc_chunk(i)?;
        Ok((i, chunk))
    }
}

pub fn alloc_chunk(i: &'_ [u8]) -> IResult<&'_ [u8], ESMChunk<'_>> {
    let orig = i;
    let (i, iden) = FourCC::parse(i)?;
    let (_, size) = u32::parse_le(i)?;

    if size == 0 {
        Ok((i, ESMChunk { data: &[] }))
    } else if &iden.0 == b"TES4" {
        let (i, data) = take(size + 24)(orig)?;
        Ok((i, ESMChunk { data }))
    } else if &iden.0 == b"GRUP" {
        let (i, data) = take(size)(orig)?;
        Ok((i, ESMChunk { data }))
    } else {
        panic!("alloc_chunk encountered unexpected chunk type: {:?}", iden);
    }
}


pub fn get_file_chunks(i: &'_ [u8]) -> IResult<&'_ [u8], Vec<ESMChunk<'_>>> {
    let (i, chunks) = many0(ESMChunk::parse)(i)?;
    Ok((i, chunks))
}

// ====================================================================================================