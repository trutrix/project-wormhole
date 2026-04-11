use crate::dev::*;

// ====================================================================================================

/// Top level data pointer
#[derive(Debug)]
pub struct ESMChunk<'esm> {
    pub data: &'esm[u8]
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for ESMChunk<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, chunk) = alloc_chunk(i)?;
        Ok((i, chunk))
    }
}

// ====================================================================================================

pub fn alloc_chunk<'esm>(i: &'esm [u8]) -> IResult<&'esm [u8], ESMChunk<'esm>> {
    // Keep original pointer
    let orig = i;

    // Parse the iden
    let (i, iden) = FourCC::parse(i)?;

    // Parse the size
    let (_, size) = u32::parse_le(i)?;

    // If size zero, return empty buffer
    if size == 0 {
        Ok((i, ESMChunk { data: &[] }))
    }

    // If iden is header, add 24 to size of data buffer
    else if &iden.0 == b"TES4" {
        let (i, data) = take(size + 24)(orig)?;
        Ok((i, ESMChunk { data }))
    } 

    // If the iden is a group just take size normally
    else if &iden.0 == b"GRUP" {

        let (i, data) = take(size)(orig)?;
        Ok((i, ESMChunk { data }))
    } 
    
    // Undefined behavior
    else {
        panic!("alloc_chunk encountered unexpected chunk type: {:?}", iden);
    }
}

// ====================================================================================================

pub fn get_file_chunks<'esm>(i: &'esm [u8]) -> IResult<&'esm [u8], Vec<ESMChunk<'esm>>> {
    let (i, chunks) = many0(ESMChunk::parse)(i)?;
    Ok((i, chunks))
}

// ====================================================================================================