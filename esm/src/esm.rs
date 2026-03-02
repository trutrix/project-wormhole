use std::{collections::HashMap, fs::File, io::prelude::*};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{dev::*, prelude::FormIdTrait, records::{SingleRecord, all::*}, structs::{chunk::get_file_chunks, group::TopGroup, record::RawRecord}};

pub mod diff;
pub mod mapped;
pub mod raw;
pub mod full;

// ====================================================================================================


pub trait ESMUtils {
    fn load_file(file_path: &str) -> Result<Self, ESMError> where Self: Sized;
    fn load_dir(dir_path: &str) -> Result<Self, ESMError> where Self: Sized;
    fn append<T>(&mut self, other: &T);
    fn parse(i: &[u8]) -> Result<Self, ESMError> where Self: Sized;
}


// ====================================================================================================







/// A more fully-featured ESM parser that attempts to interpret records and fields
/// This is still a work in progress and is not yet complete
// #[deprecated]
// pub struct SmartESM {
//     pub header: FileHeader,
//     // pub chunks: Vec<TopGroup>,
//     // pub rchunks: Vec<TopGroup>,
//     pub data_groups: Vec<TopGroup>
// }

// impl Parse<&[u8]> for SmartESM {
//     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        
//         let (leftover, (chunks, rchunks)) = get_file_chunks2(i)?;
//         //println!("Chunks: {}, RChunks: {}", chunks.len(), rchunks.len());

//         // Debugging if file has leftover data after parsing chunks
//         #[cfg(debug_assertions)]
//         {
//             if !leftover.is_empty() {
//                 println!("Warning: leftover data after parsing file chunks: {} bytes", leftover.len());
//             }
//             //println!("Parsed {} file chunks", chunks.len());
//         }

//         // First chunk should be the file header
//         let (_, header) = FileHeader::parse(chunks[0].data)?;
//         let mut parsed_data = Vec::new();
//         let mut parsed_refr = Vec::new();

//         rayon::scope(|s|{
//             // Data thread
//             s.spawn(|_|{
//                 //let start = std::time::Instant::now();
//                 for chunk in chunks.iter().skip(1) {
//                     parsed_data.push(TopGroup::parse(chunk.data));
//                 }
//                 //println!("Data groups parse time: {:?}", start.elapsed())
//             });

//             //Refr thread
//             s.spawn(|_|{
//                 //let start = std::time::Instant::now();
//                 for rchunk in rchunks {
//                     parsed_refr.push(TopGroup::parse(rchunk.data));
//                 }
//                 //println!("Refr groups parse time: {:?}", start.elapsed())
//             });

//         });

//         Ok((i, Self { header, data_groups: Vec::new() }) )
//     }
// }


// ====================================================================================================






// ====================================================================================================

#[derive(Debug)]
pub enum ESMError {
    IO(std::io::Error),
    Nom(nom::Err<nom::error::Error<&'static [u8]>>),
    InvalidFile,
    InvalidHeader,
    InvalidRecord,
    InvalidField,
    InvalidGroup,
    InvalidVersionControl,
    InvalidData,
    NotEnoughBytes(String),
    StringConversionError(String),
    GameSetting(String)
}

impl From<std::io::Error> for ESMError {
    fn from(err: std::io::Error) -> Self {
        ESMError::IO(err)
    }
}

impl From<nom::Err<nom::error::Error<&'static[u8]>>> for ESMError {
    fn from(value: nom::Err<nom::error::Error<&'static[u8]>>) -> Self {
        ESMError::Nom(value)
    }
}

// ================================================================================

// use std::rc::Rc;

// pub struct SmartESM2 {
//     pub header: FileHeader,
//     pub records: HashMap<FormId, SingleRecord>
// }

// impl ESMUtils for SmartESM2 {
//     fn load_file(file_path: &str) -> Result<Self, ESMError> where Self: Sized {
//         let mut file = File::open(file_path)?;
//         let mut buf = Vec::new();
//         file.read_to_end(&mut buf)?;
//         Ok(SmartESM2::parse(&buf)?)
//     }
    
//     fn load_dir(dir_path: &str) -> Result<Self, ESMError> where Self: Sized {
//         todo!()
//     }
    
//     fn append<T>(&mut self, other: &T) {
//         todo!()
//     }
    
//     fn parse(i: &[u8]) -> Result<Self, ESMError> where Self: Sized {
//         let (_, esm) = ESMFull::parse_mt(i).map_err(|_| ESMError::InvalidGroup)?;

//         let mut records: HashMap<FormId, SingleRecord> = HashMap::new();

//         for group in esm.groups {

//             match group {
//                 TopGroup::Unhandled(group) => {
//                     //println!("Unhandled group: {:?}", group.header.label);
//                 },
//                 TopGroup::AACT(group) => { 
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::AACT(item)).unwrap();
//                     }
//                 },
//                 TopGroup::ACTI(group) => { 
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::ACTI(item)).unwrap();
//                     }
//                 },
//                 TopGroup::ADDN(group) => { 
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::ADDN(item)).unwrap();
//                     }
//                 },
//                 TopGroup::AECH(group) => {
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::AECH(item));
//                     }
//                 }
//                 TopGroup::ALCH(group) => {
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::ALCH(item));
//                     }
//                 }
//                 TopGroup::AMDL(group) => {
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::AMDL(item));
//                     }
//                 }
//                 TopGroup::AMMO(group) => {
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::AMMO(item));
//                     }
//                 }
//                 TopGroup::ANIO(group) => {
//                     for item in group.data {
//                         records.insert(item.header.form_id.clone(), SingleRecord::ANIO(item));
//                     }
//                 }

//                 _ => {
//                     println!("Unhandled group variant");
//                 }

//             }
//         }

//         Ok( Self { header: esm.header, records} )
//     }

// }






