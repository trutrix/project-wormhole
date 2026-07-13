use crate::{dev::*, es::{self, es_group::{ESGroup, ESGroupHeader}, es_record::{ESRecord, ESRecordHeader}}};

// ===================================================================================================

#[derive(Debug)]
pub enum ESObject {
    Group(ESGroup),
    Record(ESRecord),
}

// ===================================================================================================

impl ESObject {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (_, iden) = FourCC::parse(i)?;
        if &iden.0 == b"GRUP" {
            let (i, group) = ESGroup::parse(i)?;
            Ok((i, ESObject::Group(group)))
        } else {
            let (i, record) = ESRecord::parse(i)?;
            Ok((i, ESObject::Record(record)))
        }
    }

    /// This function assumes you have checked if there is more data to parse
    fn parse_unchecked(i: &[u8]) -> IResult<&[u8], Self> {
        if &[i[0], i[1], i[2], i[3]] == b"GRUP" {
            let (i, group) = ESGroup::parse(i)?;
            Ok((i, ESObject::Group(group)))
        } else {
            let (i, record) = ESRecord::parse(i)?;
            Ok((i, ESObject::Record(record)))
        }
    }
}

// ===================================================================================================

#[cfg(feature = "speedy")]
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
        ESObject::parse(i)
    }
}

// ===================================================================================================

impl ESHeader<ESGroupHeader> for ESObject {
    fn header(&self) -> &ESGroupHeader {
        match self {
            ESObject::Group(g) => todo!(),
            _ => { panic!("Tried to get wrong header type for group. Not sure how this even happened.") }
        }
    }
}

impl ESHeader<ESRecordHeader> for ESObject {
    fn header(&self) -> &ESRecordHeader {
        match self {
            ESObject::Record(r) => todo!(),
            _ => { panic!("Tried to get wrong header type for record. Not sure how this even happened.") }
        }
    }
}

// ===================================================================================================

impl ESObjectTraits for ESObject {
    fn object_count(&self) -> usize {
        match self {
            ESObject::Group(esgroup) => esgroup.object_count() + 1usize,
            ESObject::Record(_) => 1usize,
        }
    }

    fn object_size(&self) -> &u32 {
        match self {
            ESObject::Group(esgroup) => esgroup.object_size(),
            ESObject::Record(esrecord) => &esrecord.header().size,
        }
    }
}

// ===================================================================================================

pub trait ESObjectTraits {
    fn object_count(&self) -> usize { 1usize }
    fn object_size(&self) -> &u32;
}

// ===================================================================================================

pub trait ESHeader<H> {
    fn header(&self) -> &H;
}