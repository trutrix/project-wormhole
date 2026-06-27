use crate::{dev::*, es::{es_group::ESGroup, es_record::ESRecord}};

// ===================================================================================================

#[derive(Debug)]
pub enum ESObject {
    Group(ESGroup),
    Record(ESRecord),
}

// ===================================================================================================

impl<'a, C: speedy::Context> speedy::Readable<'a, C> for ESObject {
    #[inline]
    fn read_from<R: speedy::Reader<'a, C>>(reader: &mut R) -> std::result::Result<Self, C::Error> {
        let kind = reader.peek_u32()?;

        match kind {
            // GRUP
            1196578128 => {
                let t0: ESGroup = reader.read_value()?;
                Ok(ESObject::Group(t0))
            }
            _ => {
                let t0: ESRecord = reader.read_value()?;
                Ok(ESObject::Record(t0))
            }
            _ => Err(speedy::private::error_invalid_enum_variant()),
        }
    }
}

// ===================================================================================================

impl nom_derive::Parse<&[u8]> for ESObject {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        if &[i[0], i[1], i[2], i[3]] == b"GRUP" {
            let (i, group) = ESGroup::parse(i)?;
            Ok((i, ESObject::Group(group)))
        } else {
            let (i, record) = ESRecord::parse(i)?;
            Ok((i, ESObject::Record(record)))
        }
    }
}