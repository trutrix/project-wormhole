

use nom_derive::nom::{IResult, number::complete::{le_u8, le_u16, le_u32}, bytes::complete::take};

pub struct SizedString<T> {
    pub size: T,
    pub value: String
}


impl nom_derive::Parse<&[u8]> for SizedString<u32> {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, size) = le_u32(i)?;
        let (i, value) = take(size)(i)?;

        if let Ok(s) = String::from_utf8(value.to_vec()) {
            Ok((i, SizedString { size, value: s }))
        } else {
            Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
        }
    }
}

impl nom_derive::Parse<&[u8]> for SizedString<u16> {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, size) = le_u16(i)?;
        let (i, value) = take(size)(i)?;

        if let Ok(s) = String::from_utf8(value.to_vec()) {
            Ok((i, SizedString { size, value: s }))
        } else {
            Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
        }
    }
}

impl nom_derive::Parse<&[u8]> for SizedString<u8> {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, size) = le_u8(i)?;
        let (i, value) = take(size)(i)?;

        if let Ok(s) = String::from_utf8(value.to_vec()) {
            Ok((i, SizedString { size, value: s }))
        } else {
            Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
        }
    }
}