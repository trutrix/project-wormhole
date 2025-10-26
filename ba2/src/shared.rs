use std::fmt::Debug;

use shared::common::SizedString16;

use super::prelude::*;



#[derive(Debug, Clone)]
pub struct BA2Header {
    pub id: FourCC,
    pub version: u32,
    pub archive_type: ArchiveType,
    pub file_count: u32,
    pub name_table_offset: u64
}

impl Parse<&[u8]> for BA2Header {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, error::Error<&[u8]>> {
        let (i, id) = FourCC::parse(i)?;

        if id != cc4!(BTDX) {
            return Err(nom::Err::Error(error::Error::new(i, nom::error::ErrorKind::Tag)));
        }

        let (i, version) = nom::number::complete::le_u32(i)?;
        let (i, archive_type) = ArchiveType::parse(i)?;
        let (i, file_count) = nom::number::complete::le_u32(i)?;
        let (i, name_table_offset) = nom::number::complete::le_u64(i)?;
        Ok((i, BA2Header { id, version, archive_type, file_count, name_table_offset }))
    }
}



#[derive(PartialEq, Clone)]
pub enum ArchiveType {
    General,
    Texture
}

impl Parse<&[u8]> for ArchiveType {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, error::Error<&[u8]>> {
        let (i, archive_type) = nom::number::complete::le_u32(i)?;
        match archive_type {
            cc4!(GNRL) => Ok((i, ArchiveType::General)),
            cc4!(DX10) => Ok((i, ArchiveType::Texture)),
            _ => Err(nom::Err::Error(error::Error::new(i, nom::error::ErrorKind::Tag)))
        }
    }
}

impl Debug for ArchiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveType::General => write!(f, "General [GRNL]"),
            ArchiveType::Texture => write!(f, "Texture [DX10]")
        }
    }
}


pub fn get_file_names(file: &mut File, offset: u64) -> Result<Vec<String>, std::io::Error> {

    // Store the current position, so we can return to it later
    let origin = file.stream_position()?;

    // Get file size
    let file_size = file.metadata()?.len();

    // Create buffer for the name table
    let mut buf = vec![0u8; (file_size - offset) as usize];
    
    // Seek to the name table offset
    file.seek(SeekFrom::Start(offset))?;
    
    // Read the name table
    file.read_exact(&mut buf)?;

    // Seek back to the original position (before parsing, incase there is a recoverable error)
    file.seek(SeekFrom::Start(origin))?;

    // Parse the names
    let (_, names) = many0(complete(SizedString16::parse))(&buf).unwrap();

    let names = names.iter().map(|s| s.0.clone()).collect();

    Ok(names)
}
