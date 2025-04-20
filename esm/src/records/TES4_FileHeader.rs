use std::fmt::write;

use nom_derive::nom::combinator::complete;
use proc::define_record;

use crate::{dev::*, esm::ESMError, structs::{field::{Field, FieldHeader}, record::{RawRecord, Record}}, traits::FieldParser};


// pub type FileHeader = Record<Field<FileHeaderField>>;

// impl TryFrom<RawRecord<'_>> for FileHeader {
//     type Error = ESMError;
//     fn try_from(value: RawRecord<'_>) -> Result<Self, Self::Error> {
//         let (_, fields) = many0(complete(FileHeaderField::parse_field))(value.data).expect("Failed to convert FileHeader from RawRecord");
//         Ok(Self { header: value.header, fields })
//     }
// }


// #[derive(Debug)]
// pub enum FileHeaderField {
//     InteriorCellCount(u32),
//     TagificationCount(u32),
//     Author(String),
//     Unknown(FourCC, usize)
// }


// impl FieldParser<FileHeaderField> for FileHeaderField {

//     fn parse_field_body(i: &[u8], header: FieldHeader) -> IResult<&[u8], FileHeaderField, nom::error::Error<&[u8]>> {
//         match &header.iden().0 {
//             b"INCC" => {
//                 let (i, count) = le_u32(i)?;
//                 Ok((i, FileHeaderField::InteriorCellCount(count)))
//             }
//             _ => {
//                 let (i, _data) = take(header.size())(i)?;
//                 Ok((i, FileHeaderField::Unknown(*header.iden(), header.size())))
//             }
//         }
//     }
// }


define_record! {
    b"TES4",
    FileHeader, [
        b"INCC", InteriorCellCount, u32;
        b"INTV", AvailableTags, u32;
        b"HEDR", Metadata, FileHeaderMetadata;
        b"CNAM", Author, ESMString;
        b"SNAM", Description, ESMString; // TODO: Check if current
        b"ONAM", OverriddenForms, Vec<FormId>;
        b"TNAM", TransientItems, FileHeaderTransientItems;
    ]
}


#[derive(Debug, NomLE)]
pub struct FileHeaderMetadata {
    pub version: f32,
    pub object_count: u32,
    pub next_object_id: u32
}

#[derive(NomLE)]
pub struct FileHeaderTransientItems {
    pub type_: u32,
    pub ids: Vec<FormId>
}

impl std::fmt::Debug for FileHeaderTransientItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHeaderTransientItems {{ type_: {}, ids: {} items }}", self.type_, self.ids.len())
    }
}