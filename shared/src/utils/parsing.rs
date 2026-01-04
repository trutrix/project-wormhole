use std::fmt::Display;

use nom_derive::nom::number::complete::le_u8;
use nom_derive::Parse;
use nom_derive::nom;

#[derive(Debug)]
pub struct U8Boolean(pub bool);

impl Display for U8Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parse<&[u8]> for U8Boolean {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, value) = parse_u8_bool(i)?;
        Ok((i, U8Boolean(value)))
    }
}

pub fn parse_u8_bool(i: &[u8]) -> nom::IResult<&[u8], bool, nom::error::Error<&[u8]>> {
    let (i, value) = le_u8(i)?;
    Ok((i, value != 0))
}