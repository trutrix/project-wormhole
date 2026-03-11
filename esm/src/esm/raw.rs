use std::collections::HashMap;

use crate::{dev::*, groups::prelude::RawInteriorCellBlock, records::all::FileHeader};


/// This is a barebones parsing of an ESM file.  
/// It does not attempt to interpret any records or fields.  
/// It simply breaks the file into its constituent groups and records.  
/// This is useful for debugging and for understanding the structure of the file. 
/// More advanced parsing can be built on top of this.

#[derive(Debug)]
pub struct RawESM<'esm> {
    pub header: FileHeader,
    pub worlds: HashMap<FormId, RawWorldRecord<'esm>>,
    pub records: HashMap<FormId, RawRecord<'esm>>,
    pub quests: Vec<RawQuestGroup<'esm>>,
    pub interior_references: HashMap<FormId, RawRecord<'esm>>,
    pub world_references: HashMap<FormId, RawRecord<'esm>>
}

// ====================================================================================================

impl<'esm> RawESM<'esm> {
    pub fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {
        let mut worlds = HashMap::new();
        let mut records = HashMap::new();
        let mut quests = Vec::new();
        let mut interior_references = HashMap::new();
        let mut world_references = HashMap::new();
        let mut wrc = 0;

        
        let (i, header) = FileHeader::parse(i)?;
        let mut raw = i;

        while !raw.is_empty() {

            let (_, gh) = GroupHeader::parse(raw)?;
            
            match gh.label {
                GroupLabel::Top(iden) => {
                    match &iden.0 {
                        b"CELL" => {
                            let (i, (_ghead, graw)) = alloc_group(raw)?;
                            raw = i;
                            let (_, icb) = many0(RawInteriorCellBlock::parse)(graw)?;

                            for block in icb {
                                for sub_block in block.sub_blocks {
                                    for record in sub_block.data {
                                        if record.has_children() {
                                            let children = record.cell_children.unwrap();
                                            for g in children.data {
                                                for r in g.data {
                                                    if let Some(or) = interior_references.insert(r.header.form_id.clone(), r) {
                                                        panic!("Duplicate reference form id: {}", or.header.form_id);
                                                    }
                                                }
                                            }
                                        }
                                        records.insert(record.cell.header.form_id.clone(), record.cell);
                                    }
                                }
                            }
                        }
                        b"WRLD" => {
                            // println!("Parsing {:?}", gh.label);
                            let (i, gw) = RawWorldGroup::parse(raw)?;
                            raw = i;
                            

                            for world in gw.worlds {
                                worlds.insert(world.world.header.form_id.clone(), world);
                            }
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

        Ok((i, Self { header, worlds, records, quests, interior_references, world_references }))
    }

    
}

// ====================================================================================================