use std::io::{Read, Seek};

use crate::{dev::*, records::TES4_FileHeader::FileHeader, structs::record::{RawRecord, RecordHeader}};


pub struct ESM1<'esm> {
    file: std::fs::File,
    junk: Option<RawRecord<'esm>>,
    pub header: FileHeader,
    
}

impl<'esm> ESM1<'esm> {
    pub fn new(path: &str, hbuf: &'esm mut [u8;24], dbuf: &'esm mut Vec<u8>) -> Result<Self, ESMError> {
        // Open file handle
        let mut file = std::fs::File::open(path)?;

        // Read the first 24 bytes of the file into the buffer
        file.read_exact(hbuf)?;

        // Parse the Record from the buffer
        let (_, header) = RecordHeader::parse(hbuf).expect("Failed to parse first record header");

        // Create a new buffer for the entire first record (should be 'TES4')
        *dbuf = vec![0u8; header.size as usize + 24];

        // Seek to start and read the entire first record into the buffer
        file.seek(std::io::SeekFrom::Start(0))?;
        file.read_exact(dbuf)?;

        

        if let Ok((_, record)) = FileHeader::parse(dbuf) {
            Ok(ESM1 { file, junk: None, header: record.try_into().unwrap() })
        } else {
            Err(ESMError::InvalidFile)
        }
    }

    pub fn create_buffers() -> ([u8;24], Vec<u8>) {
        let hbuf = [0u8;24];
        let dbuf:  Vec<u8> = Vec::new();
        (hbuf, dbuf)
    }
}

#[derive(Debug)]
pub enum ESMError {
    IO(std::io::Error),
    InvalidFile,
    InvalidHeader,
    InvalidRecord,
    InvalidField,
    InvalidGroup,
    InvalidVersionControl,
    InvalidData,
}

impl From<std::io::Error> for ESMError {
    fn from(err: std::io::Error) -> Self {
        ESMError::IO(err)
    }
}