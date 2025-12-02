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


#[derive(Debug)]
pub struct SmartChunks<'esm> {
    pub header: FileHeader,
    pub data_groups: Vec<ESMChunk<'esm>>,
    pub reference_groups: Vec<ESMChunk<'esm>>
}


impl<'esm> Parse<&'esm[u8]> for SmartChunks<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let mut data_groups = Vec::new();
        let mut reference_groups = Vec::new();
        
        
        let (i, header) = FileHeader::parse(i)?;

        let mut remaining = i;

        while remaining.len() > 0 {

            let (_, header) = GroupHeader::parse(remaining)?;
            let (i, chunk) = alloc_chunk(remaining)?;
            remaining = i;

            if REFERENCE_GROUPS.contains(&&header.iden.0) {
                reference_groups.push(chunk);
            } else {
                data_groups.push(chunk);
            }


        }


        Ok((remaining, Self { header, data_groups, reference_groups }))


    }
}