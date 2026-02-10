use crate::dev::*;

pub trait GroupParser<T> where T: for<'esm> Parse<&'esm[u8]> {
    fn parse_group(i: &[u8]) -> IResult<&[u8], GroupVec<T>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, items) = many0(T::parse_le)(raw)?;
        Ok((i, GroupVec { header, data: items} ))
    }
}