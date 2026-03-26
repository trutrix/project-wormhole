use std::collections::HashMap;

use rayon::{iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator}, slice::ParallelSlice};

use crate::{dev::*, esm::ESMParseMode, groups::prelude::RawInteriorCellBlock, prelude::MapContents, records::all::FileHeader, structs::chunk::{get_file_chunks, get_file_chunks2}};


/// This is a barebones parsing of an ESM file.  
/// It does not attempt to interpret any records or fields.  
/// It simply breaks the file into its constituent groups and records.  
/// This is useful for debugging and for understanding the structure of the file. 
/// More advanced parsing can be built on top of this.

#[derive(Debug)]
pub struct RawESM<'esm> {
    pub header: FileHeader,
    pub records: HashMap<FormId, RawRecord<'esm>>,
    pub references: HashMap<FormId, RawRecord<'esm>>,
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
                            let (i, gw) = RawWorldGroup::parse(raw)?;
                            raw = i;
                            
                            for world in gw.worlds {
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

        Ok((i, Self { header, records, references }))
    }


    pub fn parse_mt(i: &'esm[u8], parse_mode: ESMParseMode) -> IResult<&'esm[u8], Self> {

        // Initialize maps
        let mut records = HashMap::new();
        let mut references = HashMap::new();

        // Get chunks for multithread
        let (_, (data_chunks, refr_chunks)) = get_file_chunks2(i)?;

        // Full parse file header
        let (i, header) = FileHeader::parse(data_chunks[0].data)?;

        

        rayon::scope(|s| {
            s.spawn(|_| {
                let start = std::time::Instant::now();
                for chunk in data_chunks.iter().skip(1) {
                    let (_, (_, raw)) = alloc_group(chunk.data).expect("Could not allocate group.");
                    let (_, data_records) = many0(RawRecord::parse)(raw).expect("Could not parse raw records.");
                    for record in data_records {
                        records.insert(record.header.form_id, record);
                    }
                }
                println!("Data parse times: {:?}", start.elapsed());
            });

            s.spawn(|_| {
                for chunk in refr_chunks {
                    let (_, (gh, raw)) = alloc_group(chunk.data).expect("Could not allocate group.");

                    match gh.label {
                        // Top group verified (sort of), proceed to parsers
                        GroupLabel::Top(iden) => {
                            match &iden.0 {

                                b"CELL" => {

                                }

                                b"WRLD" => {

                                }

                                b"QUST" => {
                                    
                                }


                                _ => {
                                    panic!("Unexpected top group encountered {:?}", gh.label);
                                }
                            }
                        }

                        _ => {
                            panic!("Unexpected group encountered {:?}", gh.label);
                        }
                    }
                }
            });
        });

        
        Ok((i, Self { header, records, references }))
        

        // Loop until buffer is empty
        // while !raw.is_empty() {

        //     // Get the next group header
        //     // Note: We are assuming all top level structures should be groups
        //     let (_, gh) = GroupHeader::parse(raw)?;
            
        //     // Make sure the group label is correct
        //     match gh.label {

        //         // Top group verified (sort of), proceed to parsers
        //         GroupLabel::Top(iden) => {
        //             match &iden.0 {

        //                 // Contains InteriorCellBlock list
        //                 b"CELL" => {
        //                     let (i, (_ghead, graw)) = alloc_group(raw)?;
        //                     raw = i;
        //                     let (_, icb) = many0(RawInteriorCellBlock::parse)(graw)?;

        //                     for block in icb {
        //                         for sub_block in block.sub_blocks {
        //                             for record in sub_block.data {
        //                                 record.insert_into_two_maps(&mut records, &mut references);
        //                             }
        //                         }
        //                     }
        //                 }

        //                 // Contains WRLD records with accompanying WorldChildren groups
        //                 b"WRLD" => {
        //                     let (i, gw) = RawWorldGroup::parse(raw)?;
        //                     raw = i;
                            
        //                     for world in gw.worlds {
        //                         if world.has_children() {
        //                             let children = world.world_children.unwrap();
                                    
        //                             records.insert(children.cell.cell.header.form_id, children.cell.cell);

        //                             for block in children.blocks {
        //                                 for sub_block in block.sub_blocks {
        //                                     for cell in sub_block.cells {
        //                                         cell.insert_into_two_maps(&mut records, &mut references);
        //                                     }
        //                                 }
        //                             }

        //                         }
        //                     }
        //                 }

        //                 // TODO: Figure out whats in this group
        //                 // Appears to be different than CELL and WRLD
        //                 b"QUST" => {
        //                     // Allocate and skip for now
        //                     let (i, _gq) = RawQuestGroup::parse(raw)?;
        //                     raw = i;

        //                     // #[cfg(debug_assertions)]
        //                     // eprintln!("Skipping unsupported QUST group.");
        //                 }

        //                 // Every other group appears to be data records, 
        //                 _ => {
        //                     let (i, rg) = RawGroup::parse(raw)?;
        //                     raw = i;
        //                     for r in rg.data {
        //                         records.insert(r.header.form_id, r);
        //                     }
        //                 }
        //             }
        //         }
        //         _ => {
        //             // Parsing will for sure fail, panic for now, maybe pass result in later iteration
        //             panic!("Encountered non-top group in RawESM")
        //         }
        //     }


        // }

        
    }
}

// ====================================================================================================