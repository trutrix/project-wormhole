use crate::dev::*;

define_record2! {
    b"SCCO",
    SceneCollection, [
        EditorId;
        b"QNAM", Quest, FormId;
        b"XNAM", Coordinates, [i32;2];
        b"SNAM", Scene, FormId;
        b"VNAM", Unknown1, [u8;4];
    ]
}