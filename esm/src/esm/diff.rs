use std::collections::{HashMap, HashSet};
use crate::{dev::*, records::all::FileHeader};

// ====================================================================================================

pub struct ESMDiff<'esm> {
    pub header: FileHeader,
    pub data_records: HashMap<FormId, RawRecord<'esm>>,
    pub cells: HashMap<FormId, RawCellRecord<'esm>>
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for ESMDiff<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        

        let (i, header) = FileHeader::parse(i)?;
        let mut records = HashMap::new();
        let mut cells = HashMap::new();

        let mut raw = i;

        while raw.len() > 0 {
            let (i, (gh, gd)) = alloc_group(raw)?;
            raw = i;


            match gh.label {
                GroupLabel::Top(label) => {
                    match &label.0 {
                        b"WRLD" | b"QUST" => {
                        //     let (_, recs) = many0(RawRecord::parse)(graw)?;
                        //     for r in recs {
                        //         records.insert(r.header.form_id, r);
                        //     }
                        }
                        b"CELL" => {
                            let (i, blocks) = many0(RawInteriorCellBlock::parse)(gd)?;

                            #[cfg(debug_assertions)]
                            if i.len() != 0 {
                                panic!("Not all bytes consumed for CELL group: {} bytes left", i.len());
                            }

                            for block in blocks {

                                //println!("{:?}: contains {} sub blocks", block.header.label, block.sub_blocks.len());

                                for sub_block in block.sub_blocks {
                                    //println!("  {:?}: Contains {} cell records", sub_block.header.label, sub_block.data.len());
                                    for record in sub_block.data {
                                        cells.insert(record.cell.header.form_id.clone(), record);
                                    }
                                }
                            }
                        }
                        _ => {
                            let (_, recs) = many0(RawRecord::parse)(gd)?;
                            for r in recs {
                                records.insert(r.header.form_id.clone(), r);
                            }
                        }
                    }


                }
                _ => {
                    panic!("Unexpected group label: {:?}", gh.label);
                }
                
            }
        }


        Ok((i, ESMDiff { 
            header,
            data_records: records,
            cells
        }))
    }
}

// ====================================================================================================

impl ESMDiff<'_> {
    
    

}

// ====================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ESMParseMode {
    Full,
    DataOnly,
    ReferenceOnly
}

// ====================================================================================================

pub fn get_diff_form_ids(new_esm: &ESMDiff, old_esm: &mut ESMDiff<'_>) -> ESMDiffResult {

    let mut result = ESMDiffResult::default();

    result.header_modified = new_esm.header != old_esm.header;
    
    for (self_id, self_record) in &new_esm.data_records {

        if let Some(other_record) = old_esm.data_records.get(self_id) {
            if self_record != other_record {
                result.modified.insert(self_id.clone());
            } else {
                result.unchanged.insert(self_id.clone());
            }
            old_esm.data_records.remove(self_id);
        } else {
            result.additions.insert(self_id.clone());
        }
    }

    for (leftover, _) in &old_esm.data_records {
        result.deletions.insert(leftover.clone());
    }

    result
}

// ====================================================================================================

#[derive(Debug)]
pub struct ESMDiffResult {
    pub header_modified: bool,
    pub additions: HashSet<FormId>,
    pub deletions: HashSet<FormId>,
    pub modified: HashSet<FormId>,
    pub unchanged: HashSet<FormId>
}

// ====================================================================================================

impl Default for ESMDiffResult {
    fn default() -> Self {
        Self {
            header_modified: false,
            additions: HashSet::new(),
            deletions: HashSet::new(),
            modified: HashSet::new(),
            unchanged: HashSet::new()
        }
    }
}

// ====================================================================================================

impl ESMDiffResult {
    pub fn print_summary(&self) {
        println!("Header Modified: {:?}", self.header_modified);
        println!("Additions: {:?}", self.additions.len());
        println!("Deletions: {:?}", self.deletions.len());
        println!("Modified {:?}", self.modified.len());
        println!("Unchanged: {:?}", self.unchanged.len());
    }
}