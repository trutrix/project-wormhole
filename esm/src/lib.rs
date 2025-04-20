pub mod types;
pub mod records;
pub mod traits;
pub mod structs;
pub mod esm;
mod tests;

mod dev {
    pub use nom_derive::{Parse, NomLE};
    pub use nom::IResult;
    pub use nom::number::complete::*;
    pub use nom::multi::many0;
    pub use nom::bytes::complete::take;
    pub use nom_derive::nom;


    pub use super::structs::fourcc::FourCC;
    pub use super::structs::record::*;
    pub use super::structs::field::*;
    pub use super::structs::group::*;
    pub use super::types::*;
    pub use super::structs::strings::*;
}