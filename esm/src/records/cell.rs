use crate::dev::*;


define_record! {
    b"CELL", Cell, [
        b"EDID", EditorId, ESMString;
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
            return Ok((i, CellLocalWaterLevel::NoWater));
        } else {
            return Ok((i, CellLocalWaterLevel::WaterHeight(height)));
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
pub struct RawCellChildren<'esm> {
    pub header: GroupHeader,
    pub data: &'esm[u8]
}

impl<'esm> Parse<&'esm[u8]> for RawCellChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, RawCellChildren { header, data }))
    }
}