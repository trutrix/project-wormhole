use crate::dev::*;

define_record! {
    b"OTFT",
    Outfit, [
        EditorId;
        b"INAM", Items, Vec<FormId>;
    ]
}