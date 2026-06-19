use crate::dev::*;

define_record3! {
    "iden": b"ASTP";
    "name": AssociationType;
    "fields": [
        EditorId;
        b"MPRT", MaleParentTitle, ESMString;
        b"FPRT", FemaleParentTitle, ESMString;
        b"MCHT", MaleChildTitle, ESMString;
        b"FCHT", FemaleChildTitle, ESMString;
        b"DATA", Flags, AssociationTypeFlags;
    ]
}

#[derive(Debug, NomLE, PartialEq)]
pub struct AssociationTypeFlags;

// Field dump - {FPRT, FCHT, MPRT, MCHT, DATA}