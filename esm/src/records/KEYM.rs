use crate::dev::*;

define_record! {
    b"KEYM",
    Key, [
        EditorId;
        VirtualMachineAdapter;
        ObjectBounds;
        PreviewTransform;
        FullName;
        Keywords;
        AllModelData;
        Destructible;
        b"DATA", Data, KeyData;
        b"YNAM", PickUpSound, FormId;
        b"ZNAM", PutDownSound, FormId;
    ]
}


#[derive(Debug, NomLE)]
pub struct KeyData {
    pub value: u32,
    pub weight: f32
}