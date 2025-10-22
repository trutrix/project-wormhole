use crate::dev::*;

define_record! {
    b"DOOR",
    Door, [
        EditorId;
        AllModelData;
        ObjectBounds;
        VirtualMachineAdapter;
        Keywords;
        PreviewTransform;
        FullName;
        Destructible;
        b"SNAM", SoundOpen, FormId;
        b"ANAM", SoundClose, FormId;
        b"BNAM", SoundLoop, FormId;
        b"FNAM", Flags, u8;
        b"ONAM", OpenText, LocalizedString;
        b"CNAM", CloseText, LocalizedString;
    ]
}