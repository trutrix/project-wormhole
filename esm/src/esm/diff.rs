use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::{dev::*, esm::RawESM, records::all::FileHeader};



pub struct ESMDiff<'esm> {
    pub header: FileHeader,
    pub records: HashMap<FormId, RawRecord<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for ESMDiff<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, raw) = RawESM::parse(i)?;

        Ok((i, ESMDiff { 
            header: raw.header,
            records: raw.records
        }))
    }
}


impl ESMDiff<'_> {
    
    

}

pub fn get_diff_form_ids(new_esm: &ESMDiff, old_esm: &mut ESMDiff<'_>) -> ESMDiffResult {

    let mut result = ESMDiffResult::default();

    result.header_changed = new_esm.header != old_esm.header;
    
    for (self_id, self_record) in &new_esm.records {

        if let Some(other_record) = old_esm.records.get(self_id) {
            if self_record != other_record {
                result.changed.insert(self_id.clone());
            } else {
                result.same.insert(self_id.clone());
            }
            old_esm.records.remove(self_id);
        } else {
            result.additions.insert(self_id.clone());
        }
    }

    for (leftover, _) in &old_esm.records {
        result.deletions.insert(leftover.clone());
    }

    result
}


#[derive(Debug)]
pub struct ESMDiffResult {
    pub header_changed: bool,
    pub additions: HashSet<FormId>,
    pub deletions: HashSet<FormId>,
    pub changed: HashSet<FormId>,
    pub same: HashSet<FormId>
}

impl Default for ESMDiffResult {
    fn default() -> Self {
        Self {
            header_changed: false,
            additions: HashSet::new(),
            deletions: HashSet::new(),
            changed: HashSet::new(),
            same: HashSet::new()
        }
    }
}

impl ESMDiffResult {
    pub fn print_summary(&self) {
        println!("Header Changed: {:?}", self.header_changed);
        println!("Additions: {:?}", self.additions.len());
        println!("Deletions: {:?}", self.deletions.len());
        println!("Changed: {:?}", self.changed.len());
        println!("Same: {:?}", self.same.len());
    }
}