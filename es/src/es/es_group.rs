use crate::{dev::*, es::{es_object::ESObjectTraits, es_record::ESVersionControl}, groups::prelude::*};

// ====================================================================================================

#[derive(Debug)]
pub enum ESGroup {
    Top(TopGroup),
    WorldChildren(WorldChildren),
    InteriorCellBlock(InteriorCellBlock),
    InteriorCellSubBlock(InteriorCellSubBlock),
    ExteriorCellBlock(ExteriorCellBlock),
    ExteriorCellSubBlock(ExteriorCellSubBlock),
    CellChildren(CellChildren),
    TopicChildren(TopicChildren),
    CellPersistentChildren(CellPersistentChildren),
    CellTemporaryChildren(CellTemporaryChildren),
    CellVisibleDistantChildren(CellVisibleDistantChildren),
    Unknown([u8;4])
}

// ====================================================================================================

impl nom_derive::Parse<&[u8]> for ESGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, header) = ESGroupHeader::parse(i)?;
        match header.get_label() {
            ESGroupLabel::Top(four_cc) => todo!(),
            ESGroupLabel::WorldChildren(form_id) => todo!(),
            ESGroupLabel::InteriorCellBlock(_) => todo!(),
            ESGroupLabel::InteriorCellSubBlock(_) => todo!(),
            ESGroupLabel::ExteriorCellBlock(cell_location) => todo!(),
            ESGroupLabel::ExteriorCellSubBlock(cell_location) => todo!(),
            ESGroupLabel::CellChildren(form_id) => todo!(),
            ESGroupLabel::TopicChildren(form_id) => todo!(),
            ESGroupLabel::CellPersistentChildren(form_id) => todo!(),
            ESGroupLabel::CellTemporaryChildren(form_id) => todo!(),
            ESGroupLabel::CellVisibleDistantChildren(form_id) => todo!(),
            ESGroupLabel::Unknown(_) => todo!(),
        }
    }
}

// ====================================================================================================

#[cfg(feature = "speedy")]
impl<'a, C: speedy::Context> Readable<'a, C> for ESGroup {
    fn read_from< R: speedy::Reader< 'a, C > >( reader: &mut R ) -> Result< Self, <C as speedy::Context>::Error > {
        let header: ESGroupHeader = reader.read_value()?;

        match header.get_label() {
            ESGroupLabel::Top(four_cc) => todo!(),
            ESGroupLabel::WorldChildren(form_id) => todo!(),
            ESGroupLabel::InteriorCellBlock(_) => todo!(),
            ESGroupLabel::InteriorCellSubBlock(_) => todo!(),
            ESGroupLabel::ExteriorCellBlock(cell_location) => todo!(),
            ESGroupLabel::ExteriorCellSubBlock(cell_location) => todo!(),
            ESGroupLabel::CellChildren(form_id) => todo!(),
            ESGroupLabel::TopicChildren(form_id) => todo!(),
            ESGroupLabel::CellPersistentChildren(form_id) => todo!(),
            ESGroupLabel::CellTemporaryChildren(form_id) => todo!(),
            ESGroupLabel::CellVisibleDistantChildren(form_id) => todo!(),
            ESGroupLabel::Unknown(_) => todo!(),
        }
    }
}

// ====================================================================================================

#[derive(Debug, Clone, NomLE)]
#[cfg_attr(feature = "speedy", derive(Readable, Writable))]
pub struct ESGroupHeader {
    /// Should always be ` b"GRUP" `
    pub iden: FourCC,
    /// Size INCLUDING header, unlike RecordHeader,
    pub size: u32,
    pub label_value: [u8;4],
    pub label_type: u32,
    /// TODO: Groups appear to have different version control info
    pub version_control: ESVersionControl
}

// ====================================================================================================

impl ESGroupHeader {
    pub fn get_label(&self) -> ESGroupLabel {
        // let (i, data) = <[u8;4]>::parse(i)?;
        // let (i, label_type) = le_u32(i)?;

        match self.label_type {
            0 => { ESGroupLabel::Top(FourCC(self.label_value)) }
            1 => { ESGroupLabel::WorldChildren(self.label_value.into()) }
            2 => { ESGroupLabel::InteriorCellBlock(i32::from_le_bytes(self.label_value)) }
            3 => { ESGroupLabel::InteriorCellSubBlock(i32::from_le_bytes(self.label_value)) }
            4 => { ESGroupLabel::ExteriorCellBlock(self.label_value.into()) }
            5 => { ESGroupLabel::ExteriorCellSubBlock(self.label_value.into()) }
            6 => { ESGroupLabel::CellChildren(self.label_value.into()) }
            7 => { ESGroupLabel::TopicChildren(self.label_value.into()) }
            8 => { ESGroupLabel::CellPersistentChildren(self.label_value.into()) }
            9 => { ESGroupLabel::CellTemporaryChildren(self.label_value.into()) }
            10 => { ESGroupLabel::CellVisibleDistantChildren(self.label_value.into()) }
            _ => { ESGroupLabel::Unknown(self.label_value) }
        }
    }
}

// ====================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ESGroupLabel {
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

impl ESObjectTraits for ESGroup {
    fn object_count(&self) -> usize {
        1usize
    }
}