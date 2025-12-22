
use nom_derive::nom::IResult;


pub trait ParseVersioned: Sized {
    fn parse_versioned(i: &[u8], version: u32) -> IResult<&[u8], Self>;
    #[allow(unused_variables)]
    fn parse_versioned_depth(i: &[u8], version: u32, depth: usize) -> IResult<&[u8], Self> { unimplemented!("You called this without actually implementing it.") }
}