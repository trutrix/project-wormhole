use crate::dev::*;

define_record3! {
    "iden": b"NAVI";
    "name": NavigationMeshInfoMap;
    "fields": [
        // No EDID, single record per NAVI
        b"NVER", Version, u32;
        b"NVMI", MapInfo, NavMeshMapInfo;
        b"NVPP", PreferredPathing, NavMeshPreferredPathing;
    ]
}

#[derive(Debug, NomLE)]
pub struct NavMeshMapInfo {
    // TODO: fill out
}


#[derive(Debug, NomLE)]
pub struct NavMeshPreferredPathing {
    // TODO: fill out (verbose)
}