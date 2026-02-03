use std::{collections::HashMap, fs::File, io::{Read, Seek}, path::PathBuf};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{dev::*, records::{self, SingleRecord, SingleRecordRef, all::*}, structs::{chunk::{get_file_chunks, get_file_chunks2}, group::TopGroup, record::RawRecord, world::WorldEntry}};


// ====================================================================================================


pub trait ESMUtils {
    fn load_file(file_path: &str) -> Result<Self, ESMError> where Self: Sized;
    fn load_dir(dir_path: &str) -> Result<Self, ESMError> where Self: Sized;
    fn append<T>(&mut self, other: &T);
    fn parse(i: &[u8]) -> Result<Self, ESMError> where Self: Sized;
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

        while !raw.is_empty() {

            let (_, gh) = GroupHeader::parse(raw)?;
            

            match gh.label {
                GroupLabel::Top(iden) => {
                    match &iden.0 {
                        b"CELL" => {
                            let (i, (_ghead, graw)) = alloc_group(raw)?;
                            // println!("{:?}", ghead);
                            raw = i;
                            let (_, icb) = many0(RawInteriorCellBlock::parse)(graw)?;
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

    pub fn parse_mt(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {
        let mut cells = Vec::new();
        let mut worlds = Vec::new();
        let mut records = HashMap::new();
        let mut quests = Vec::new();

        
        let (i, header) = FileHeader::parse(i)?;
        let mut raw = i;

        while !raw.is_empty() {

            let (_, gh) = GroupHeader::parse(raw)?;
            

            match gh.label {
                GroupLabel::Top(iden) => {
                    match &iden.0 {
                        b"CELL" => {
                            let (i, (_ghead, graw)) = alloc_group(raw)?;
                            // println!("{:?}", ghead);
                            raw = i;
                            let (_, icb) = many0(RawInteriorCellBlock::parse)(graw)?;
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
#[deprecated]
pub struct SmartESM {
    pub header: FileHeader,
    // pub chunks: Vec<TopGroup>,
    // pub rchunks: Vec<TopGroup>,
    pub data_groups: Vec<TopGroup>
}

impl Parse<&[u8]> for SmartESM {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        
        let (leftover, (chunks, rchunks)) = get_file_chunks2(i)?;
        //println!("Chunks: {}, RChunks: {}", chunks.len(), rchunks.len());

        // Debugging if file has leftover data after parsing chunks
        #[cfg(debug_assertions)]
        {
            if !leftover.is_empty() {
                println!("Warning: leftover data after parsing file chunks: {} bytes", leftover.len());
            }
            //println!("Parsed {} file chunks", chunks.len());
        }

        // First chunk should be the file header
        let (_, header) = FileHeader::parse(chunks[0].data)?;
        let mut parsed_data = Vec::new();
        let mut parsed_refr = Vec::new();

        rayon::scope(|s|{
            // Data thread
            s.spawn(|_|{
                //let start = std::time::Instant::now();
                for chunk in chunks.iter().skip(1) {
                    parsed_data.push(TopGroup::parse(chunk.data));
                }
                //println!("Data groups parse time: {:?}", start.elapsed())
            });

            //Refr thread
            s.spawn(|_|{
                //let start = std::time::Instant::now();
                for rchunk in rchunks {
                    parsed_refr.push(TopGroup::parse(rchunk.data));
                }
                //println!("Refr groups parse time: {:?}", start.elapsed())
            });

        });

        Ok((i, Self { header, data_groups: Vec::new() }) )
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct ESMFull {
    pub header: FileHeader,
    pub groups: Vec<TopGroup>,
}

impl ESMFull {
    pub fn parse_mt(i: &[u8]) -> IResult<&[u8], Self> {
        
        let (i, chunks) = get_file_chunks(i)?;

        let (_, header) = FileHeader::parse(chunks[0].data)?;

        let groups = chunks.par_iter().skip(1).map(|x| {
            let (_, header) = GroupHeader::parse(x.data).unwrap();
            
            if let Ok((_, g)) = TopGroup::parse(x.data) {
                g
            } else {
                panic!("Failed parsing group: {:?}", header);
            }
        }).collect();


        Ok((i, Self { header, groups}))

    }

    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, header) = FileHeader::parse(i)?;
        let (i, groups) = many0(TopGroup::parse)(i)?;
        Ok((i, Self { header, groups}))
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



// ================================================================================

use std::rc::Rc;

pub struct SmartESM2 {
    pub header: FileHeader,
    pub records: HashMap<FormId, SingleRecord>
}

impl ESMUtils for SmartESM2 {
    fn load_file(file_path: &str) -> Result<Self, ESMError> where Self: Sized {
        let mut file = File::open(file_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(SmartESM2::parse(&buf)?)
    }
    
    fn load_dir(dir_path: &str) -> Result<Self, ESMError> where Self: Sized {
        todo!()
    }
    
    fn append<T>(&mut self, other: &T) {
        todo!()
    }
    
    fn parse(i: &[u8]) -> Result<Self, ESMError> where Self: Sized {
        let (_, esm) = ESMFull::parse_mt(i).map_err(|_| ESMError::InvalidGroup)?;

        let mut records: HashMap<FormId, SingleRecord> = HashMap::new();

        for group in esm.groups {

            match group {
                TopGroup::Unhandled(group) => {
                    //println!("Unhandled group: {:?}", group.header.label);
                },
                TopGroup::AACT(group) => { 
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::AACT(item)).unwrap();
                    }
                },
                TopGroup::ACTI(group) => { 
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::ACTI(item)).unwrap();
                    }
                },
                TopGroup::ADDN(group) => { 
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::ADDN(item)).unwrap();
                    }
                },
                TopGroup::AECH(group) => {
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::AECH(item));
                    }
                }
                TopGroup::ALCH(group) => {
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::ALCH(item));
                    }
                }
                TopGroup::AMDL(group) => {
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::AMDL(item));
                    }
                }
                TopGroup::AMMO(group) => {
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::AMMO(item));
                    }
                }
                TopGroup::ANIO(group) => {
                    for item in group.data {
                        records.insert(item.header.form_id.clone(), SingleRecord::ANIO(item));
                    }
                }

                _ => {
                    println!("Unhandled group variant");
                }

            }
        }

        Ok( Self { header: esm.header, records} )
    }

}


// ================================================================================



pub struct MappedESM {
    pub header: FileHeader,
    pub indices: HashMap<FormId, SingleRecordRef>,
    pub game_settings: HashMap<FormId, Rc<GameSetting>>,
}


impl From<ESMFull> for MappedESM {
    fn from(value: ESMFull) -> Self {
        
        let header = value.header;
        let mut indices = HashMap::new();
        let mut game_settings = HashMap::new();


        for group in value.groups {

            match group {
                TopGroup::GMST(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gs = Rc::new(record);
                        let gr = SingleRecordRef::GMST(gs.clone());
                        game_settings.insert(form_id.clone(), gs);
                        indices.insert(form_id.clone(), gr);
                    }
                }

                _ => {}
                
            }
        }

        Self { header, indices, game_settings}
    }
}