use std::collections::HashMap;

use crate::{dev::*, groups::prelude::{CellChildren, RawCellChildren}, prelude::MapContents};

// ====================================================================================================

define_record3! {
    "iden": b"CELL";
    "name": Cell;
    "child_type": CellChildren;
    "fields": [
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

// ====================================================================================================

#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct CombinedReferenceIndex {
    pub mesh_count: u32,
    pub reference_count: u32,
    #[nom(Count = "mesh_count")]
    pub meshes: Vec<u32>,
    #[nom(Count = "reference_count")]
    pub references: Vec<u32>
    // todo
}

// ====================================================================================================

#[derive(Debug, PartialEq)]
pub enum CellLocalWaterLevel {
    NoWater,
    WaterHeight(f32)
}

// ====================================================================================================

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

// ====================================================================================================

#[derive(Debug, NomLE)]
pub struct CombinedReference {
    pub local_id: u32,
    pub ref_id: u32
}

// ====================================================================================================

#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct GridLocation {
    pub x: u32,
    pub y: u32,
    pub flags: u32
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawCellRecord<'esm> {
    pub cell: RawRecord<'esm>,
    pub cell_children: Option<RawCellChildren<'esm>>
}

// ====================================================================================================

impl<'esm> MapContents<HashMap<FormId, RawRecord<'esm>>> for RawCellRecord<'esm> {
    
    fn insert_into_one_map(self, combined_map: &mut HashMap<FormId, RawRecord<'esm>>) {
        if let Some(children) = self.cell_children {
            for group in children.data {
                for block in group.data {
                    combined_map.insert(block.header.form_id, block);
                }
            }
        }
        combined_map.insert(self.cell.header.form_id, self.cell);
    }

    fn insert_into_two_maps(self, data_map: &mut HashMap<FormId, RawRecord<'esm>>, refr_map: &mut HashMap<FormId, RawRecord<'esm>>) {
        if let Some(children) = self.cell_children {
            for group in children.data {
                for block in group.data {
                    refr_map.insert(block.header.form_id, block);
                }
            }
        }
        data_map.insert(self.cell.header.form_id, self.cell);
    }
}

// ====================================================================================================

impl <'esm> Parse<&'esm[u8]> for RawCellRecord<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {

        // Parse the cell record first
        let (i, cell) = RawRecord::parse(i)?;

        // Check if there is any data left after the cell record, if not return immediately
        if i.is_empty() {
            return Ok((i, Self { cell, cell_children: None }));
        }

        // Get the next id
        let (_, next_id) = FourCC::parse(i)?;

        // If the next id isn't a group, return immediately
        if &next_id.0 != b"GRUP" {
            return Ok((i, Self { cell, cell_children: None }));
        }

        let (_, ghead) = GroupHeader::parse(i)?;

        match ghead.label {
            GroupLabel::CellChildren(_) => {
                
                let (i, cell_children) = RawCellChildren::parse(i)?;
                Ok((i, Self { cell, cell_children: Some(cell_children) }))
            }
            _ => {
                //println!("Found non-CellChildren group after Cell record: {:?}, skipping", ghead);
                Ok((i, Self { cell, cell_children: None }))
            }
        }
    }
}