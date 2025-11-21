use crate::dev::*;

define_record2! {
    b"BNDS",
    BendableSpline, [
        EditorId;
        ObjectBounds;
        b"DNAM", Data, BendableSplineData;
        b"TNAM", Texture, u8; // TODO: Find value type
    ]
}

#[derive(Debug, NomLE)]
pub struct BendableSplineData {

}