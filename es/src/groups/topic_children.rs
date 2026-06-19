use crate::dev::*;

#[derive(Debug, NomLE)]
pub struct TopicChildren {

}


#[derive(Debug)]
pub struct RawTopicChildren<'esm> {
    pub header: GroupHeader,
    pub records: Vec<RawRecord<'esm>>
}


impl<'esm> Parse<&'esm[u8]> for RawTopicChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, records) = many0(RawRecord::parse)(raw)?;
        Ok((i, Self { header, records }))
    }
}