use crate::dev::*;

define_record3! {
    "iden": b"RFGP";
    "name": ReferenceGroup;
    "fields": [
        EditorId;
        b"NNAM", Name, ESMString;
        b"RNAM", Reference, FormId;
    ]
}