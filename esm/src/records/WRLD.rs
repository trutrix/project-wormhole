use crate::{dev::*, groups::prelude::WorldChildren, structs::geometry::CellLoc};


define_record3! {
    "iden":b"WRLD";
    "name": Worldspace;
    "child_type": WorldChildren;
    "fields": [
            EditorId;
            // b"CNAM", Color, u32; // Duplicate field
            b"ZNAM", Music, FormId;

            b"CLSZ", CellSizeData, u8; //TODO
            b"CNAM", Climate, FormId;
            b"DATA", Flags, u8;
            b"DNAM", DefaultHeight, Vec<f32>;

            b"ICON", MapImage, ESMString;

            b"MNAM", MapData, MapData; // TODO

            b"NAM2", Water, FormId;
            b"NAM3", LODWaterType, FormId;
            b"NAM4", LODWaterHeight, FormId;
            b"NAMA", DistantLODMultiplier, f32;
    
            b"NAM0", SizeMin, [f32;2];
            b"NAM9", SizeMax, [f32;2];

            b"OFST", AbsoluteData, u8; // TODO
            b"ONAM", WorldOffsetData, WorldOffsetData; //TODO

            b"PNAM", UseFlags, u16;
            b"WCTR", CenterCell, [u16;2];
            b"RNAM", LocIdRef, WorldRNAM; //TODO
            FullName;
            b"MHDT", MaxHeightData, MaxHeightDataWorld;
            b"WNAM", ParentWorldspace, FormId;

            b"XLCN", Location, FormId;
            b"XWEM", WaterEnvironmentMap, ESMString;
            b"WLEV", WaterLevelData, u8; // TODO
        ]
}

#[derive(Debug, NomLE)]
pub struct WorldEntry {
    pub worldspace: Worldspace,
    pub children: WorldChildren
}


// -- Worldspace specific field structs

#[derive(NomLE, PartialEq, Eq)]
pub struct WorldRNAM {
    pub loc: CellLoc,
    #[nom(LengthCount = "le_u32")]
    pub references: Vec<WorldIdLoc>
}

impl std::fmt::Debug for WorldRNAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location: {:?}, Verbose reference array", self.loc)
    }
}

#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct WorldIdLoc {
    pub form_id: FormId,
    pub loc: CellLoc
}


#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct MaxHeightDataWorld {
    pub min: [i16;2],
    pub max: [i16;2],
    pub cell_data: WorldCellData
}

#[derive(NomLE, PartialEq, Eq)]
pub struct WorldCellData(pub Vec<WorldQuadHeight>);

impl std::fmt::Debug for WorldCellData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data too verbose.")
    }
}

#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct WorldQuadHeight {
    pub bottom_left: u8,
    pub bottom_right: u8,
    pub top_left: u8,
    pub top_right: u8
}


#[derive(Debug, NomLE, PartialEq)]
pub struct WorldOffsetData {
    pub scale: f32, 
    pub x: f32, 
    pub y: f32, 
    pub z: f32
}

#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct MapData {
    pub width: i32, 
    pub height: i32, 
    pub top_left: [i16;2], 
    pub bottom_right: [i16;2]
}