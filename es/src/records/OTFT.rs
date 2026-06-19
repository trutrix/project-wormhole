use crate::dev::*;

define_record3! {
    "iden": b"OTFT";
    "name": Outfit;
    "fields": [
        EditorId;
        b"INAM", Items, Vec<FormId>;
    ]
}