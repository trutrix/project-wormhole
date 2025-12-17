use super::dev::*;


#[derive(Debug, Clone)]
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



#[derive(PartialEq, Clone)]
pub enum ArchiveType {
    General,
    Texture
}

impl Parse<&[u8]> for ArchiveType {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, error::Error<&[u8]>> {
        let (i, archive_type) = take(4usize)(i)?;
        if let Ok(archive_type) = <&[u8; 4]>::try_from(archive_type) {
            if archive_type == b"GNRL" {
                Ok((i, ArchiveType::General))
            } else if archive_type == b"DX10" {
                Ok((i, ArchiveType::Texture))
            } else {
                Err(nom::Err::Error(error::Error::new(i, nom::error::ErrorKind::Tag)))
            }
        } else {
            Err(nom::Err::Error(error::Error::new(i, nom::error::ErrorKind::Char)))
        }
    }
}

impl std::fmt::Debug for ArchiveType {
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



pub fn sized_string_none_if_empty(i: &[u8], size: StringSize) -> IResult<&[u8], Option<String>> {
    match size {
        StringSize::U8 => {
            let (i, len) = le_u8(i)?;
            if len == 0 {
                Ok((i, None))
            } else {
                let (i, s) = nom::bytes::complete::take(len)(i)?;
                Ok((i, Some(String::from_utf8_lossy(s).to_string())))
            }
        },
        StringSize::U16 => {
            let (i, len) = le_u16(i)?;
            if len == 0 {
                Ok((i, None))
            } else {
                let (i, s) = nom::bytes::complete::take(len)(i)?;
                Ok((i, Some(String::from_utf8_lossy(s).to_string())))
            }
        },
        StringSize::U32 => {
            let (i, len) = le_u32(i)?;
            if len == 0 {
                Ok((i, None))
            } else {
                let (i, s) = nom::bytes::complete::take(len)(i)?;
                Ok((i, Some(String::from_utf8_lossy(s).to_string())))
            }
        },
    }
}

pub enum StringSize {
    U8,
    U16,
    U32
}

pub fn sized8_string_none_if_empty(i: &[u8]) -> IResult<&[u8], Option<String>> {
    sized_string_none_if_empty(i, StringSize::U8)
}

pub fn sized16_string_none_if_empty(i: &[u8]) -> IResult<&[u8], Option<String>> {
    sized_string_none_if_empty(i, StringSize::U16)
}

pub fn sized32_string_none_if_empty(i: &[u8]) -> IResult<&[u8], Option<String>> {
    sized_string_none_if_empty(i, StringSize::U32)
}


pub struct SizedString<T> {
    pub size: T,
    pub value: Option<String>
}

impl nom_derive::Parse<&[u8]> for SizedString<u32> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, size) = le_u32(i)?;
        if size == 0 {
            Ok((i, SizedString { size, value: None }))
        } else {
            let (i, value) = nom::bytes::complete::take(size)(i)?;
            Ok((i, SizedString { size, value: Some(String::from_utf8_lossy(value).to_string()) }))
        }
    }
}

impl nom_derive::Parse<&[u8]> for SizedString<u16> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, size) = le_u16(i)?;
        if size == 0 {
            Ok((i, SizedString { size, value: None }))
        } else {
            let (i, value) = nom::bytes::complete::take(size)(i)?;
            Ok((i, SizedString { size, value: Some(String::from_utf8_lossy(value).to_string()) }))
        }
    }
}

impl nom_derive::Parse<&[u8]> for SizedString<u8> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, size) = le_u8(i)?;
        if size == 0 {
            Ok((i, SizedString { size, value: None }))
        } else {
            let (i, value) = nom::bytes::complete::take(size)(i)?;
            Ok((i, SizedString { size, value: Some(String::from_utf8_lossy(value).to_string()) }))
        }
    }
}

impl<T> std::convert::Into<String> for SizedString<T> {
    fn into(self) -> String {
        self.value.unwrap_or_default()
    }
}



