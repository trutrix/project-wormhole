use nom_derive::{Parse, nom::{IResult, bytes::complete::take, number::complete::{le_u8, le_u16, le_u32}}};

// ESM strings are a hybrid of a sized string and a null-terminated string.

// pub struct SizedString<T> {
//     pub size: T,
//     pub value: String
// }


// impl nom_derive::Parse<&[u8]> for SizedString<u32> {
//     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
//         let (i, size) = le_u32(i)?;
//         let (i, value) = take(size)(i)?;

//         if let Ok(s) = String::from_utf8(value.to_vec()) {
//             Ok((i, SizedString { size, value: s }))
//         } else {
//             Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
//         }
//     }
// }

// impl nom_derive::Parse<&[u8]> for SizedString<u16> {
//     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
//         let (i, size) = le_u16(i)?;
//         let (i, value) = take(size)(i)?;

//         if let Ok(s) = String::from_utf8(value.to_vec()) {
//             Ok((i, SizedString { size, value: s }))
//         } else {
//             Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
//         }
//     }
// }

// impl nom_derive::Parse<&[u8]> for SizedString<u8> {
//     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
//         let (i, size) = le_u8(i)?;
//         let (i, value) = take(size)(i)?;

//         if let Ok(s) = String::from_utf8(value.to_vec()) {
//             Ok((i, SizedString { size, value: s }))
//         } else {
//             Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
//         }
//     }
// }


// ================================================================================


pub struct U8ESMString(pub String);

impl Parse<&[u8]> for U8ESMString {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, size) = le_u8(i)?;

        if size == 0 {
            return Ok((i, U8ESMString(String::new())));
        }

        // Discard the trailing null byte
        let (i, value) = take(size-1)(i)?;

        if let Ok(s) = String::from_utf8(value.to_vec()) {
            Ok((i, U8ESMString(s)))
        } else {

            #[cfg(debug_assertions)]
            panic!("Failed to parse U32ESMString: Invalid UTF-8 sequence");

            #[cfg(not(debug_assertions))]
            Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
        }
    }
}


// ================================================================================

pub struct U16ESMString(pub String);

impl Parse<&[u8]> for U16ESMString {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, size) = le_u16(i)?;

        if size == 0 {
            return Ok((i, U16ESMString(String::new())));
        }
        
        // Discard the trailing null byte
        let (i, value) = take(size-1)(i)?;

        if let Ok(s) = String::from_utf8(value.to_vec()) {
            Ok((i, U16ESMString(s)))
        } else {

            #[cfg(debug_assertions)]
            panic!("Failed to parse U32ESMString: Invalid UTF-8 sequence");

            #[cfg(not(debug_assertions))]
            Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
        }
    }
}


// ================================================================================

pub struct U32ESMString(pub String);

impl Parse<&[u8]> for U32ESMString {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, size) = le_u32(i)?;

        if size == 0 {
            return Ok((i, U32ESMString(String::new())));
        }

        // Discard the trailing null byte
        let (i, value) = take(size-1)(i)?;

        if let Ok(s) = String::from_utf8(value.to_vec()) {
            Ok((i, U32ESMString(s)))
        } else {

            #[cfg(debug_assertions)]
            panic!("Failed to parse U32ESMString: Invalid UTF-8 sequence");

            #[cfg(not(debug_assertions))]
            Err(nom_derive::nom::Err::Error(nom_derive::nom::error::Error::new(i, nom_derive::nom::error::ErrorKind::MapRes)))
        }
    }
}


// ================================================================================


