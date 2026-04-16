use std::collections::HashMap;

use crate::{dev::*, prelude::MapContents, records::SingleRecord};

// TODO: this is wrong, need to make a generic group enum that contains TopGroup
pub enum ESObject {
    Record(SingleRecord),
    Group(Group<ESObject>),
}

// ====================================================================================================

// impl<'esm> Parse<&'esm[u8]> for ESObject {
//     fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
//         let (_, iden) = FourCC::parse(i)?;

//         if &iden.0 == b"GRUP" {
//             let (i, group) = <Group<ESObject>>::parse(i)?;
//             Ok((i, ESObject::Group(group)))
//         } else {
//             let (i, record) = SingleRecord::parse(i)?;
//             Ok((i, ESObject::Record(record)))
//         }
//     }
// }

// ====================================================================================================

// This is slower but far more flexible
pub enum RawESObject<'esm> {
    Record(RawRecord<'esm>),
    Group(Group<RawESObject<'esm>>)
}

// ====================================================================================================

impl RawESObject<'_> {
    pub fn get_object_count(&self) -> usize {

        let mut count = 1;

        match self {
            RawESObject::Record(_) => count,
            RawESObject::Group(group) => {
                for item in &group.data {
                    count += item.get_object_count();
                }
                count
            },
        }
    }
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for RawESObject<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        //let (_, iden) = <[u8;4]>::parse(i)?;

        if &[i[0], i[1], i[2], i[3]] == b"GRUP" {
            let (i, group) = <Group<RawESObject>>::parse(i)?;
            Ok((i, RawESObject::Group(group)))
        } else {
            let (i, record) = RawRecord::parse(i)?;
            Ok((i, RawESObject::Record(record)))
        }
    }
}

// ====================================================================================================

impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for RawESObject<'esm> {
    fn insert_into_one_map(self, combined_map: &mut HashMap<FormId, RawRecord<'esm>>) {
        match self {
            RawESObject::Record(raw_record) => {
                
                #[cfg(not(debug_assertions))]
                combined_map.insert(raw_record.header.form_id, raw_record);

                #[cfg(debug_assertions)]
                if let Some(item) = combined_map.insert(raw_record.header.form_id, raw_record) {
                    println!("Warning: a duplicate overwrote itself in the same file. {:?}", item.header)
                }
            },
            RawESObject::Group(group) => {
                for obj in group.data {
                    obj.insert_into_one_map(combined_map);
                }
            }
        }
    }
}

// ====================================================================================================

impl std::fmt::Debug for RawESObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawESObject::Record(record) => write!(f, "Record: {:?}", record.header.iden),
            RawESObject::Group(group) => write!(f, "Group: {:?}", group.header.label),
        }
    }
}