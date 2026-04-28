use crate::{dev::*, groups::prelude::{CellChildren, CellPersistentChildren, CellTemporaryChildren, CellVisibleDistantChildren, ExteriorCellBlock, ExteriorCellSubBlock, InteriorCellBlock, InteriorCellSubBlock, TopGroup, TopicChildren, WorldChildren}};

// ===================================================================================================

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
    Unknown
}

// ===================================================================================================

impl Parse<&[u8]> for ESGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::Top(_) => { 
                Ok((i, ESGroup::Top(TopGroup::parse_pre_alloc(raw, header)?.1))) 
            },
            GroupLabel::WorldChildren(_) => {
                Ok((i, ESGroup::WorldChildren(WorldChildren::parse_pre_alloc(raw, header)?.1)))
            },
            _ => {
                todo!()
            }
        }

    }
}