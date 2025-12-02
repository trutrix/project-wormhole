use crate::{dev::*, records::all::FileHeader, structs::data};


// ====================================================================================================

#[derive(Debug, NomLE, zerocopy::FromBytes)]
pub struct ChunkHeader {
    pub iden: FourCC,
    pub size: u32,
    pub field1: u32,
    pub field2: u32,
    pub field3: [u8; 8],
}

// ====================================================================================================

#[derive(Debug)]
pub struct ESMChunk<'esm> {
    pub header: ChunkHeader,
    pub data: &'esm[u8]
}


impl<'esm> Parse<&'esm[u8]> for ESMChunk<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, chunk) = alloc_chunk(i)?;
        Ok((i, chunk))
    }
}

pub fn alloc_chunk(i: &'_ [u8]) -> IResult<&'_ [u8], ESMChunk<'_>> {

    let (i, header) = ChunkHeader::parse(i)?;

    if header.size == 0 {
        Ok((i, ESMChunk { header, data: &[] }))
    } else if &header.iden.0 == b"TES4" {
        let (i, data) = take(header.size)(i)?;
        Ok((i, ESMChunk { header, data }))
    } else if &header.iden.0 == b"GRUP" {
        let (i, data) = take(header.size - 24)(i)?;
        Ok((i, ESMChunk { header, data }))
    } else {
        panic!("alloc_chunk encountered unexpected chunk type: {:?}", header.iden);
    }
}


pub fn get_file_chunks(i: &'_ [u8]) -> IResult<&'_ [u8], Vec<ESMChunk<'_>>> {
    let (i, chunks) = many0(ESMChunk::parse)(i)?;
    Ok((i, chunks))
}


pub struct SmartChunks<'esm> {
    pub header: FileHeader,
    pub chunks: Vec<ESMChunk2<'esm>>
}


impl<'esm> Parse<&'esm[u8]> for SmartChunks<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, header) = FileHeader::parse(i)?;
        let (i, chunks) = many0(ESMChunk2::parse)(i)?;

        Ok((i, Self { header, chunks }))
    }
}


// ====================================================================================================


pub struct ESMChunk2<'esm> {
    pub header: GroupHeader,
    pub data: &'esm[u8]
}

impl<'esm> Parse<&'esm[u8]> for ESMChunk2<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}