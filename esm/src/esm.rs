use std::{collections::HashMap, io::{Read, Seek}};

use crate::{dev::*, records::{all::{FileHeaderField, GameSetting, GameSettingField}, TES4::FileHeader}, structs::record::{RawRecord, RecordHeader}, traits::{RecordParser, GroupParser}};


// ====================================================================================================


pub struct ESM1<'esm> {
    file: std::fs::File,
    junk: Option<RawRecord<'esm>>,
    pub header: FileHeader,
    pub groups: Vec<RawDataGroup<'esm>>,
    
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
            Ok(ESM1 { file, junk: None, header: record.try_into().unwrap(), groups: Vec::new() })
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


// ====================================================================================================

#[derive(Debug)]
pub struct RawESM<'esm> {
    pub header: FileHeader,
    pub cells: Vec<RawInteriorCellBlock<'esm>>,
    pub worlds: Vec<RawWorldGroup<'esm>>,
    pub records: HashMap<u32, RawRecord<'esm>>
}

impl<'esm> RawESM<'esm> {
    pub fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {
        let mut cells = Vec::new();
        let mut worlds = Vec::new();
        let mut records = HashMap::new();


        let (i, header) = FileHeader::parse(i)?;
        let mut raw = i;

        while raw.len() > 0 {

            let (_, gh) = GroupHeader::parse(raw)?;
            

            match gh.label {
                GroupLabel::Top(iden) => {
                    match &iden.0 {
                        b"CELL" => {
                            let (i, (ghead, graw)) = alloc_group(raw)?;
                            // println!("{:?}", ghead);
                            raw = i;
                            let (_, icb) = many0(complete(RawInteriorCellBlock::parse))(graw)?;
                            cells = icb;
                        }
                        b"WRLD" => {
                            // println!("Parsing {:?}", gh.label);
                            let (i, gw) = RawWorldGroup::parse(raw)?;
                            raw = i;
                            worlds.push(gw);
                        }
                        b"QUST" => {
                            // println!("Skipping: {:?}", gh.label);
                            let (i, _) = alloc_group(raw)?;
                            raw = i;
                        }
                        _ => {
                            // println!("Parsing {:?}", gh.label);
                            let (i, rg) = RawDataGroup::parse(raw)?;
                            raw = i;
                            for r in rg.data {
                                records.insert(r.header.form_id, r);
                            }
                        }
                    }
                }
                _ => {
                    panic!("Encountered non-top group in RawESM")
                }
            }


        }

        Ok((i, Self { header, cells, worlds, records }))
    }
}

// ====================================================================================================

pub struct SmartESM {
    pub header: Record<FileHeaderField>
}

impl SmartESM {
    pub fn parse_complete(i: &[u8]) -> Result<Self, ESMError> {
        if let Ok((i, header)) = FileHeader::parse_record(i) {

            
            if let Ok((i, gmst)) = <Group<Record<GameSettingField>>>::parse_group(i) {
                println!("yay");
                Ok(Self { header })
            } else {
                println!("nay");
                Ok(Self { header })
            }


        } else {
            Err(ESMError::InvalidHeader)
        }
        
    }

    
}


// ====================================================================================================


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
    GameSetting(String)
}

impl From<std::io::Error> for ESMError {
    fn from(err: std::io::Error) -> Self {
        ESMError::IO(err)
    }
}