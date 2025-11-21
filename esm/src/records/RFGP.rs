use crate::dev::*;

define_record2! {
    b"RFGP",
    ReferenceGroup, [
        EditorId;
        b"NNAM", Name, ESMString;
        b"RNAM", Reference, FormId;
    ]
}