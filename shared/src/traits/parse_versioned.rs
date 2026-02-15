
use nom_derive::nom::IResult;


pub trait ParseVersioned<'nom, T, E>: Sized {
    fn parse_versioned(i: &'nom[u8], version: T) -> IResult<&'nom[u8], Self, E>;
}


pub trait ParseV<I, V> where Self: Sized {
    fn parsev(i: I, version: V) -> nom_derive::nom::IResult<I, Self, nom_derive::nom::error::Error<I>>;
}