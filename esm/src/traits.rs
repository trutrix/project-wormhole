use nom_derive::nom::IResult;

use crate::{dev::*, structs::field::{Field, FieldHeader}};

pub mod record;
pub mod group;
pub mod field;
pub mod parse;

pub mod prelude {
    pub use super::record::*;
    pub use super::group::*;
    pub use super::field::*;
    pub use super::parse::*;
}


// ====================================================================================================


// ====================================================================================================



// ====================================================================================================




// ====================================================================================================

// pub trait ESMParser<T> where T: for<'esm> Parse<&'esm[u8]> {
//     fn parse_as_group(i: &[u8]) -> IResult<&[u8], Group<T>> {
//         let (i, (header, raw)) = alloc_group(i)?;
//         let (_, items) = many0(T::parse_le)(raw)?;
//         Ok((i, Group { header, data: items} ))
//     }
//     fn parse_as_record(i: &[u8]) -> IResult<&[u8], Record<T>> {
//         let (i, (header, raw)) = alloc_record(i)?;
//         let (_, fields) = many0(T::parse_le)(raw)?;
//         Ok((i, Record { header, fields }))
//     }
// }

// ====================================================================================================



// ====================================================================================================


pub trait VirtualMachineAdapterTrait {
    fn get_adapter_name(&self) -> &str;
    fn get_property_count(&self) -> usize;
    fn get_properties(&self) -> &Vec<VMADPropertyEntry>;
}

// ====================================================================================================

pub trait EDID {
    fn get_editor_id(&self) -> &ESMString;
}

impl<T: EDID> ParseField<Field<EditorId>> for T {
    fn parse_field(i: &[u8]) -> IResult<&[u8], Field<EditorId>, nom::error::Error<&[u8]>> {
        let (i, edid) = <Field::<EditorId>>::parse(i)?;
        Ok((i, edid))
    }
}

// ====================================================================================================


pub trait ParseField<T> {
    fn parse_field(i: &[u8]) -> IResult<&[u8], T, nom::error::Error<&[u8]>>;
}