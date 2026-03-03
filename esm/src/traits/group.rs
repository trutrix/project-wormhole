use crate::dev::*;

pub trait GroupParser<T> where T: for<'esm> Parse<&'esm[u8]> {
    fn parse_group(i: &[u8]) -> IResult<&[u8], Group<T>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, items) = many0(T::parse_le)(raw)?;
        Ok((i, Group { header, data: items} ))
    }
}


pub trait GroupTraits {
    fn parse_as_group(&self) -> IResult<&[u8], Self> where Self: Sized;
    fn get_group_header(&self) -> &GroupHeader;
}