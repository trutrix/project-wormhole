use crate::dev::*;

define_record2! {
    b"KEYM",
    Key, [
        EditorId;
        VirtualMachineAdapter;
        ObjectBounds;
        PreviewTransform;
        FullName;
        Keyword;
        ModelData;
        Destructible;
        PickUpPutDown;
        b"DATA", Data, KeyData;
    ]
}


#[derive(Debug, NomLE)]
pub struct KeyData {
    pub value: u32,
    pub weight: f32
}