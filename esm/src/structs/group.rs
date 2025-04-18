use crate::dev::*;

use super::record::VersionControl;


#[derive(Debug, NomLE)]
pub struct GroupHeader {
    pub iden: FourCC, // Always 'GRUP'
    pub size: u32, // Size INCLUDING header, unlike RecordHeader,
    pub label: GroupLabel, // 8 bytes, reversed process
    pub version_control: VersionControl // TODO: Unsure if records and groups share the same version information
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupLabel {
    Top(FourCC),
    WorldChildren(u32),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock([i16;2]),
    ExteriorCellSubBlock([i16;2]),
    CellChildren(u32),
    TopicChildren(u32),
    CellPersistentChildren(u32),
    CellTemporaryChildren(u32),
    CellVisibleDistantChildren(u32),
    Unknown(FourCC)
}

impl Parse<&[u8]> for GroupLabel {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, data) = FourCC::parse(i)?;
        let (i, label_type) = le_u32(i)?;

        match label_type {
            0 => {
                Ok((i, GroupLabel::Top(data)))
            }
            1 => {
                Ok((i, GroupLabel::WorldChildren(u32::from_le_bytes(data.0))))
            }
            2 => {
                Ok((i, GroupLabel::InteriorCellBlock(i32::from_le_bytes(data.0))))
            }
            3 => {
                Ok((i, GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(data.0))))
            }
            4 => {
                Ok((i, GroupLabel::ExteriorCellBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())])))
            }
            5 => {
                Ok((i, GroupLabel::ExteriorCellSubBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())])))
            }
            6 => {
                Ok((i, GroupLabel::CellChildren(u32::from_le_bytes(data.0))))
            }
            7 => {
                Ok((i, GroupLabel::TopicChildren(u32::from_le_bytes(data.0))))
            }
            8 => {
                Ok((i, GroupLabel::CellPersistentChildren(u32::from_le_bytes(data.0))))
            }
            9 => {
                Ok((i, GroupLabel::CellTemporaryChildren(u32::from_le_bytes(data.0))))
            }
            10 => {
                Ok((i, GroupLabel::CellVisibleDistantChildren(u32::from_le_bytes(data.0))))
            }
            _ => {
                Ok((i, GroupLabel::Unknown(data)))
            }
        }




    }
}