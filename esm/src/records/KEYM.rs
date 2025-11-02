use crate::dev::*;

define_record! {
    b"KEYM",
    Key, [
        EditorId;
        VirtualMachineAdapter;
        ObjectBounds;
        PreviewTransform;
        FullName;
        Keyword;
        AllModelData;
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