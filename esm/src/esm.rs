use std::{collections::HashMap, io::{Read, Seek}};

use crate::{dev::*, records::{all::{FileHeaderField, GameSettingField}, TES4::FileHeader}, structs::record::{RawRecord, RecordHeader}, traits::{RecordParser, GroupParser}};


// ====================================================================================================

pub const SPECIAL_GROUPS: [&[u8;4];3] = [b"WRLD", b"CELL", b"QUST"];

// ====================================================================================================


pub struct ESM1<'esm> {
    _file: std::fs::File,
    _junk: Option<RawRecord<'esm>>,
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
            Ok(ESM1 { _file: file, _junk: None, header: record.try_into().unwrap(), groups: Vec::new() })
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


/// This is a barebones parsing of an ESM file.  
/// It does not attempt to interpret any records or fields.  
/// It simply breaks the file into its constituent groups and records.  
/// This is useful for debugging and for understanding the structure of the file. 
/// More advanced parsing can be built on top of this.    

#[derive(Debug)]
pub struct RawESM<'esm> {
    pub header: FileHeader,
    pub cells: Vec<RawInteriorCellBlock<'esm>>,
    pub worlds: Vec<RawWorldGroup<'esm>>,
    pub records: HashMap<FormId, RawRecord<'esm>>,
    pub quests: Vec<RawQuestGroup<'esm>>,
}

impl<'esm> RawESM<'esm> {
    pub fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {
        let mut cells = Vec::new();
        let mut worlds = Vec::new();
        let mut records = HashMap::new();
        let mut quests = Vec::new();


        let (i, header) = FileHeader::parse(i)?;
        let mut raw = i;

        while raw.len() > 0 {

            let (_, gh) = GroupHeader::parse(raw)?;
            

            match gh.label {
                GroupLabel::Top(iden) => {
                    match &iden.0 {
                        b"CELL" => {
                            let (i, (_ghead, graw)) = alloc_group(raw)?;
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
                            let (i, gq) = RawQuestGroup::parse(raw)?;
                            raw = i;
                            quests.push(gq);
                        }
                        _ => {
                            // println!("Parsing {:?}", gh.label);
                            let (i, rg) = RawDataGroup::parse(raw)?;
                            raw = i;
                            for r in rg.data {
                                records.insert(r.header.form_id.clone(), r);
                            }
                        }
                    }
                }
                _ => {
                    panic!("Encountered non-top group in RawESM")
                }
            }


        }

        Ok((i, Self { header, cells, worlds, records, quests }))
    }
}

// ====================================================================================================


/// A more fully-featured ESM parser that attempts to interpret records and fields
/// This is still a work in progress and is not yet complete
pub struct SmartESM {
    pub header: Record<FileHeaderField>
}

impl SmartESM {
    pub fn parse_complete(i: &[u8]) -> Result<Self, ESMError> {
        if let Ok((i, header)) = FileHeader::parse_record(i) {

            
            if let Ok((_, gmst)) = <Group<Record<GameSettingField>>>::parse_group(i) {
                println!("{:?}", gmst.data[0].header);
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