use crate::dev::*;
use crate::traits::ValidateData;
use super::record::VersionControl;

// ====================================================================================================

#[derive(Debug, Clone)]
pub struct GroupHeader {
    /// Should always be ` b"GRUP" `
    pub iden: FourCC,
    /// Size INCLUDING header, unlike RecordHeader,
    pub size: u32,
    /// The type of group
    pub label: GroupLabel,
    /// TODO: Groups appear to have different version control info
    pub version_control: VersionControl
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupLabel {
    Top(FourCC),
    WorldChildren(FormId),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock(CellLocation),
    ExteriorCellSubBlock(CellLocation),
    CellChildren(FormId),
    TopicChildren(FormId),
    CellPersistentChildren(FormId),
    CellTemporaryChildren(FormId),
    CellVisibleDistantChildren(FormId),
    Unknown([u8;4])
}

// ====================================================================================================


// TODO: the logic here can be slightly improved as there is far more non-top groups
// impl Parse<&[u8]> for GroupLabel {
//     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
//         let (i, data) = FourCC::parse(i)?;
//         let (i, label_type) = le_u32(i)?;

//         match label_type {
//             0 => { Ok((i, GroupLabel::Top(data))) }
//             1 => { Ok((i, GroupLabel::WorldChildren(FormId(u32::from_le_bytes(data.0))))) }
//             2 => { Ok((i, GroupLabel::InteriorCellBlock(i32::from_le_bytes(data.0)))) }
//             3 => { Ok((i, GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(data.0)))) }
//             4 => { Ok((i, GroupLabel::ExteriorCellBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
//             5 => { Ok((i, GroupLabel::ExteriorCellSubBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
//             6 => { Ok((i, GroupLabel::CellChildren(FormId(u32::from_le_bytes(data.0))))) }
//             7 => { Ok((i, GroupLabel::TopicChildren(FormId(u32::from_le_bytes(data.0))))) }
//             8 => { Ok((i, GroupLabel::CellPersistentChildren(FormId(u32::from_le_bytes(data.0))))) }
//             9 => { Ok((i, GroupLabel::CellTemporaryChildren(FormId(u32::from_le_bytes(data.0))))) }
//             10 => { Ok((i, GroupLabel::CellVisibleDistantChildren(FormId(u32::from_le_bytes(data.0))))) }
//             _ => { Ok((i, GroupLabel::Unknown(data))) }
//         }
//     }
// }

impl Parse<&[u8]> for GroupLabel {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, data) = <[u8;4]>::parse(i)?;
        let (i, label_type) = le_u32(i)?;

        match label_type {
            0 => { Ok((i, GroupLabel::Top(FourCC(data)))) }
            1 => { Ok((i, GroupLabel::WorldChildren(data.into()))) }
            2 => { Ok((i, GroupLabel::InteriorCellBlock(i32::from_le_bytes(data)))) }
            3 => { Ok((i, GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(data)))) }
            4 => { Ok((i, GroupLabel::ExteriorCellBlock(data.into()))) }
            5 => { Ok((i, GroupLabel::ExteriorCellSubBlock(data.into()))) }
            6 => { Ok((i, GroupLabel::CellChildren(data.into()))) }
            7 => { Ok((i, GroupLabel::TopicChildren(data.into()))) }
            8 => { Ok((i, GroupLabel::CellPersistentChildren(data.into()))) }
            9 => { Ok((i, GroupLabel::CellTemporaryChildren(data.into()))) }
            10 => { Ok((i, GroupLabel::CellVisibleDistantChildren(data.into()))) }
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

// ====================================================================================================

#[derive(Clone, PartialEq, Eq)]
pub struct CellLocation(pub [i16;2]);

// ====================================================================================================

impl From<[u8;4]> for CellLocation {
    fn from(value: [u8;4]) -> Self {
        let y = [value[0], value[1]];
        let x = [value[2], value[3]];
        Self([i16::from_le_bytes(x), i16::from_le_bytes(y)])
    }
}

// ====================================================================================================

impl std::fmt::Debug for CellLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CellLocation(x: {}, y: {})", self.0[0], self.0[1])
    }
}