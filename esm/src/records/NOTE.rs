use crate::dev::*;

define_record! {
    b"NOTE",
    Note, [
        EditorId;
        ObjectBounds;
        ModelData;
        FullName;
        PreviewTransform;
        VirtualMachineAdapter;
        PickUpPutDown;
        b"DATA", ValueWeight, ValueWeight;
        b"PNAM", ProgramFilePath, ESMString;
        b"SNAM", Scene, FormId; // TODO: verify type - always 4 bytes
        b"DNAM", Type, u32; // TODO: enum
    ]
}