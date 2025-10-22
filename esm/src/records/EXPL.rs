use crate::dev::*;

define_record! {
    b"EXPL",
    Explosion, [
        EditorId;
        FullName;
        AllModelData;
        ObjectBounds;
        b"DATA", Data, ExplosionData;
        b"EITM", ObjectEffect, FormId;
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}


#[derive(Debug, NomLE)]
pub struct ExplosionData;