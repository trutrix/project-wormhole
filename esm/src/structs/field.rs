use crate::dev::*;


#[derive(Debug, NomLE)]
pub struct Field<T> {
    pub header: FieldHeader,
    pub data: T
}


#[derive(Debug, NomLE)]
pub struct FieldHeader {
    pub iden: FourCC,
    pub size: u16, // World groups contain different field headers
}


#[derive(Debug, NomLE)]
pub struct FieldHeader32 {
    _size_iden: FourCC, // Should be 'XXXX'
    _skipped_size: u16, // Should be 4,
    pub size: u32, // Size of the field data in bytes
    pub iden: FourCC, // Real field iden
    _skipped_size2: u16 // Should be 0
}

// ================================================================================================

// For quick debugging, this is a raw field with no parsing
pub struct RawField<'esm> {
    pub header: FieldHeader,
    pub data: &'esm [u8]
}


// Implement nom_derive::Parse for FieldHeader
impl<'esm, 'nom> Parse<&'nom[u8]> for RawField<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, header) = FieldHeader::parse(i)?;
        let (i, data) = nom::bytes::complete::take(header.size as usize)(i)?;
        Ok((i, RawField { header, data }))
    }
}


// Implement debug ourselves to avoid dumping too many raw bytes
impl std::fmt::Debug for RawField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawField {{ header: {:?}, data: [verbose bytes]", self.header)
    }
}