use crate::dev::*;

define_record! {
    b"CONT",
    Container, [
        EditorId;
        VirtualMachineAdapter;
        PreviewTransform;
        AllModelData;
        ObjectBounds;
        FullName;
        Keyword;
        Destructible;
        b"ONAM", Filters, Vec<FormId>;
        b"SNAM", SoundOpen, FormId;
        b"QNAM", SoundClose, FormId;
        b"TNAM", SoundTake, FormId;
        b"COCT", ItemCount, u32; // Defines how many CNTO fields there are
        b"CNTO", Item, (FormId, u32); // Repeats - Always 8 bytes
        b"DATA", Data, (u8, f32); // Always 5 bytes
        b"FTYP", ForcedLocRefType, u32; // Unknown - Always 4 bytes
        b"NTRM", NativeTerminal, FormId; // References a terminal
        Properties;
    ]
}