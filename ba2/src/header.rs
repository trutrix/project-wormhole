use crate::dev::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BA2Header {
    pub id: [u8;4],
    pub version: u32,
    pub archive_type: ArchiveType,
    pub file_count: u32,
    pub name_table_offset: u64
}

impl Parse<&[u8]> for BA2Header {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, error::Error<&[u8]>> {
        let (i, id) = <[u8;4]>::parse(i)?;

        if id != *b"BTDX" {
            return Err(nom::Err::Error(error::Error::new(i, nom::error::ErrorKind::Tag)));
        }

        let (i, version) = le_u32(i)?;
        let (i, archive_type) = ArchiveType::parse(i)?;
        let (i, file_count) = le_u32(i)?;
        let (i, name_table_offset) = le_u64(i)?;
        Ok((i, BA2Header { id, version, archive_type, file_count, name_table_offset }))
    }
}
