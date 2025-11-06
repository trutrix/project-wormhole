use crate::dev::*;

define_record! {
    b"RFGP",
    ReferenceGroup, [
        EditorId;
        b"NNAM", Name, ESMString;
        b"RNAM", Reference, FormId;
    ]
}