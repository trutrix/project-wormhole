use nom_derive::nom::IResult;

use crate::{dev::*, structs::field::{Field, FieldHeader}};


// ====================================================================================================

pub trait FieldParser<T> {
    fn parse_field(i: &[u8]) -> IResult<&[u8], Field<T>, nom::error::Error<&[u8]>> {
        let (i, header) = FieldHeader::parse(i)?;
        let (i, data) = Self::parse_field_body(i, header)?;
        Ok((i, Field { header, data }))
    }

    fn parse_field_body(i: &[u8], header: FieldHeader) -> IResult<&[u8], T, nom::error::Error<&[u8]>>;
}

// ====================================================================================================

pub trait EditorId {
    fn try_get_editor_id(&self) -> Option<ESMString>;
}


// ====================================================================================================

pub trait RecordParser<T> where T: for<'esm> Parse<&'esm[u8]> {
    fn parse_body(i: &[u8]) -> IResult<&[u8], Vec<T>, nom::error::Error<&[u8]>> {
        let (i, fields) = many0(complete(T::parse_le))(i)?;
        Ok((i, fields))
    }
    fn parse_record(i: &[u8]) -> IResult<&[u8], Record<T>, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;

        if header.flags.is_compressed() {
            if let Ok(dec) = decompress_record(raw) {
                if let Ok((_, fields)) = Self::parse_body(&dec) {
                    Ok((i, Record { header, fields }))
                } else {
                    println!("Failed to parse decompressed record: {:?}", header);
                    Ok((i, Record { header, fields: Vec::new() }))
                }
            } else {
                println!("Failed to decompress record: {:?}", header);
                Ok((i, Record { header, fields: Vec::new() }))
            }
        } else {
            let (_, fields) = Self::parse_body(raw)?;
            Ok((i, Record { header, fields }))
        }
    }
}

// ====================================================================================================

pub trait GroupParser<T> where T: for<'esm> Parse<&'esm[u8]> {
    fn parse_group(i: &[u8]) -> IResult<&[u8], Group<T>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, items) = many0(complete(T::parse_le))(raw)?;
        Ok((i, Group { header, data: items} ))
    }
}


// ====================================================================================================

pub trait ESMParser<T> where T: for<'esm> Parse<&'esm[u8]> {
    fn parse_as_group(i: &[u8]) -> IResult<&[u8], Group<T>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, items) = many0(complete(T::parse_le))(raw)?;
        Ok((i, Group { header, data: items} ))
    }
    fn parse_as_record(i: &[u8]) -> IResult<&[u8], Record<T>> {
        let (i, (header, raw)) = alloc_record(i)?;
        let (_, fields) = many0(complete(T::parse_le))(raw)?;
        Ok((i, Record { header, fields }))
    }
}

// ====================================================================================================

pub trait Children {
    fn has_children(&self) -> bool;
    fn get_children<T>(&self) -> Option<T>;
}