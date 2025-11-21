use crate::dev::*;

define_record2! {
    b"DFOB",
    DefaultObject, [
        EditorId;
        b"DATA", Data, FormId; // References several different types of records
    ]
}