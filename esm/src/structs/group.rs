use crate::dev::*;
use crate::traits::ValidateData;
use super::record::VersionControl;

// ====================================================================================================

#[derive(Debug, Clone)]
pub struct GroupHeader {
    pub iden: FourCC, // Always 'GRUP'
    pub size: u32, // Size INCLUDING header, unlike RecordHeader,
    pub label: GroupLabel, // 8 bytes, reversed process
    pub version_control: VersionControl // TODO: Unsure if records and groups share the same version information
}

// ====================================================================================================

impl Parse<&[u8]> for GroupHeader {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, iden) = FourCC::parse(i)?;
        let (i, size) = le_u32(i)?;
        let (i, label) = GroupLabel::parse(i)?;
        let (i, version_control) = VersionControl::parse(i)?;

        Ok((i, GroupHeader { iden, size, label, version_control }))
    }
}

// ====================================================================================================

impl ValidateData for GroupHeader {
    fn is_valid(&self) -> bool {
        &self.iden.0 == b"GRUP"
    }
}

// ====================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupLabel {
    Top(FourCC),
    WorldChildren(FormId),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock([i16;2]),
    ExteriorCellSubBlock([i16;2]),
    CellChildren(FormId),
    TopicChildren(FormId),
    CellPersistentChildren(FormId),
    CellTemporaryChildren(FormId),
    CellVisibleDistantChildren(FormId),
    Unknown(FourCC)
}

// ====================================================================================================

impl Parse<&[u8]> for GroupLabel {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, data) = FourCC::parse(i)?;
        let (i, label_type) = le_u32(i)?;

        match label_type {
            0 => { Ok((i, GroupLabel::Top(data))) }
            1 => { Ok((i, GroupLabel::WorldChildren(FormId(u32::from_le_bytes(data.0))))) }
            2 => { Ok((i, GroupLabel::InteriorCellBlock(i32::from_le_bytes(data.0)))) }
            3 => { Ok((i, GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(data.0)))) }
            4 => { Ok((i, GroupLabel::ExteriorCellBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
            5 => { Ok((i, GroupLabel::ExteriorCellSubBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
            6 => { Ok((i, GroupLabel::CellChildren(FormId(u32::from_le_bytes(data.0))))) }
            7 => { Ok((i, GroupLabel::TopicChildren(FormId(u32::from_le_bytes(data.0))))) }
            8 => { Ok((i, GroupLabel::CellPersistentChildren(FormId(u32::from_le_bytes(data.0))))) }
            9 => { Ok((i, GroupLabel::CellTemporaryChildren(FormId(u32::from_le_bytes(data.0))))) }
            10 => { Ok((i, GroupLabel::CellVisibleDistantChildren(FormId(u32::from_le_bytes(data.0))))) }
            _ => { Ok((i, GroupLabel::Unknown(data))) }
        }
    }
}

// ====================================================================================================

/// Parse a group header and allocate the next byte slice
/// 
/// Debug: panic if not `GRUP` header
pub fn alloc_group(i: &[u8]) -> IResult<&[u8], (GroupHeader, &[u8])> {

    // parse the header, assumed to be for group
    let (i, header) = GroupHeader::parse(i)?;

    // Assert header should be group
    // If the header is not for a group, it will allocate the chunk incorrectly
    #[cfg(debug_assertions)]
    if &header.iden.0 != b"GRUP" {
        panic!("Invalid group header: {:?}", header.iden);
    }

    // Grab the next byte slice
    // Minus 24 because group headers do not include their own size.
    let (i, raw) = take(header.size as usize - 24)(i)?;
    
    Ok((i, (header, raw)))
}

// ====================================================================================================

#[derive(Debug)]
pub struct Group<T> {
    pub header: GroupHeader,
    pub data: Vec<T>
}

// ====================================================================================================

impl<'esm, T> Parse<&'esm[u8]> for Group<T> where T: for<'nom> Parse<&'esm[u8]> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {

        let (i, (header, data)) = alloc_group(i)?;
        
        #[cfg(debug_assertions)]
        let hc = header.clone();

        let (lo, data) = Self::parse_with_header(data, header)?;

        #[cfg(debug_assertions)]
        if !lo.is_empty() {
            panic!("Group did not consume its data: {:?}", hc);
        }

        Ok((i, data))
    }
}

// ====================================================================================================

impl<'esm, T> Group<T> where T: for<'nom> Parse<&'esm[u8]> {
    pub fn parse_with_header(i: &'esm[u8], header: GroupHeader) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, data) = many0(T::parse)(i)?;
        Ok((i, Group { header, data }))
    }
}