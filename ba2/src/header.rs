use crate::dev::*;

use project_wormhole_shared::structs::fourcc::FourCC;
use nom::error::ErrorKind;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BA2Header {
    pub id: FourCC,
    pub version: u32,
    pub archive_type: FourCC,
    pub file_count: u32,
    pub name_table_offset: u64
}

impl Parse<&[u8]> for BA2Header {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, error::Error<&[u8]>> {
        
        let (i, id) = FourCC::parse(i)?;

        if id.0 != *b"BTDX" {
            return Err(nom::Err::Error(error::Error::new(i, ErrorKind::Tag)));
        }

        let (i, version) = le_u32(i)?;

        let ab = i;
        let (i, archive_type) = FourCC::parse(i)?;


        if archive_type.0 != *b"GNRL" && archive_type.0 != *b"DX10" {
            return Err(nom::Err::Error(error::Error::new(&ab[0..4], ErrorKind::Tag)));
        }

        let (i, file_count) = le_u32(i)?;
        let (i, name_table_offset) = le_u64(i)?;
        Ok((i, BA2Header { id, version, archive_type, file_count, name_table_offset }))
    }
}


impl BA2Header {
    pub fn is_texture_archive(&self) -> bool {
        self.archive_type.0 == *b"DX10"
    }

    pub fn is_general_archive(&self) -> bool {
        self.archive_type.0 == *b"GNRL"
    }
}