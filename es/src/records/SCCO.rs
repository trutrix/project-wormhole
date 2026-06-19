use crate::dev::*;

define_record3! {
    "iden": b"SCCO";
    "name": SceneCollection;
    "fields": [
        EditorId;
        b"QNAM", Quest, FormId;
        b"XNAM", Coordinates, [i32;2];
        b"SNAM", Scene, FormId;
        b"VNAM", Unknown1, [u8;4];
    ]
}