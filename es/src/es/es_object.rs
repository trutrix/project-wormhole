use crate::{dev::*, es::{self, es_group::{ESGroupHeader, ESGroupTrait, ESGroupTyped}, es_record::{ESRecordFlags, ESRecordHeader, ESRecordTyped, ESVersionControl}}, traits::{ParseAllocated, ParseAllocated2}};

// ====================================================================================================

#[derive(Debug)]
pub enum ESObject {
    Record(ESRecordTyped),
    Group(ESGroupTyped)
}

// ====================================================================================================

/// Custom parsing function if you do not know what the next object will be
impl Parse<&[u8]> for ESObject {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        // Get iden and size
        let (i, iden) = FourCC::parse(i)?;
        let (i, size) = le_u32(i)?;

        // Check if object is a group (for header and sizing)
        if &iden.0 == b"GRUP" {

            // Subtract 24 from size if it not zero
            let size = if size == 0 { size } else { size - 24 };
            
            // Get rest of group header data
            let (i, label_value) = <[u8;4]>::parse(i)?;
            let (i, label_type) = le_u32(i)?;
            let (i, version_control) = ESVersionControl::parse(i)?;

            // Put data into group header
            let header = ESGroupHeader { iden, size, label_value, label_type, version_control };

            // Allocate the raw data
            let (i, raw) = take(size as usize)(i)?;

            // Parse and handle results
            match ESGroupTyped::parse_allocated2(header, raw) {
                Ok((_, data)) => Ok((i, ESObject::Group(data))),
                Err(e) => Err(e),
            }
        } else {

            // Get rest of record header
            let (i, flags) = ESRecordFlags::parse(i)?;
            let (i, form_id) = FormId::parse(i)?;
            let (i, version_control) = ESVersionControl::parse(i)?;

            // Put data into header
            let header = ESRecordHeader { iden, size, flags, form_id, version_control };

            // Allocate raw data
            let (i, raw) = take(size as usize)(i)?;

            // Parse and handle results
            match ESRecordTyped::parse_allocated(header, raw) {
                Ok(data) => Ok((i, ESObject::Record(data))),
                Err(e) => Err(nom::Err::Error(e)),
            }
        }
    }
}

// ====================================================================================================

pub trait ESObjectTrait {
    fn object_count(&self) -> &usize;
    fn object_size(&self) -> &u32;
}

// ====================================================================================================

impl ESObjectTrait for ESObject {
    fn object_count(&self) -> &usize {
        match self {
            ESObject::Record(_) => &1usize,
            ESObject::Group(esgroup_typed) => esgroup_typed.object_count()
        }
    }

    fn object_size(&self) -> &u32 {
        match self {
            ESObject::Record(r) => r.object_size(),
            ESObject::Group(g) => g.object_size(),
        }
    }
}

// ====================================================================================================

#[derive(Debug)]
pub enum ESError {
    NotImplemented,
    NotRecord,
    NotGroup
}