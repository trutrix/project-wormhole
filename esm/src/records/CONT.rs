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
        Keywords;
        b"DSTD", Unknown1, ESMString; // Always 20 bytes - Unknown
        b"DSTF", Unknown2, u32; // Always 0 - Unknown
        b"ONAM", Filters, Vec<FormId>;
        b"SNAM", SoundOpen, FormId;
        b"QNAM", SoundClose, FormId;
        b"TNAM", SoundTake, FormId;
        b"COCT", ItemCount, u32; // Defines how many CNTO fields there are
        b"CNTO", Item, (FormId, u32); // Repeats - Always 8 bytes
        b"DATA", Data, (u8, f32); // Always 5 bytes
        b"FTYP", ForcedLocRefType, u32; // Unknown - Always 4 bytes
        b"NTRM", NativeTerminal, FormId; // References a terminal
        b"DMDT", DestructibleModelData, u8; // TODO - Always 92 bytes
        b"DMDL", DestructibleModelPath, ESMString; // Path to destructible model
        b"PRPS", Properties, Vec<(FormId, f32)>;
        b"DEST", DestructibleHeader, u8; // TODO - Always 8 bytes
    ]
}