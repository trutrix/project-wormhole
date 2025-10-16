use crate::dev::*;

#[derive(Debug)]
pub struct VirtualMachineAdapter {

}


impl Parse<&[u8]> for VirtualMachineAdapter {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        Ok((i, Self {}))
    }
}