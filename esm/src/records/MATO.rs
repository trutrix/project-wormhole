use crate::dev::*;

define_record3! {
    "iden": b"MATO";
    "name": MaterialObject;
    "fields": [
        EditorId;
        ModelData;
        b"DATA", Data, MaterialObjectData;
        b"DNAM", PropertyData, EmptyParser; // TODO: unknown, strange sizes - may be binary
    ]
}

#[derive(Debug, NomLE)]
pub struct MaterialObjectData {
    // TODO: fill out
}