use crate::dev::*;


pub enum ESMObject {
    Record()
}



pub enum ESMRawObject<'esm> {
    Record(RawRecord<'esm>),
    Group(Group<ESMRawObject<'esm>>)
}

impl<'esm> Parse<&'esm[u8]> for ESMRawObject<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (_, iden) = FourCC::parse(i)?;

        if &iden.0 == b"GRUP" {
            let (i, group) = <Group<ESMRawObject>>::parse(i)?;
            Ok((i, ESMRawObject::Group(group)))
        } else {
            let (i, record) = RawRecord::parse(i)?;
            Ok((i, ESMRawObject::Record(record)))
        }


    }
}