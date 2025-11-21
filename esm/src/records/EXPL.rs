use crate::dev::*;

define_record2! {
    b"EXPL",
    Explosion, [
        EditorId;
        FullName;
        ModelData;
        ObjectBounds;
        b"DATA", Data, ExplosionData;
        b"EITM", ObjectEffect, FormId;
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}


#[derive(Debug, NomLE)]
pub struct ExplosionData;