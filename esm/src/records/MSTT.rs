use crate::dev::*;

define_record! {
    b"MSTT",
    MoveableStatic, [
        EditorId;
        VirtualMachineAdapter;
        ObjectBounds;
        PreviewTransform;
        FullName;
        AllModelData;
        Destructible;
        Keywords;
        Properties;
        b"DATA", OnLocalMap, u8;
        b"SNAM", LoopingSound, FormId;
    ]
}