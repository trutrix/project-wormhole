use crate::dev::*;

define_record3! {
    "iden": b"BNDS";
    "name": BendableSpline;
    "fields": [
        EditorId;
        ObjectBounds;
        b"DNAM", Data, BendableSplineData;
        b"TNAM", Texture, u8; // TODO: Find value type
    ]
}

#[derive(Debug, NomLE)]
pub struct BendableSplineData {

}