use crate::{dev::*, es::{es_group::top::ESTop, es_object::ESObject, es_record::{ESRecordHeader, ESVersionControl}}, groups::prelude::*, traits::ParseAllocated};

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
pub enum ESGroupTyped {
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

impl nom_derive::Parse<&[u8]> for ESGroupTyped {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, header) = ESGroupHeader::parse(i)?;
        let (i, raw) = take(header.size as usize - 24)(i)?;
        match header.get_label() {
            ESGroupLabel::Top(_) => { 
                if let Ok(g) = ESTop::parse_allocated(header, raw) {
                    Ok((i, ESGroupTyped::Top(g)))
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
            ESGroupLabel::Unknown(_) => Ok((i, ESGroupTyped::Unknown(header))),
        }
    }
}

// ====================================================================================================

#[cfg(feature = "speedy")]
impl<'a, C: speedy::Context> Readable<'a, C> for ESGroupTyped {
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
        ESGroupLabel::from((self.label_value, self.label_type))
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

impl From<([u8;4], u32)> for ESGroupLabel {
    fn from(value: ([u8;4], u32)) -> Self {
        match value.1 {
            0 => { ESGroupLabel::Top(FourCC(value.0)) }
            1 => { ESGroupLabel::WorldChildren(value.0.into()) }
            2 => { ESGroupLabel::InteriorCellBlock(i32::from_le_bytes(value.0)) }
            3 => { ESGroupLabel::InteriorCellSubBlock(i32::from_le_bytes(value.0)) }
            4 => { ESGroupLabel::ExteriorCellBlock(value.0.into()) }
            5 => { ESGroupLabel::ExteriorCellSubBlock(value.0.into()) }
            6 => { ESGroupLabel::CellChildren(value.0.into()) }
            7 => { ESGroupLabel::TopicChildren(value.0.into()) }
            8 => { ESGroupLabel::CellPersistentChildren(value.0.into()) }
            9 => { ESGroupLabel::CellTemporaryChildren(value.0.into()) }
            10 => { ESGroupLabel::CellVisibleDistantChildren(value.0.into()) }
            _ => { ESGroupLabel::Unknown(value.0) }
        }
    }
}

// ====================================================================================================

impl ESObject for ESGroupTyped {
    fn object_count(&self) -> &usize {
        &1usize
    }

    fn object_size(&self) -> &u32 {
        match self {
            ESGroupTyped::Top(estop) => estop.object_size(),
            ESGroupTyped::WorldChildren(g) => &g.header.size,
            ESGroupTyped::InteriorCellBlock(g) => &g.header.size,
            ESGroupTyped::InteriorCellSubBlock(g) => &g.header.size,
            ESGroupTyped::ExteriorCellBlock(g) => &g.header.size,
            ESGroupTyped::ExteriorCellSubBlock(g) => &g.header.size,
            ESGroupTyped::CellChildren(g) => &g.header.size,
            ESGroupTyped::TopicChildren(g) => &g.header.size,
            ESGroupTyped::CellPersistentChildren(g) => &g.header.size,
            ESGroupTyped::CellTemporaryChildren(g) => &g.header.size,
            ESGroupTyped::CellVisibleDistantChildren(g) => &g.header.size,
            ESGroupTyped::Unknown(g) => &g.size,
        }
    }

    fn try_get_form_id(&self) -> Option<&FormId> {
        None
    }
}

// ====================================================================================================


pub fn alloc_group(i: &[u8]) -> IResult<&[u8], (ESGroupHeader, &[u8])> {
    let (i, header) = ESGroupHeader::parse(i)?;
    let (i, raw) = take(header.size as usize)(i)?;
    Ok((i, (header, raw)))
}

// ====================================================================================================

pub trait ESGroupTraits {
    fn get_header(&self) -> &ESGroupHeader;
}

// ====================================================================================================

impl ESGroupTraits for ESGroupTyped {
    fn get_header(&self) -> &ESGroupHeader {
        match self {
            ESGroupTyped::Top(top) => { top.get_header() },
            ESGroupTyped::WorldChildren(g) => &g.header,
            ESGroupTyped::InteriorCellBlock(g) => &g.header,
            ESGroupTyped::InteriorCellSubBlock(g) => &g.header,
            ESGroupTyped::ExteriorCellBlock(g) => &g.header,
            ESGroupTyped::ExteriorCellSubBlock(g) => &g.header,
            ESGroupTyped::CellChildren(g) => &g.header,
            ESGroupTyped::TopicChildren(g) => &g.header,
            ESGroupTyped::CellPersistentChildren(g) => &g.header,
            ESGroupTyped::CellTemporaryChildren(g) => &g.header,
            ESGroupTyped::CellVisibleDistantChildren(g) => &g.header,
            ESGroupTyped::Unknown(g) => g,
        }
    }
}

// ====================================================================================================

impl ParseAllocated<ESGroupHeader, &[u8]> for ESGroupTyped {
    fn parse_allocated(header: ESGroupHeader, raw: &[u8]) -> Result<Self, nom::error::Error<&[u8]>> {
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

pub trait ESGroupT {
    fn group_label(&self) -> &ESGroupLabel;
    fn group_size(&self) -> &u32;
    fn try_get_group_iden(&self) -> Option<&FourCC>;
}


/// Implement [ESObject] for anything that implements [ESGroupT]
impl<T> ESObject for T where T: ESGroupT {
    fn object_count(&self) -> &usize { todo!("More logic needs to be fleshed out") }
    fn object_size(&self) -> &u32 { self.group_size() }
    fn try_get_form_id(&self) -> Option<&FormId> { None }
}