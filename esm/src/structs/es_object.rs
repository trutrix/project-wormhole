use std::{collections::HashMap, num::NonZero};

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

    pub fn print_header_info(&self, indent: usize) {
        let mut ind = String::new();

        for _ in 0..indent {
            ind.push(' ');
        }

        match self {
            RawESObject::Record(raw_record) => println!("{ind}{} - {}", raw_record.header.iden, raw_record.header.form_id),
            RawESObject::Group(group) => {
                match &group.header.label {
                    GroupLabel::Top(four_cc) => println!("{ind}Top - {}", four_cc),
                    GroupLabel::WorldChildren(form_id) => println!("{ind}WorldChildren -  for {}", form_id),
                    GroupLabel::InteriorCellBlock(index) => println!("{ind}InteriorCellBlock - {}", index),
                    GroupLabel::InteriorCellSubBlock(index) => println!("{ind}InteriorCellSubBlock - {}", index),
                    GroupLabel::ExteriorCellBlock(cell_location) => println!("{ind}ExteriorCellBlock - {:?}", cell_location),
                    GroupLabel::ExteriorCellSubBlock(cell_location) => println!("{ind}ExteriorCellSubBlock - {:?}", cell_location),
                    GroupLabel::CellChildren(form_id) => println!("{ind}CellChildren - for {}", form_id),
                    GroupLabel::TopicChildren(form_id) => println!("{ind}TopicChildren - for {}", form_id),
                    GroupLabel::CellPersistentChildren(form_id) => println!("{ind}CellPersistentChildren - for {}", form_id),
                    GroupLabel::CellTemporaryChildren(form_id) => println!("{ind}CellTemporaryChildren - for {}", form_id),
                    GroupLabel::CellVisibleDistantChildren(form_id) => println!("{ind}CellVisibleDistantChildren - for {}", form_id),
                    GroupLabel::Unknown(bytes) => println!("{ind}Unknown {:?}", bytes),
                }
            }
        }
    }
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for RawESObject<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        //let (_, iden) = <[u8;4]>::parse(i)?;


        if &[i[0], i[1], i[2], i[3]] == b"GRUP" {
            let (i, group) = <Group<RawESObject>>::parse_le(i)?;
            Ok((i, RawESObject::Group(group)))
        } else {
            let (i, record) = RawRecord::parse_le(i)?;
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