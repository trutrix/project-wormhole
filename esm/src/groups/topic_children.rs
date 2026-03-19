use crate::dev::*;

#[derive(Debug, NomLE)]
pub struct TopicChildren {

}


#[derive(Debug)]
pub struct RawTopicChildren<'esm> {
    pub record: RawRecord<'esm>
}


impl<'esm> Parse<&'esm[u8]> for RawTopicChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (_, first_id) = FourCC::parse(i)?;
        panic!("First topic child id: {:?}", first_id);
    }
}