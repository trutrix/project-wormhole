use std::{collections::HashMap, sync::{Arc, Mutex}};

use rayon::{iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator}, slice::ParallelSlice};

use crate::{dev::*, esm::ESMParseMode, groups::prelude::{InteriorCellBlock, RawInteriorCellBlock, RawTopGroup}, prelude::MapContents, records::all::FileHeader, structs::chunk::{get_file_chunks, get_file_chunks2}};


/// This is a barebones parsing of an ESM file.  
/// It does not attempt to interpret any records or fields.  
/// It simply breaks the file into its constituent groups and records.  
/// This is useful for debugging and for understanding the structure of the file. 
/// More advanced parsing can be built on top of this.

#[derive(Debug)]
pub struct RawESM<'esm> {
    pub header: FileHeader,
    pub data_map: HashMap<FormId, RawRecord<'esm>>,
    pub refr_map: HashMap<FormId, RawRecord<'esm>>,
}

// ====================================================================================================

impl<'esm> RawESM<'esm> {
    pub fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {

        // Initialize maps
        let mut records = HashMap::new();
        let mut references = HashMap::new();

        // Full parse file header
        let (i, header) = FileHeader::parse(i)?;

        // Init top level buffer
        let mut raw = i;

        // Loop until buffer is empty
        while !raw.is_empty() {

            // Get the next group header
            // Note: We are assuming all top level structures should be groups
            let (_, gh) = GroupHeader::parse(raw)?;
            
            // Make sure the group label is correct
            match gh.label {

                // Top group verified (sort of), proceed to parsers
                GroupLabel::Top(iden) => {
                    match &iden.0 {

                        // Contains InteriorCellBlock list
                        b"CELL" => {
                            let (i, (_ghead, graw)) = alloc_group(raw)?;
                            raw = i;
                            let (_, icb) = many0(RawInteriorCellBlock::parse)(graw)?;

                            for block in icb {
                                for sub_block in block.sub_blocks {
                                    for record in sub_block.data {
                                        record.insert_into_two_maps(&mut records, &mut references);
                                    }
                                }
                            }
                        }

                        // Contains WRLD records with accompanying WorldChildren groups
                        b"WRLD" => {
                            //let start = std::time::Instant::now();
                            let (i, gw) = RawWorldGroup::parse(raw)?;
                            //println!("Worlds parse time: {:?}", start.elapsed());
                            raw = i;
                            
                            for world in gw.data {
                                if world.has_children() {
                                    let children = world.world_children.unwrap();
                                    
                                    records.insert(children.cell.cell.header.form_id, children.cell.cell);

                                    for block in children.blocks {
                                        for sub_block in block.sub_blocks {
                                            for cell in sub_block.cells {
                                                cell.insert_into_two_maps(&mut records, &mut references);
                                            }
                                        }
                                    }

                                }
                            }
                        }

                        // TODO: Figure out whats in this group
                        // Appears to be different than CELL and WRLD
                        b"QUST" => {
                            // Allocate and skip for now
                            let (i, _gq) = RawQuestGroup::parse(raw)?;
                            raw = i;

                            // #[cfg(debug_assertions)]
                            // eprintln!("Skipping unsupported QUST group.");
                        }

                        // Every other group appears to be data records, 
                        _ => {
                            let (i, rg) = RawGroup::parse(raw)?;
                            raw = i;
                            for r in rg.data {
                                records.insert(r.header.form_id, r);
                            }
                        }
                    }
                }
                _ => {
                    // Parsing will for sure fail, panic for now, maybe pass result in later iteration
                    panic!("Encountered non-top group in RawESM")
                }
            }


        }

        Ok((i, Self { header, data_map: records, refr_map: references }))
    }



