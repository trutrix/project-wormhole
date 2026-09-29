use crate::dev::*;

#[derive(Debug, NomLE)]
pub struct TopicChildren {

}


#[derive(Debug)]
pub struct RawTopicChildren<'es> {
    pub header: GroupHeader,
    pub records: Vec<RawRecord<'es>>
}


impl<'es> Parse<&'es[u8]> for RawTopicChildren<'es> {
    fn parse(i: &'es[u8]) -> IResult<&'es[u8], Self, nom::error::Error<&'es[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, records) = many0(RawRecord::parse)(raw)?;
        Ok((i, Self { header, records }))
    }
}