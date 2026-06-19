use crate::dev::*;


pub trait GroupTraits {
    fn parse_as_group(&self) -> IResult<&[u8], Self> where Self: Sized;
    fn get_group_header(&self) -> &GroupHeader;
}