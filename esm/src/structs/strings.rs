use crate::dev::*;




// Basically a CString
// Custom implementation to avoid foreign parsing errors
#[derive(Debug)]
pub struct ESMString(pub String);

impl Parse<&[u8]> for ESMString {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (_, left) = take(i.len()-1)(i)?;
        let s = String::from_utf8_lossy(left).to_string();
        Ok((&[], Self(s)))
    }
}