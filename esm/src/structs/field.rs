use crate::dev::*;

#[derive(Debug, NomLE)]
pub struct Field<T> {
    pub header: FieldHeader,
    pub data: T,
}

#[derive(Debug, NomLE, Clone, Copy)]
pub struct FieldHeader16 {
    pub iden: FourCC,
    pub size: u16, // World groups contain different field headers
}

#[derive(Debug, NomLE, Clone, Copy)]
pub struct FieldHeader32 {
    _size_iden: FourCC,  // Should be 'XXXX'
    _skipped_size: u16,  // Should be 4,
    pub size: u32,       // Size of the field data in bytes
    pub iden: FourCC,    // Real field iden
    _skipped_size2: u16, // Should be 0
}

#[derive(Debug, Clone, Copy)]
pub enum FieldHeader {
    Normal(FieldHeader16),
    Large(FieldHeader32),
}

impl FieldHeader {
    pub fn size(&self) -> usize {
        match self {
            FieldHeader::Normal(h) => h.size as usize,
            FieldHeader::Large(h) => h.size as usize,
        }
    }

    pub fn iden(&self) -> &FourCC {
        match self {
            FieldHeader::Normal(h) => &h.iden,
            FieldHeader::Large(h) => &h.iden,
        }
    }
}


// TODO: Most groups do not have the extra size field.
// It would be more efficient to have seperate structs for the two types of fields
// but this is easier to implement for now.
impl Parse<&[u8]> for FieldHeader {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (orig, iden) = FourCC::parse(i)?;

        if iden == b"XXXX" {
            // Always 4 bytes
            let (i, size) = le_u16(orig)?;

            // Actual size of data
            let (i, real_size) = le_u32(i)?;

            // The real iden is the next 4 bytes
            let (i, real_iden) = FourCC::parse(i)?;

            // Always 0
            let (i, zero_size) = le_u16(i)?;

            Ok((i, FieldHeader::Large(FieldHeader32 {
                _size_iden: iden,
                _skipped_size: size,
                size: real_size,
                iden: real_iden,
                _skipped_size2: zero_size,
            })))
        } else {
            let (i, size) = le_u16(orig)?;
            Ok((i, FieldHeader::Normal(FieldHeader16 { iden, size })))
        }
        
    }
}

// ================================================================================================

// For quick debugging, this is a raw field with no parsing
pub struct RawField<'esm> {
    pub header: FieldHeader,
    pub data: &'esm [u8],
}

// Implement nom_derive::Parse for FieldHeader
impl<'esm, 'nom> Parse<&'nom [u8]> for RawField<'esm>
where
    'nom: 'esm,
{
    fn parse(i: &'nom [u8]) -> IResult<&'nom [u8], Self, nom::error::Error<&'nom [u8]>> {
        let (i, (header, data)) = alloc_field(i)?;
        Ok((i, RawField { header, data }))
    }
}

// Implement debug ourselves to avoid dumping too many raw bytes
impl std::fmt::Debug for RawField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RawField {{ header: {:?}, data: [verbose bytes]",
            self.header
        )
    }
}

pub fn alloc_field(i: &[u8]) -> IResult<&[u8], (FieldHeader, &[u8]), nom::error::Error<&[u8]>> {
    let (i, header) = FieldHeader::parse(i)?;
    let size = match &header {
        FieldHeader::Normal(h) => h.size as usize,
        FieldHeader::Large(h) => h.size as usize,
    };
    let (i, data) = take(size)(i)?;
    Ok((i, (header, data)))
}
