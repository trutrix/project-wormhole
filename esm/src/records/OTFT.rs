use crate::dev::*;

define_record2! {
    b"OTFT",
    Outfit, [
        EditorId;
        b"INAM", Items, Vec<FormId>;
    ]
}