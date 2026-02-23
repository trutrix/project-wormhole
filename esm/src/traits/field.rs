use crate::dev::*;


pub trait FieldParser<T> {
    fn parse_field(i: &[u8]) -> IResult<&[u8], Field<T>, nom::error::Error<&[u8]>> {
        let (i, header) = FieldHeader::parse(i)?;
        let (i, data) = Self::parse_field_body(i, header)?;
        Ok((i, Field { header, data }))
    }

    fn parse_field_body(i: &[u8], header: FieldHeader) -> IResult<&[u8], T, nom::error::Error<&[u8]>>;
}


// =================================================================================================

pub trait ParseField<T, V, E> {
    fn parse_field(i: &[u8], version: Option<V>) -> IResult<&[u8], Field<T>, E>;
}
