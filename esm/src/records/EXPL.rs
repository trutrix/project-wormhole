use crate::dev::*;

define_record3! {
    "iden": b"EXPL";
    "name": Explosion;
    "fields": [
        EditorId;
        FullName;
        ModelData;
        ObjectBounds;
        b"DATA", Data, ExplosionData;
        b"EITM", ObjectEffect, FormId;
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}


#[derive(Debug, NomLE, PartialEq)]
pub struct ExplosionData;