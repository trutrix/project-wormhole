use crate::{
    dev::*,
    es::{
        es_group::{ESGroupHeader, ESGroupTyped},
        es_record::{ESRecordFlags, ESRecordHeader, ESRecordTyped, ESVersionControl},
    },
    traits::{ParseAllocated, ParseAllocated2},
};

/*
 * New structure notes:
 * - A Record is an list of fields
 * - A Group is a list of Records and or Groups
 * - Records and Groups are considered objects
 * - WRLD, CELL, QUST contain sub groups
 */

// ====================================================================================================

pub trait ESObject {
    fn object_count(&self) -> &usize;
    /// The size present in the header
    fn header_size_value(&self) -> &u32;
    /// Real size this object would occupy in memory
    fn header_real_size(&self) -> u32 {
        if self.is_group() {
            *self.header_size_value()
        } else {
            *self.header_size_value() + 24
        }
    }
    fn is_group(&self) -> bool;
}

// ====================================================================================================

/// Custom parsing function if you do not know what the next object will be
pub fn parse_es_object(i: &[u8]) -> IResult<&[u8], Box<dyn ESObject>> {
    // Get iden and size
    let (i, iden) = FourCC::parse(i)?;
    let (i, size) = le_u32(i)?;

    // Check if object is a group (for header and sizing)
    if &iden.0 == b"GRUP" {
        // Subtract 24 from size if it not zero
        let size = if size == 0 { size } else { size - 24 };

        // Get rest of group header data
        let (i, label_value) = <[u8; 4]>::parse(i)?;
        let (i, label_type) = le_u32(i)?;
        let (i, version_control) = ESVersionControl::parse(i)?;

        // Put data into group header
        let header = ESGroupHeader {
            iden,
            size,
            label_value,
            label_type,
            version_control,
        };

        // Allocate the raw data
        let (i, raw) = take(size as usize)(i)?;

        // Parse and handle results
        match ESGroupTyped::parse_allocated2(header, raw) {
            Ok((_, data)) => Ok((i, Box::new(data))),
            Err(e) => Err(e),
        }
    } else {
        // Get rest of record header
        let (i, flags) = ESRecordFlags::parse(i)?;
        let (i, form_id) = FormId::parse(i)?;
        let (i, version_control) = ESVersionControl::parse(i)?;

        // Put data into header
        let header = ESRecordHeader {
            iden,
            size,
            flags,
            form_id,
            version_control,
        };

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
        if self.is_group() {
            write!(
                f,
                "ESObject -> ESGroup {{ bytes: {:?}, object_count: {:?} }}",
                self.header_size_value(),
                self.object_count()
            )
        } else {
            write!(
                f,
                "ESObject -> ESRecord {{ bytes: {:?} }}",
                self.header_size_value()
            )
        }
    }
}

// ====================================================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum ESError {
    NotImplemented,
    NotRecord,
    NotGroup,
}
