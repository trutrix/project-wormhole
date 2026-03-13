use crate::dev::*;

define_record3! {
    "iden": b"PACK";
    "name": Package;
    "fields": [
        EditorId;
        VirtualMachineAdapter;
        Condition;
        b"PLDT", Location, PackageLocationData;
        b"POBA", OnBeginMarker, EmptyParser;
        b"POEA", OnEndMarker, EmptyParser;
        b"POCA", OnChangeMarker, EmptyParser;
        
    ]
}

#[derive(Debug, NomLE)]
pub struct PackageLocationData {
    // TODO: fill out - length 12 and 16 observed
}