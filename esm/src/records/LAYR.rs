use crate::dev::*;

define_record2! {
    b"LAYR",
    Layer, [
        EditorId;
        b"PNAM", Parent, FormId;
    ]
}