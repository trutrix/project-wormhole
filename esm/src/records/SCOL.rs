use crate::dev::*;

define_record3! {
    "iden": b"SCOL";
    "name": StaticCollection;
    "fields": [
        EditorId;
        PreviewTransform;
        ModelData;
        ObjectBounds;
        FullName;
        b"FLTR", Filter, ESMString;
        b"ONAM", StaticItem, FormId;
        b"DATA", ItemPlacements, Vec<StaticItemPlacement>;
    ]
}


// TODO: Smallest DATA structure size is 28 bytes, yet it seems to be wildly inaccurate
// Maybe its all halves?
#[derive(Debug, NomLE)]
pub struct StaticItemPlacement {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: f32
}