use std::collections::HashMap;

use crate::{dev::*, groups::prelude::{RawTopGroup, TopGroup}, prelude::MapContents, records::SingleRecord};

// TODO: this is wrong, need to make a generic group enum that contains TopGroup
pub enum ESMObject {
    Record(SingleRecord),
    Group(TopGroup),
}

// ====================================================================================================

// This is slower but far more flexible
pub enum ESMRawObject<'esm> {
    Record(RawRecord<'esm>),
    Group(Group<ESMRawObject<'esm>>)
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for ESMRawObject<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (_, iden) = FourCC::parse(i)?;

        if &iden.0 == b"GRUP" {
            let (i, group) = <Group<ESMRawObject>>::parse(i)?;
            Ok((i, ESMRawObject::Group(group)))
        } else {
            let (i, record) = RawRecord::parse(i)?;
            Ok((i, ESMRawObject::Record(record)))
        }


    }
}

// ====================================================================================================

impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for ESMRawObject<'esm> {
    fn insert_into_one_map(self, combined_map: &mut HashMap<FormId, RawRecord<'esm>>) {
        match self {
            ESMRawObject::Record(raw_record) => {
                combined_map.insert(raw_record.header.form_id, raw_record);
            },
            ESMRawObject::Group(group) => {
                for obj in group.data {
                    obj.insert_into_one_map(combined_map);
                }
            }
        }
    }
}

// ====================================================================================================