use rayon::prelude::*;
use crate::{dev::*, records::all::FileHeader, structs::{chunk::get_file_chunks, es_object::RawESObject}};

// ====================================================================================================

/// This is a barebones parsing of an ESM file.  
/// 
/// It does not attempt to interpret any records or fields.  
/// 
/// It simply breaks the file into its constituent groups and records.  
/// 
/// This is useful for debugging and for understanding the structure of the file. 
/// 
/// More advanced parsing can be built on top of this.

pub struct ESRaw<'esm> {
    pub header: FileHeader,
    pub objects: Vec<RawESObject<'esm>>
    // pub group_counter: u32,
}


impl<'esm> ESRaw<'esm> {
    /// WIP -  Anything over one for the threads parameter just makes the function auto allocate threads, actual thread control is planned for later
    // pub fn parse_v2(i: &'esm [u8], threads: usize) -> IResult<&'esm [u8], Self> {

    //     // let mut group_counter = 0;

    //     // Initialize maps
    //     let mut data_map = HashMap::new();

    //     // Get chunks for multithread
    //     let (i, chunks) = get_file_chunks(i)?;

    //     // Full parse file header
    //     let (_, header) = FileHeader::parse(chunks[0].data)?;

    //     // If thread is just one or zero, parse normally without threads
    //     let group_results: Vec<_> = if threads <= 1 {
    //         chunks.iter().skip(1).map(|x| {
    //             RawTopGroup::parse(x.data)
    //         }).collect()
    //     } 
    //     // If threads over 1, use par_iter() to multithread the parsing
    //     else {
    //         chunks.par_iter().skip(1).map(|x| {
    //             RawTopGroup::parse(x.data)
    //         }).collect()
    //     };

    //     // Iterate through the groups
    //     for group_result in group_results {
    //         match group_result {
    //             Ok((_, group)) => {
    //                 match group {
    //                     RawTopGroup::Common(raw_records) => {
    //                         for record in raw_records {

    //                             #[cfg(debug_assertions)]
    //                             if let Some(result) = data_map.insert(record.header.form_id, record) {
    //                                 panic!("A record tried to overwrite itself in the same file. {:?}", result.header);
    //                             }

    //                             #[cfg(not(debug_assertions))]
    //                             data_map.insert(record.header.form_id, record);
    //                         }
    //                     }
    //                     RawTopGroup::Quest(raw_quest_records) => {

    //                         for quest_entry in raw_quest_records {

    //                             match quest_entry {
    //                                 RawQuestItem::Record(raw_record) => {
    //                                     data_map.insert(raw_record.header.form_id, raw_record);
    //                                 },
    //                                 RawQuestItem::Children(raw_cell_visible_distant_children) => {

    //                                     for child in raw_cell_visible_distant_children.items {
    //                                         match child {
    //                                             RawCellVisibleDistantChild::Dialog(raw_dialog) => {
    //                                                 if let Some(topic_children) = raw_dialog.children {
    //                                                     for child in topic_children.records {
    //                                                         data_map.insert(child.header.form_id, child);
    //                                                     }
    //                                                 }

    //                                                 data_map.insert(raw_dialog.record.header.form_id, raw_dialog.record);
    //                                             },
    //                                             RawCellVisibleDistantChild::DialogBranch(raw_record) => {
    //                                                 data_map.insert(raw_record.header.form_id, raw_record);
    //                                             }
    //                                             RawCellVisibleDistantChild::Scene(raw_record) => {
    //                                                 data_map.insert(raw_record.header.form_id, raw_record);
    //                                             }
    //                                         }
    //                                     }
    //                                 },
    //                             }
    //                         }
    //                     }
    //                     RawTopGroup::World(raw_world_records) => {
    //                         for world in raw_world_records {

    //                             if let Some(children) = world.world_children {
    //                                 data_map.insert(children.cell.cell.header.form_id, children.cell.cell);

    //                                 for block in children.blocks {
    //                                     block.insert_into_one_map(&mut data_map);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     RawTopGroup::Cell(raw_interior_cell_blocks) => {
    //                         for block in raw_interior_cell_blocks {
    //                             block.insert_into_one_map(&mut data_map);
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(_) => { println!("Failed to parse RawTopGroup") }
    //         }
    //     }

    //     Ok((i, Self { header, data_map /*, group_counter */ }))
    // }

    pub fn parse_as_objects(i: &'esm [u8], threads: usize) -> IResult<&'esm [u8], Self> {

        let mut objects = Vec::new();

        let (i, chunks) = get_file_chunks(i)?;

        let (_, header) = FileHeader::parse(chunks[0].data)?;

        // If thread is just one or zero, parse normally without threads
        let groups: Vec<_> = if threads <= 1 {
            chunks.iter().skip(1).map(|x| {
                RawESObject::parse(x.data)
            }).collect()
        } 
        // If threads over 1, use par_iter() to multithread the parsing
        else {
            chunks.par_iter().skip(1).map(|x| {
                RawESObject::parse(x.data)
            }).collect()
        };

        for obj in groups {
            objects.push(obj?.1);
        }

        Ok((i, Self { header, objects }))
    }

    pub fn get_full_object_count(&self) -> usize {
        let mut count = 0;

        for o in &self.objects {
            count += o.get_object_count();
        }

        count
    }
}

// ====================================================================================================