    pub fn parse_mt(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {

        // Initialize maps
        let mut data_map = HashMap::new();
        let mut refr_map = HashMap::new();

        // Get chunks for multithread
        let (i, chunks) = get_file_chunks(i)?;

        // Full parse file header
        let (_, header) = FileHeader::parse(chunks[0].data)?;

        let groups: Vec<RawTopGroup> = chunks.par_iter().skip(1).map(|x| {
            let (_, header) = GroupHeader::parse(x.data).unwrap();
            
            if let Ok((_, g)) = RawTopGroup::parse(x.data) {
                g
            } else {
                panic!("Failed parsing group: {:?}", header);
            }
        }).collect();

        for group in groups {
            match group {
                RawTopGroup::Common(raw_records) => {
                    for record in raw_records {

                        #[cfg(debug_assertions)]
                        if let Some(result) = data_map.insert(record.header.form_id, record) {
                            panic!("A record tried to overwrite itself in the same file. {:?}", result.header);
                        }


                        #[cfg(not(debug_assertions))]
                        data_map.insert(record.header.form_id, record);
                    }
                }
                RawTopGroup::Quest(raw_quest_records) => {

                    for quest_entry in raw_quest_records {
                        if let Some(quest_children) = quest_entry.quest_children {
                            for quest_child in quest_children.items {
                                match quest_child {
                                    crate::groups::prelude::RawCellVisibleDistantChild::Dialog(raw_dialog) => {
                                        

                                        data_map.insert(raw_dialog.record.header.form_id, raw_dialog.record);
                                    },
                                    crate::groups::prelude::RawCellVisibleDistantChild::DialogBranch(raw_record) => {
                                        data_map.insert(raw_record.header.form_id, raw_record);
                                    },
                                    crate::groups::prelude::RawCellVisibleDistantChild::Scene(raw_record) => {
                                        data_map.insert(raw_record.header.form_id, raw_record);
                                    }
                                }
                            }
                        }
                        data_map.insert(quest_entry.quest.header.form_id, quest_entry.quest);
                    }

                }
                RawTopGroup::World(raw_world_records) => {
                    for world in raw_world_records {
                        if world.has_children() {
                            let children = world.world_children.unwrap();
                            
                            data_map.insert(children.cell.cell.header.form_id, children.cell.cell);

                            for block in children.blocks {
                                for sub_block in block.sub_blocks {
                                    for cell in sub_block.cells {
                                        cell.insert_into_two_maps(&mut data_map, &mut refr_map);
                                    }
                                }
                            }

                        }
                    }
                }
                RawTopGroup::Cell(raw_interior_cell_blocks) => {
                    for block in raw_interior_cell_blocks {
                        for sub_block in block.sub_blocks {
                            for record in sub_block.data {
                                record.insert_into_two_maps(&mut data_map, &mut refr_map);
                            }
                        }
                    }
                }
            }
        }

        Ok((i, Self { header, data_map, refr_map }))
    }
}

// ====================================================================================================



pub struct ESMRaw<'esm> {
    pub header: FileHeader,
    pub data_map: HashMap<FormId, RawRecord<'esm>>
}


impl<'esm> ESMRaw<'esm> {
    

    pub fn parse(i: &'esm [u8]) -> IResult<&'esm [u8], Self> {

        // Initialize maps
        let mut data_map = HashMap::new();

        // Get chunks for multithread
        let (i, chunks) = get_file_chunks(i)?;

        // Full parse file header
        let (_, header) = FileHeader::parse(chunks[0].data)?;

        let group_results: Vec<_> = chunks.par_iter().skip(1).map(|x| {
            RawTopGroup::parse(x.data)
        }).collect();

        for group_result in group_results {
            match group_result {
                Ok((_, group)) => {
                    match group {
                        RawTopGroup::Common(raw_records) => {
                            for record in raw_records {

                                #[cfg(debug_assertions)]
                                if let Some(result) = data_map.insert(record.header.form_id, record) {
                                    panic!("A record tried to overwrite itself in the same file. {:?}", result.header);
                                }


                                #[cfg(not(debug_assertions))]
                                data_map.insert(record.header.form_id, record);
                            }
                        }
                        RawTopGroup::Quest(raw_quest_records) => {

                            for quest_entry in raw_quest_records {
                                if let Some(quest_children) = quest_entry.quest_children {
                                    for quest_child in quest_children.items {
                                        match quest_child {
                                            crate::groups::prelude::RawCellVisibleDistantChild::Dialog(raw_dialog) => {
                                
                                                if let Some(topic_children) = raw_dialog.children {
                                                    for tc in topic_children.records {
                                                        data_map.insert(tc.header.form_id, tc);
                                                    }
                                                }


                                                data_map.insert(raw_dialog.record.header.form_id, raw_dialog.record);
                                            },
                                            crate::groups::prelude::RawCellVisibleDistantChild::DialogBranch(raw_record) => {
                                                data_map.insert(raw_record.header.form_id, raw_record);
                                            },
                                            crate::groups::prelude::RawCellVisibleDistantChild::Scene(raw_record) => {
                                                data_map.insert(raw_record.header.form_id, raw_record);
                                            }
                                        }
                                    }
                                }
                                data_map.insert(quest_entry.quest.header.form_id, quest_entry.quest);
                            }

                        }
                        RawTopGroup::World(raw_world_records) => {
                            for world in raw_world_records {
                                if world.has_children() {
                                    let children = world.world_children.unwrap();
                    
                                    data_map.insert(children.cell.cell.header.form_id, children.cell.cell);

                                    for block in children.blocks {
                                        for sub_block in block.sub_blocks {
                                            for cell in sub_block.cells {
                                                cell.insert_into_one_map(&mut data_map);
                                            }
                                        }
                                    }

                                }
                            }
                        }
                        RawTopGroup::Cell(raw_interior_cell_blocks) => {
                            for block in raw_interior_cell_blocks {
                                block.insert_into_one_map(&mut data_map);
                            }
                        }
                    }
                }
                Err(_) => { println!("Failed to parse RawTopGroup") }
            }
        }



        Ok((i, Self { header, data_map }))
    }


}