use crate::dev::*;

define_record! {
    b"FURN",
    Furniture, [
        EditorId;
        Keywords;
        AllModelData;
        Destructible;
        ObjectBounds;
        VirtualMachineAdapter;
        FullName;
        PreviewTransform;
        Condition;
        b"CITC", ConditionCount, u32;

        b"PRPS", Properties, Vec<(FormId, f32)>;
        b"NTRM", NativeTerminal, FormId;
        b"STOP", StopMark, EmptyParser;

    ]
}