use crate::dev::*;

define_record! {
    b"ASTP",
    AssociationType, [
        EditorId;
        b"MPRT", MaleParentTitle, ESMString;
        b"FPRT", FemaleParentTitle, ESMString;
        b"MCHT", MaleChildTitle, ESMString;
        b"FCHT", FemaleChildTitle, ESMString;
        b"DATA", Flags, AssociationTypeFlags;
    ]
}

#[derive(Debug, NomLE)]
pub struct AssociationTypeFlags;

// Field dump - {FPRT, FCHT, MPRT, MCHT, DATA}