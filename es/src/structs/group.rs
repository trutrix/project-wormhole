use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

impl std::fmt::Display for GroupLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupLabel::Top(four_cc) => write!(f, "Top({})", four_cc),
            GroupLabel::WorldChildren(form_id) => write!(f, "WorldChildren({})", form_id),
            GroupLabel::InteriorCellBlock(i) => write!(f, "InteriorCellBlock({})", i),
            GroupLabel::InteriorCellSubBlock(i) => write!(f, "InteriorCellSubBlock({})", i),
            GroupLabel::ExteriorCellBlock(cell_location) => write!(f, "ExteriorCellBlock({:?})", cell_location),
            GroupLabel::ExteriorCellSubBlock(cell_location) => write!(f, "ExteriorCellSubBlock({:?})", cell_location),
            GroupLabel::CellChildren(form_id) => write!(f, "CellChildren({})", form_id),
            GroupLabel::TopicChildren(form_id) => write!(f, "TopicChildren({})", form_id),
            GroupLabel::CellPersistentChildren(form_id) => write!(f, "CellPersistentChildren({})", form_id),
            GroupLabel::CellTemporaryChildren(form_id) => write!(f, "CellTemporaryChildren({})", form_id),
            GroupLabel::CellVisibleDistantChildren(form_id) => write!(f, "CellVisibleDistantChildren({})", form_id),
            GroupLabel::Unknown(_) => todo!(),
        }
    }
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
            _ => { panic!("Unknown group encountered"); Ok((i, GroupLabel::Unknown(data))) }
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

impl<'esm, T> Parse<&'esm[u8]> for Group<T> where T: for<'nom> Parse<&'esm[u8]> + Send {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {

        let (i, (header, data)) = alloc_group(i)?;
        
        #[cfg(debug_assertions)]
        let hc = header.clone();

        let (lo, data) = Self::parse_pre_alloc(data, header)?;

        #[cfg(debug_assertions)]
        if !lo.is_empty() {
            panic!("Group did not consume its data: {:?}", hc);
        }

        Ok((i, data))
    }
}

// ====================================================================================================

impl<'esm, T> Group<T> where T: for<'nom> Parse<&'esm[u8]> + Send {
    pub fn parse_pre_alloc(raw: &'esm[u8], header: GroupHeader) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        
        let mut sub = raw;
        let mut chunks = Vec::new();

        while !sub.is_empty() {

            let (i, next_id) = FourCC::parse(sub)?;
            let (_, size) = le_u32(i)?;

            if &next_id.0 == b"GRUP" {
                let (i, chunk) = take(size as usize)(sub)?;
                chunks.push(chunk);
                sub = i;
            } else {
                let (i, chunk) = take(size as usize + 24)(sub)?;
                chunks.push(chunk);
                sub = i;
            }
        }

        let data = chunks.par_iter().map(|x| { 
            T::parse(x).unwrap().1
        }).collect();

        Ok((sub, Group { header, data }))
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


// ====================================================================================================


// pub trait GroupParser<T> where T: GroupParser<T> + Send {

//     fn parse_header(i: &[u8]) -> IResult<&[u8], GroupHeader, nom::error::Error<&[u8]>> {
//         GroupHeader::parse(i)
//     }
     
//     fn parse_body(i: &[u8]) -> IResult<&[u8], T, nom::error::Error<&[u8]>> {
//         T::parse(i)
//     }

//     fn parse(i: &[u8]) -> IResult<&[u8], Group<T>, nom::error::Error<&[u8]>> {
//         Group::<T>::parse(i)
//     }
// }