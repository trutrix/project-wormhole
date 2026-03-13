use crate::dev::*;

define_record3! {
    "iden": b"LAYR";
    "name": Layer;
    "fields": [
        EditorId;
        b"PNAM", Parent, FormId;
    ]
}