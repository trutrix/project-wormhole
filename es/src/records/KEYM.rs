use crate::dev::*;

define_record3! {
    "iden": b"KEYM";
    "name": Key;
    "fields": [
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


#[derive(Debug, NomLE, PartialEq)]
pub struct KeyData {
    pub value: u32,
    pub weight: f32
}