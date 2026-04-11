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

#[derive(Clone)]
pub struct ESMFileChunk<'esm> {
    pub data: &'esm[u8]
}

impl ESMFileChunk<'_> {
    pub fn is_group(&self) -> bool {
        if let Ok((_, iden)) = FourCC::parse(self.data) {
            &iden.0 == b"GRUP"
        } else {
            false
        }
    }

    pub fn is_refr_group(&self) -> bool {
        if let Ok((_, group_header)) = GroupHeader::parse(self.data) {
            match group_header.label {
                GroupLabel::Top(four_cc) => {
                    REFERENCE_GROUPS.contains(&&four_cc.0)
                },
                _ => false
            }
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}


impl<'esm> Parse<&'esm[u8]> for ESMFileChunk<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        
        let orig = i;
        let (i, iden) = FourCC::parse(i)?;
        let (_, size) = u32::parse_le(i)?;

        let (i, data) = if &iden.0 == b"GRUP" {
            take(size)(orig)?
        } else {
            take(size + 24)(orig)?
        };

        Ok((i, Self { data }) )
    }
}

impl std::fmt::Debug for ESMFileChunk<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileChunk {{ data: [{} bytes] }}", self.data.len())
    }
}

impl std::fmt::Display for ESMFileChunk<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}




pub fn get_file_chunks2(i: &'_ [u8]) -> IResult<&'_ [u8], (Vec<ESMFileChunk<'_>>, Vec<ESMFileChunk<'_>>)> {
    let mut rchunks = Vec::new();
    let mut chunks = Vec::new();

    let mut input = i;
    let mut first = true;
    loop {
        if input.is_empty() {
            break;
        }

        let (i, chunk) = ESMFileChunk::parse(input)?;
        input = i;

        if first {
            // First chunk is always the header
            chunks.push(chunk);
            first = false;
            continue;
        } else if chunk.is_refr_group() {
            rchunks.push(chunk);
        } else {
            chunks.push(chunk);
        }
    }

    Ok((input, (chunks, rchunks)))
}