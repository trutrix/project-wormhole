



pub trait ParseVersioned: Sized {
    fn parse_versioned(i: &[u8], version: u32) -> nom::IResult<&[u8], Self>;
    #[allow(unused_variables)]
    fn parse_versioned_depth(i: &[u8], version: u32, depth: usize) -> nom::IResult<&[u8], Self> { unimplemented!("You called this without actually implementing it.") }
}