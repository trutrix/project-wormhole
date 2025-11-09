use crate::dev::*;

define_record! {
    b"TACT",
    TalkingActivator, [
        EditorId;
        VirtualMachineAdapter;
        FullName;
        ObjectBounds;
        ModelData;
        b"PNAM", Unknown1, FormId; // TODO: unsure if this is actually a FormId
        b"FNAM", Unknown2, u16; // TODO: unknown field
        b"VNAM", VoiceType, FormId;
    ]
}