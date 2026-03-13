use crate::dev::*;

define_record3! {
    "iden": b"DFOB";
    "name": DefaultObject;
    "fields": [
        EditorId;
        b"DATA", Data, FormId; // References several different types of records
    ]
}