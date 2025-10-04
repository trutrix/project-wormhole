use crate::dev::*;




pub type LocalizedString = u32;


// ====================================================================================================

// Basically a CString
// Custom implementation to avoid foreign parsing errors
// TODO: Make this a proper CString
#[derive(Debug)]
pub struct ESMString(pub String);

impl Parse<&[u8]> for ESMString {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (_, left) = take(i.len()-1)(i)?;
        let s = String::from_utf8_lossy(left).to_string();
        Ok((&[], Self(s)))
    }
}

impl std::fmt::Display for ESMString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0) 
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct SizedString8(pub String);

impl Parse<&[u8]> for SizedString8 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, size) = le_u8(i)?;
        let (i, raw) = take(size)(i)?;
        let s = String::from_utf8_lossy(raw).to_string();
        Ok((i, Self(s)))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct SizedString16(pub String);

impl Parse<&[u8]> for SizedString16 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, size) = le_u16(i)?;
        let (i, raw) = take(size)(i)?;
        let s = String::from_utf8_lossy(raw).to_string();
        Ok((i, Self(s)))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct SizedString32(pub String);

impl Parse<&[u8]> for SizedString32 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, size) = le_u32(i)?;
        let (i, raw) = take(size)(i)?;
        let s = String::from_utf8_lossy(raw).to_string();
        Ok((i, Self(s)))
    }
}