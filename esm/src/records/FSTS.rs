use crate::dev::*;

define_record2! {
    b"FSTS",
    FootstepSet, [
        EditorId;
        b"DATA", PlayList, Vec<FormId>;
        b"XCNT", Counts, FootstepSetCounts;
    ]
}



#[derive(Debug, NomLE)]
pub struct FootstepSetCounts {
    pub walking: u32,
    pub running: u32,
    pub sprinting: u32,
    pub sneaking: u32,
    pub swimming: u32
}