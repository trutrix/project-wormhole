use crate::dev::*;

define_record! {
    b"LAYR",
    Layer, [
        EditorId;
        b"PNAM", Parent, FormId;
    ]
}