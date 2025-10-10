use crate::dev::*;


// ====================================================================================================

// Keyword list uses two fields, a count and then a list of FormIDs to bypass the size limit of fields
pub struct KeywordList(pub Vec<FormId>);

impl Parse<&[u8]> for KeywordList {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, real_count) = u32::parse_le(i)?;
        let (i, (_header, _size)) = alloc_field(i)?;
        let (i, raw) = take(real_count * 4)(i)?; // Each FormID is 4 bytes
        let (_, ids) = many0(complete(FormId::parse_le))(raw)?;
        Ok((i, Self(ids)))
        
    }
}