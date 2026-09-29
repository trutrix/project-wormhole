use crate::dev::*;

// ====================================================================================================

/// Top level data pointer
#[derive(Debug)]
pub struct ESMChunk<'es> {
    pub data: &'es[u8]
}

// ====================================================================================================

impl<'es> Parse<&'es[u8]> for ESMChunk<'es> {
    fn parse(i: &'es[u8]) -> IResult<&'es[u8], Self, nom::error::Error<&'es[u8]>> {
        let (i, chunk) = alloc_chunk(i)?;
        Ok((i, chunk))
    }
}

// ====================================================================================================

pub fn alloc_chunk<'es>(i: &'es [u8]) -> IResult<&'es [u8], ESMChunk<'es>> {
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

pub fn get_file_chunks<'es>(i: &'es [u8]) -> IResult<&'es [u8], Vec<ESMChunk<'es>>> {
    let (i, chunks) = many0(ESMChunk::parse)(i)?;
    Ok((i, chunks))
}

// ====================================================================================================