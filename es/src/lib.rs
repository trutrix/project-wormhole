pub mod types;
pub mod records;
pub mod traits;
pub mod structs;
pub mod es;
pub mod consts;
pub mod groups;
mod tests;

pub use nom_derive::Parse;


mod dev {
    pub use nom_derive::{Parse, NomLE};
    pub use nom::IResult;
    pub use nom::number::complete::*;
    pub use nom::multi::many0;
    pub use nom::bytes::complete::take;
    
    pub use nom_derive::nom;
    pub use project_wormhole_proc::define_record3;

    
    pub use super::consts::*;

    pub use project_wormhole_shared::structs::fourcc::FourCC;
    
    pub use super::structs::record::*;
    pub use super::structs::field::*;
    pub use super::structs::group::*;
    pub use super::types::*;
    pub use super::structs::strings::*;
    pub use super::structs::geometry::*;
    pub use super::structs::colors::*;
    pub use super::structs::virtual_machine_adapter::*;
    pub use super::structs::destructible::*;
    pub use super::structs::data::*;
    pub use super::structs::model::*;
    pub use super::structs::form_id::FormId;

    pub use speedy::{Readable, Writable};
}


pub mod prelude {
    pub use crate::traits::prelude::*;
    pub use nom_derive::Parse;
}