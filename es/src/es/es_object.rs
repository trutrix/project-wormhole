use crate::{dev::*, es::{self, es_group::{ESGroupHeader, ESGroupTraits, ESGroupTyped}, es_record::{ESRecordFlags, ESRecordHeader, ESRecordTraits, ESRecordTyped, ESVersionControl}}, traits::ParseAllocated};

// ====================================================================================================

pub trait ESObject {
    fn object_count(&self) -> &usize;
    fn object_size(&self) -> &u32;
    fn try_get_form_id(&self) -> Option<&FormId>;
    fn parse(i: &[u8]) -> IResult<&[u8], Box<dyn ESObject>> where Self: Sized { parse_es_object(i) }
}

// ====================================================================================================

pub fn parse_es_object(i: &[u8]) -> IResult<&[u8], Box<dyn ESObject>> {
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
        match ESGroupTyped::parse_allocated(header, raw) {
            Ok(data) => Ok((i, Box::new(data))),
            Err(e) => Err(nom::Err::Error(e)),
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
            Ok(data) => Ok((i, Box::new(data))),
            Err(e) => Err(nom::Err::Error(e)),
        }
    }
}

// ====================================================================================================

impl std::fmt::Debug for dyn ESObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ESObject: {:?} object(s), {:?} bytes", self.object_count(), self.object_size())
    }
}