use std::{collections::HashMap, fs::File, io::{Read, Seek}, path::PathBuf};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{dev::*, prelude::{FormIdTrait, RecordTraits}, records::{self, SingleRecord, all::*}, structs::{chunk::{get_file_chunks, get_file_chunks2}, group::TopGroup, record::RawRecord, world::WorldEntry}};

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

impl PartialEq for RawESM<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
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

impl ESMUtils for MappedESM {
    fn load_file(file_path: &str) -> Result<Self, ESMError> where Self: Sized {
        let mut file = File::open(file_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(MappedESM::parse(&buf)?)
    }

    fn load_dir(dir_path: &str) -> Result<Self, ESMError> where Self: Sized {
        todo!()
    }

    fn append<T>(&mut self, other: &T) {
        todo!()
    }

    fn parse(i: &[u8]) -> Result<Self, ESMError> where Self: Sized {
        let (_, esm) = ESMFull::parse_mt(i).map_err(|_| ESMError::InvalidGroup)?;
        Ok(MappedESM::from(esm))
    }
}


impl From<ESMFull> for MappedESM {
    fn from(value: ESMFull) -> Self {
        
        let header = value.header;
        let mut indices = HashMap::new();

        fn iter_insert_records<T: FormIdTrait + Into<SingleRecord>>(indices: &mut HashMap<FormId, SingleRecord>, records: Vec<T>) {
            for record in records {
                let form_id = record.get_form_id().clone();
                let sr: SingleRecord = record.into();
                indices.insert(form_id, sr);
            }
        }


        for group in value.groups {
            match group {
                TopGroup::Unhandled(group_vec) => { panic!("Unhandled group: {:?}", group_vec.header.label ) },
                TopGroup::AACT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ACTI(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ADDN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AECH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ALCH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AMDL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AMMO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ANIO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AORU(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ARMA(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ARMO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ARTO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ASPC(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ASTP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AVIF(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::BNDS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::BOOK(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::BPTD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CAMS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                //TopGroup::CELL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CLAS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CLFM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CLMT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CMPO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::COBJ(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::COLL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CONT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CPTH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CSTY(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DEBR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DFOB(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DLVW(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DMGT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DOBJ(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DOOR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ECZN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::EFSH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ENCH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::EQUP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::EXPL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FACT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FLOR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FLST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FSTP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FSTS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FURN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GMST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GDRY(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GLOB(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GRAS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::HAZD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::HDPT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IDLE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IDLM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IMAD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IMGS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::INGR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::INNR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IPCT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IPDS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::KEYM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::KYWD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::KSSM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LAYR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LCRT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LCTN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LENS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LGTM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LIGH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LSCR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LTEX(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LVLI(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LVLN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MATO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MATT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MESG(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MGEF(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MISC(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MOVT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MSTT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MSWP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MUSC(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MUST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NAVI(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NOCM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NOTE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NPC_(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::OMOD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::OTFT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::OVIS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PACK(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PERK(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PKIN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PROJ(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::QUST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RACE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::REGN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RELA(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::REVB(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RFCT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RFGP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SCCO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SCOL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SCSN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SMBN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SMEN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SMQN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SNCT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SNDR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SOPM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SOUN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SPEL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SPGD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::STAG(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::STAT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TACT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TERM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TREE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TRNS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TXST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::VTYP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::WATR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::WEAP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                // TopGroup::WRLD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::WTHR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ZOOM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                _=> {}
            }
        }

        Self { header, indices }
    }
}