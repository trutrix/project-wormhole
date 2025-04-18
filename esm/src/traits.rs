use nom_derive::nom::IResult;

use crate::{dev::*, structs::field::FieldHeader};


pub trait FieldParser<T> {
    fn parse_field(i: &[u8]) -> IResult<&[u8], (FieldHeader, T), nom::error::Error<&[u8]>>;
}