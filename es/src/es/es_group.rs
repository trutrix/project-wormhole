use crate::{dev::*, es::{es_group::top::ESTopTyped, es_object::{ESObject, parse_es_object}, es_record::{ESRecordHeader, ESRecordTyped, ESVersionControl}}, groups::prelude::*, traits::{ParseAllocated, ParseAllocated2}};

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
    Top(ESTopTyped),
    WorldChildren(world_children::ESWorldChildren),
    InteriorCellBlock(interior_cell_block::ESInteriorCellBlock),
    InteriorCellSubBlock(interior_cell_sub_block::ESInteriorCellSubBlock),
    ExteriorCellBlock(exterior_cell_block::ESExteriorCellBlock),
    ExteriorCellSubBlock(exterior_cell_sub_block::ESExteriorCellSubBlock),
    CellChildren(cell_children::ESCellChildren),
    TopicChildren(ESGroup),
    CellPersistentChildren(ESGroup),
    CellTemporaryChildren(ESGroup),
    CellVisibleDistantChildren(ESGroup),
    Unknown(ESGroupHeader)
}

// ====================================================================================================

impl nom_derive::Parse<&[u8]> for ESGroupTyped {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, header) = ESGroupHeader::parse(i)?;
        let (i, raw) = take(header.size as usize - 24)(i)?;
        match header.get_label() {
            ESGroupLabel::Top(_) => { 
                if let Ok((_, g)) = ESTopTyped::parse_allocated2(header, raw) {
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

impl ParseAllocated2<ESGroupHeader, &[u8]> for ESGroupTyped {
    fn parse_allocated2(header: ESGroupHeader, raw: &[u8]) -> IResult<&[u8], Self> {
        todo!()
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

    fn is_group(&self) -> bool {
        true
    }
}

// ====================================================================================================


pub fn alloc_group(i: &[u8]) -> IResult<&[u8], (ESGroupHeader, &[u8])> {
    // Parse the header
    let (i, header) = ESGroupHeader::parse(i)?;

    // Debug only: crash if header is does not start with b"GRUP"
    #[cfg(debug_assertions)]
    if &header.iden.0 != b"GRUP" {
        panic!("Fatal Error: Tried to parse non-group ({:?}) as a group.", header.iden);
    }

    // Check if the group is empty before doing the subtraction
    // We are subtracting because the group headers include themselves in the size field
    let take_size = if header.size == 0 { 0 } else { header.size - 24 };

    // Take the bytes
    let (i, raw) = take(take_size as usize)(i)?;

    // Return it
    Ok((i, (header, raw)))
}

// ====================================================================================================

pub trait ESGroupTrait {
    fn group_label(&self) -> ESGroupLabel;
    fn group_size(&self) -> &u32;
}

// ====================================================================================================

/// Implement [ESObject] for anything that implements [ESGroupTrait]
impl ESObject for dyn ESGroupTrait {
    fn object_count(&self) -> &usize { todo!("More logic needs to be fleshed out") }
    fn object_size(&self) -> &u32 { self.group_size() }
    fn is_group(&self) -> bool { true }
}

// ====================================================================================================

#[derive(Debug)]
pub struct ESGroup {
    pub header: ESGroupHeader,
    pub items: Vec<Box<dyn ESObject>>
}

// ====================================================================================================

impl ESGroupTrait for ESGroup {
    fn group_label(&self) -> ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}


impl ESGroup {
    pub fn parse_objects(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, items) = many0(parse_es_object)(raw)?;
        Ok((i, ESGroup { header, items }))
    }
}