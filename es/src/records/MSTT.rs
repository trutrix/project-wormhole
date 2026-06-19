use crate::dev::*;

define_record3! {
    "iden": b"MSTT";
    "name": MoveableStatic;
    "fields": [
        EditorId;
        VirtualMachineAdapter;
        ObjectBounds;
        PreviewTransform;
        FullName;
        ModelData;
        Destructible;
        Keyword;
        Properties;
        b"DATA", OnLocalMap, u8;
        b"SNAM", LoopingSound, FormId;
    ]
}