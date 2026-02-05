
use nom_derive::nom::IResult;


pub trait ParseVersioned<'nom, T, E>: Sized {
    fn parse_versioned(i: &'nom[u8], version: T) -> IResult<&'nom[u8], Self, E>;
}