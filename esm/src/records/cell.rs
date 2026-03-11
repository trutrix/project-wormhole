use crate::{dev::*, groups::prelude::CellChildren};


define_record2! {
    b"CELL", Cell, [
        EditorId;
        b"DATA", Flags, u16;
        b"XCLC", GridLocation, GridLocation;
        b"MHDT", MaximumHeightData, u8;
        b"XCRI", CombinedReferenceIndex, CombinedReferenceIndex;
        b"LTMP", LightingTemplate, FormId;
        b"VISI", PreVisTimestamp, u16;
        b"RVIS", PreVisFileOf, FormId;
        b"PCMB", PreCombinedFilesTimestamp, u16;
        b"XCLW", LocalWaterLevel, CellLocalWaterLevel;
        b"XCWT", Water, FormId;
        b"XCLR", Regions, FormId;
        b"XLCN", Location, FormId;
        b"XPRI", PreVisRefIndex, FormId;
    ]
}


#[derive(Debug, NomLE)]
pub struct CombinedReferenceIndex {
    pub mesh_count: u32,
    pub reference_count: u32,
    #[nom(Count = "mesh_count")]
    pub meshes: Vec<u32>,
    #[nom(Count = "reference_count")]
    pub references: Vec<u32>
    // todo
}

#[derive(Debug)]
pub enum CellLocalWaterLevel {
    NoWater,
    WaterHeight(f32)
}

impl Parse<&[u8]> for CellLocalWaterLevel {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, height) = le_f32(i)?;
        if height == f32::MAX {
            Ok((i, CellLocalWaterLevel::NoWater))
        } else {
            Ok((i, CellLocalWaterLevel::WaterHeight(height)))
        }
    }
}

#[derive(Debug, NomLE)]
pub struct CombinedReference {
    pub local_id: u32,
    pub ref_id: u32
}


#[derive(Debug, NomLE)]
pub struct GridLocation {
    pub x: u32,
    pub y: u32,
    pub flags: u32
}


#[derive(Debug)]
pub struct CellEntry {
    pub cell: Cell,
    pub children: Option<CellChildren>
}

impl Parse<&[u8]> for CellEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse the Cell record
        let (i, cell) = Cell::parse(i)?;

        // Check if buffer is consumed (usually at the end of groups)
        if i.len() < 4 {
            return Ok((i, Self { cell, children: None }) )
        }  

        // Peek at the next FourCC to see if it's a GRUP
        let  (_, next_id) = FourCC::parse(i)?;

        // If next iden is not GRUP, there are no children 
        // The groups themselves have pointers to parents, so in theory they could be out of order
        // In practice, they always seem to follow the Cell record directly.

        // Check if next item is a group, if not return with no children
        if &next_id.0 != GRUP {
            return Ok((i, Self { cell, children: None }) )
        }

        // Peek at the next group header to see if it's CellChildren
        let (_, next_header) = GroupHeader::parse(i)?;

        match next_header.label {
            GroupLabel::CellChildren(_) => {
                let (i, children) = CellChildren::parse(i)?;
                Ok((i, Self { cell, children: Some(children) }) )
            }
            _ => {
                Ok((i, Self { cell, children: None }) )
            }
        }
    }
}