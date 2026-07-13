use crate::{dev::*, es::{es_group::top::ESTop, es_object::{ESHeader, ESObjectTraits}, es_record::ESVersionControl}, groups::prelude::*, traits::ParseAllocated};

// ====================================================================================================

pub mod top;
pub mod world_children;
pub mod interior_cell_block;
pub mod interior_cell_sub_block;
pub mod exterior_cell_block;
pub mod exterior_cell_sub_block;
pub mod cell_children;
pub mod topic_children;
pub mod cell_persistent_children;
pub mod cell_temporary_children;
pub mod cell_visible_distant_children;

// ====================================================================================================

#[derive(Debug)]
pub enum ESGroup {
    Top(ESTop),
    WorldChildren(world_children::ESWorldChildren),
    InteriorCellBlock(interior_cell_block::ESInteriorCellBlock),
    InteriorCellSubBlock(interior_cell_sub_block::ESInteriorCellSubBlock),
    ExteriorCellBlock(exterior_cell_block::ESExteriorCellBlock),
    ExteriorCellSubBlock(exterior_cell_sub_block::ESExteriorCellSubBlock),
    CellChildren(cell_children::ESCellChildren),
    TopicChildren(topic_children::ESTopicChildren),
    CellPersistentChildren(cell_persistent_children::ESCellPersistentChildren),
    CellTemporaryChildren(cell_temporary_children::ESCellTemporaryChildren),
    CellVisibleDistantChildren(cell_visible_distant_children::ESCellVisibleDistantChildren),
    Unknown(ESGroupHeader)
}

// ====================================================================================================

impl nom_derive::Parse<&[u8]> for ESGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, header) = ESGroupHeader::parse(i)?;
        let (i, raw) = take(header.size as usize - 24)(i)?;
        match header.get_label() {
            ESGroupLabel::Top(_) => { 
                if let Ok(g) = ESTop::parse_allocated(header, raw) {
                    Ok((i, ESGroup::Top(g)))
                } else {
                    Err(nom::Err::Error(nom::error::Error::new(raw, nom::error::ErrorKind::Fail)))
                }
            },
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

    fn object_size(&self) -> &u32 {
        match self {
            ESGroup::Top(estop) => estop.object_size(),
            ESGroup::WorldChildren(g) => &g.header.size,
            ESGroup::InteriorCellBlock(g) => &g.header.size,
            ESGroup::InteriorCellSubBlock(g) => &g.header.size,
            ESGroup::ExteriorCellBlock(g) => &g.header.size,
            ESGroup::ExteriorCellSubBlock(g) => &g.header.size,
            ESGroup::CellChildren(g) => &g.header.size,
            ESGroup::TopicChildren(g) => &g.header.size,
            ESGroup::CellPersistentChildren(g) => &g.header.size,
            ESGroup::CellTemporaryChildren(g) => &g.header.size,
            ESGroup::CellVisibleDistantChildren(g) => &g.header.size,
            ESGroup::Unknown(g) => &g.size,
        }
    }
}

// ====================================================================================================


pub fn alloc_group(i: &[u8]) -> IResult<&[u8], (ESGroupHeader, &[u8])> {
    let (i, header) = ESGroupHeader::parse(i)?;
    let (i, raw) = take(header.size as usize)(i)?;
    Ok((i, (header, raw)))
}

// ====================================================================================================

impl ESHeader<ESGroupHeader> for ESGroup {
    fn header(&self) -> &ESGroupHeader {
        match self {
            ESGroup::Top(estop) => todo!(),
            ESGroup::WorldChildren(g) => &g.header,
            ESGroup::InteriorCellBlock(g) => &g.header,
            ESGroup::InteriorCellSubBlock(g) => &g.header,
            ESGroup::ExteriorCellBlock(g) => &g.header,
            ESGroup::ExteriorCellSubBlock(g) => &g.header,
            ESGroup::CellChildren(g) => &g.header,
            ESGroup::TopicChildren(g) => &g.header,
            ESGroup::CellPersistentChildren(g) => &g.header,
            ESGroup::CellTemporaryChildren(g) => &g.header,
            ESGroup::CellVisibleDistantChildren(g) => &g.header,
            ESGroup::Unknown(g) => &g,
        }
    }
}