#[derive(Debug, PartialEq, NomLE, Clone, Copy)]
pub struct Bounds {
    pub center: [f32;3],
    pub radius: f32,
}



#[derive(PartialEq, Eq, Clone)]
pub struct SizedString32(pub String);

impl Parse<&[u8]> for SizedString32 {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, len) = le_u32(i)?;
        let (i, s) = take(len)(i)?;
        let s = String::from_utf8_lossy(s).to_string().replace('\0', "");
        Ok((i, SizedString32(s)))
    }
}

impl SizedString32 {
    pub fn parse_empty_as_none(i: &[u8]) -> IResult<&[u8], Option<String>> {
        let (i, result) = Self::parse(i)?;
        if result.0.len() == 0 {
            Ok((i, None))
        } else {
            Ok((i, Some(result.0)))
        }
    }
}

impl std::fmt::Display for SizedString32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for SizedString32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SizedString16(pub String);

impl Parse<&[u8]> for SizedString16 {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, len) = le_u16(i)?;
        let (i, s) = take(len)(i)?;
        let s = String::from_utf8_lossy(s).to_string().replace('\0', "");
        Ok((i, SizedString16(s)))
    }
}

impl std::fmt::Display for SizedString16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for SizedString16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

pub fn parse_ss16(i: &[u8]) -> IResult<&[u8], String> {
    let (i, len) = le_u16(i)?;
    let (i, s) = nom::bytes::complete::take(len)(i)?;
    Ok((i, String::from_utf8_lossy(s).to_string()))
}

#[derive(PartialEq, Eq)]
pub struct SizedString8(pub String);

impl Parse<&[u8]> for SizedString8 {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, len) = le_u8(i)?;
        let (i, s) = take(len)(i)?;
        let s = String::from_utf8_lossy(s).to_string().replace('\0', "");
        Ok((i, SizedString8(s)))
    }
}

impl std::fmt::Display for SizedString8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for SizedString8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bool(pub bool);

impl Parse<&[u8]> for Bool {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, b) = le_u8(i)?;
        Ok((i, Bool(b != 0)))
    }
}

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}



pub fn normalize_path(path: String) -> String {
    let mut path = path
        .replace('\\', "/")
        .replace('\0', "")
        .to_lowercase();

    if path.starts_with("./") {
        path = path[2..].to_string();
    }

    if !path.starts_with("textures/") {
        path = format!("textures/{}", path);
    }
    
    path
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MaxRef(pub Option<u32>);

impl Parse<&[u8]> for MaxRef {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, value) = le_u32(i)?;

        if value == u32::MAX {
            Ok((i, MaxRef(None)))
        } else {
            Ok((i, MaxRef(Some(value))))
        }
    }
}





pub fn standardize_path(path: &str) -> String {
    let mut path = path
        .replace('\\', "/")
        .replace('\0', "")
        .to_lowercase();

    if path.starts_with("./") {
        path = path[2..].to_string();
    }
    
    path
}

pub fn ensure_texture_parent(path: &mut String) {
    if !path.starts_with("textures/") {
        path.insert_str(0, "textures/");
    }
}

pub fn parse_u8_as_bool(i: &[u8]) -> IResult<&[u8], bool> {
    let (i, value) = le_u8(i)?;
    Ok((i, value == 1))
}


#[derive(Debug)]
pub enum Endianess {
    Big,
    Little
}

impl Parse<&[u8]> for Endianess {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, endian_type) = le_u8(i)?;
        match endian_type {
            0x00 => Ok((i, Endianess::Big)),
            0x01 => Ok((i, Endianess::Little)),
            _ => Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Tag)))
        }
    }
}


#[derive(Debug, NomLE)]
pub struct BoolU8(pub u8);

impl From<BoolU8> for bool {
    fn from(b: BoolU8) -> Self {
        if b.0 == 0 {
            false
        } else {
            true
        }
    }
}