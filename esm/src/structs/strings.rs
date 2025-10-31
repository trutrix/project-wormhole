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

#[derive(PartialEq, Eq, Clone)]
pub struct SizedString32(pub String);

impl Parse<&[u8]> for SizedString32 {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, len) = le_u32(i)?;
        let (i, s) = take(len)(i)?;
        let s = String::from_utf8_lossy(s).to_string().replace('\0', "");
        Ok((i, SizedString32(s)))
    }
}

impl SizedString32 {
    pub fn parse_empty_as_none(i: &[u8]) -> IResult<&[u8], Option<String>> {
        let (i, result) = Self::parse(i)?;
        if result.0.len() == 0 {
            Ok((i, None))
        } else {
            Ok((i, Some(result.0)))
        }
    }
}

impl std::fmt::Display for SizedString32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for SizedString32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}


// ====================================================================================================


#[derive(PartialEq, Eq, Clone)]
pub struct SizedString16(pub String);

impl Parse<&[u8]> for SizedString16 {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, len) = le_u16(i)?;
        let (i, s) = take(len)(i)?;
        let s = String::from_utf8_lossy(s).to_string().replace('\0', "");
        Ok((i, SizedString16(s)))
    }
}

impl std::fmt::Display for SizedString16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for SizedString16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

pub fn parse_ss16(i: &[u8]) -> IResult<&[u8], String> {
    let (i, len) = nom::number::complete::le_u16(i)?;
    let (i, s) = nom::bytes::complete::take(len)(i)?;
    Ok((i, String::from_utf8_lossy(s).to_string()))
}

// ====================================================================================================

#[derive(PartialEq, Eq)]
pub struct SizedString8(pub String);

impl Parse<&[u8]> for SizedString8 {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, len) = le_u8(i)?;
        let (i, s) = take(len)(i)?;
        let s = String::from_utf8_lossy(s).to_string().replace('\0', "");
        Ok((i, SizedString8(s)))
    }
}

impl std::fmt::Display for SizedString8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for SizedString8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}