use crate::dev::*;

define_record! {
    b"LCTN",
    Location, [
        EditorId;
        FullName;
        Keywords;
        b"LCPR", PersistentRefs, Vec<LocationPersistentRef>;
        b"LCEC", EncounterCell, Vec<LocationEncounterCell>;
        b"LCID", CellMarkerRef, Vec<FormId>;
        b"LCSR", CellStaticRefs, Vec<LocationCellStaticRef>;
        b"LCEP", CellEnablePoints, Vec<LocationCellEnablePoint>;
        b"LCUN", CellUnique, Vec<LocationCellUnique>;
        // there is more LC** fields but they are not in FO4
        b"CNAM", Color, Color4;
        b"PNAM", Parent, FormId;
        b"RNAM", WorldRadius, f32;
        b"ANAM", ActorFadeMultiplier, f32;
        b"MNAM", WorldMarkerRefr, FormId;
        b"NAM1", Music, FormId;
    ]
}

#[derive(Debug, NomLE)]
pub struct LocationPersistentRef {
    pub actor: FormId,
    pub location: FormId,
    pub grid_x: i16,
    pub grid_y: i16,
}

#[derive(Debug, NomLE)]
pub struct LocationEncounterCell {
    pub location: FormId,
    pub positions: Vec<[i16; 2]>
}

#[derive(Debug, NomLE)]
pub struct LocationCellStaticRef {
    pub ref_type: FormId,
    pub marker: FormId,
    pub location: FormId,
    pub grid_x: i16,
    pub grid_y: i16,
}

#[derive(Debug, NomLE)]
pub struct LocationCellEnablePoint {
    pub actor: FormId,
    pub refr: FormId,
    pub grid_x: i16,
    pub grid_y: i16
}

#[derive(Debug, NomLE)]
pub struct LocationCellUnique {
    pub actor: FormId,
    pub refr: FormId,
    pub location: FormId
}