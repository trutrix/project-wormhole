use std::{collections::HashMap, fs::File, io::{Read, Seek}, path::PathBuf};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{dev::*, records::{self, SingleRecord, all::*}, structs::{chunk::{get_file_chunks, get_file_chunks2}, group::TopGroup, record::RawRecord, world::WorldEntry}};


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
    pub indices: HashMap<FormId, SingleRecord>,
}


impl From<ESMFull> for MappedESM {
    fn from(value: ESMFull) -> Self {
        
        let header = value.header;
        let mut indices = HashMap::new();


        for group in value.groups {

            match group {
                TopGroup::AACT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::AACT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ACTI(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ACTI(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ADDN(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ADDN(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::AECH(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::AECH(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ALCH(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ALCH(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::AMDL(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::AMDL(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::AMMO(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::AMMO(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ANIO(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ANIO(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::AORU(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::AORU(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ARMA(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ARMA(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ARMO(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ARMO(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ARTO(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ARTO(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ASPC(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ASPC(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ASTP(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ASTP(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::AVIF(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::AVIF(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::BNDS(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::BNDS(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::BOOK(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::BOOK(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::BPTD(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::BPTD(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CAMS(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CAMS(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CELL(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CELL(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CLAS(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CLAS(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CLFM(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CLFM(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CLMT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CLMT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CMPO(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CMPO(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::COBJ(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::COBJ(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::COLL(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::COLL(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CONT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CONT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CPTH(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CPTH(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::CSTY(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::CSTY(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::DEBR(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::DEBR(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::DFOB(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::DFOB(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::DLVW(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::DLVW(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::DMGT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::DMGT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::DOBJ(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::DOBJ(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::DOOR(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::DOOR(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ECZN(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ECZN(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::EFSH(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::EFSH(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::ENCH(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::ENCH(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::EQUP(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::EQUP(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::EXPL(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::EXPL(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::FACT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::FACT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::FLOR(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::FLOR(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::FLST(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::FLST(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::FSTP(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::FSTP(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::FSTS(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::FSTS(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::FURN(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::FURN(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::GDRY(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::GDRY(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::GLOB(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::GLOB(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::GMST(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::GMST(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::GRAS(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::GRAS(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::HAZD(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::HAZD(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::HDPT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::HDPT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::IDLE(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::IDLE(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::INGR(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::INGR(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::IPCT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::IPCT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::IPDS(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::IPDS(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::KEYM(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::KEYM(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::KYWD(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::KYWD(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::LCRT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::LCRT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::LCTN(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::LCTN(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::LIGH(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::LIGH(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::LSCR(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::LSCR(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::LTEX(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::LTEX(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::LVLI(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::LVLI(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::LVLN(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::LVLN(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MATO(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MATO(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MATT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MATT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MESG(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MESG(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MGEF(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MGEF(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MISC(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MISC(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MOVT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MOVT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MSTT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MSTT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::MUSC(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::MUSC(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::NPC_(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::NPC_(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::OTFT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::OTFT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::PACK(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::PACK(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::PERK(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::PERK(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::PROJ(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::PROJ(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::QUST(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::QUST(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::RACE(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::RACE(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::REGN(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::REGN(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::RELA(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::RELA(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::SNCT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::SNCT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::SOPM(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::SOPM(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::SOUN(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::SOUN(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::SPGD(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::SPGD(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::STAT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::STAT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::TACT(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::TACT(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::TREE(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::TREE(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::TXST(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::TXST(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::VTYP(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::VTYP(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::WATR(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::WATR(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::WEAP(g) => {
                    for record in g.data {
                        let form_id = record.header.form_id.clone();
                        let gr = SingleRecord::WEAP(record);
                        indices.insert(form_id, gr);
                    }
                }
                TopGroup::Unhandled(group_vec) => todo!(),
                TopGroup::IDLM(group_vec) => todo!(),
                TopGroup::IMAD(group_vec) => todo!(),
                TopGroup::IMGS(group_vec) => todo!(),
                TopGroup::INNR(group_vec) => todo!(),
                TopGroup::KSSM(group_vec) => todo!(),
                TopGroup::LAYR(group_vec) => todo!(),
                TopGroup::LENS(group_vec) => todo!(),
                TopGroup::LGTM(group_vec) => todo!(),
                TopGroup::MSWP(group_vec) => todo!(),
                TopGroup::MUST(group_vec) => todo!(),
                TopGroup::NAVI(group_vec) => todo!(),
                TopGroup::NOCM(group_vec) => todo!(),
                TopGroup::NOTE(group_vec) => todo!(),
                TopGroup::OMOD(group_vec) => todo!(),
                TopGroup::OVIS(group_vec) => todo!(),
                TopGroup::PKIN(group_vec) => todo!(),
                TopGroup::REVB(group_vec) => todo!(),
                TopGroup::RFCT(group_vec) => todo!(),
                TopGroup::RFGP(group_vec) => todo!(),
                TopGroup::SCCO(group_vec) => todo!(),
                TopGroup::SCOL(group_vec) => todo!(),
                TopGroup::SCSN(group_vec) => todo!(),
                TopGroup::SMBN(group_vec) => todo!(),
                TopGroup::SMEN(group_vec) => todo!(),
                TopGroup::SMQN(group_vec) => todo!(),
                TopGroup::SNDR(group_vec) => todo!(),
                TopGroup::SPEL(group_vec) => todo!(),
                TopGroup::STAG(group_vec) => todo!(),
                TopGroup::TERM(group_vec) => todo!(),
                TopGroup::TRNS(group_vec) => todo!(),
                TopGroup::WRLD(group_vec) => todo!(),
                TopGroup::WTHR(group_vec) => todo!(),
                TopGroup::ZOOM(group_vec) => todo!(),
            }
        }

        Self { header, indices }
    }
}