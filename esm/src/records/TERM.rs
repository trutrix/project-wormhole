use crate::dev::*;

define_record2! {
    b"TERM",
    Terminal, [
        EditorId;
        Properties;
        FullName;
        Condition;
        VirtualMachineAdapter;
        Keyword;
        ModelData;
        ObjectBounds;
        b"MNAM", MarkersFlags, u32; // TODO: unsure what these flags do
        b"NAM0", HeaderText, FormId;
        b"CNTO", HoloTapeQuantity, (FormId, u32);
        b"FNAM", Unknown1, u16; // TODO: unknown field - size 2 bytes
        b"ANAM", MenuItemType, u8; // TODO: enum or flags
        b"WNAM", WelcomeText, LocalizedString;
        b"RNAM", MenuResponseText, LocalizedString;
        b"TNAM", SubMenu, FormId;
        b"ITID", MenuItemId, u16;
        b"XMRK", MarkerModelPath, ESMString;
        b"UNAM", MenuDisplayText, LocalizedString;
        b"PNAM", Unknown2, u32; // TODO: unknown field - size 4 bytes
        b"ITXT", MenuItemText, LocalizedString;
        b"COCT", HoloTapeCount, u32;
        b"SNAM", MarkerParameters, EmptyParser; // TODO: size 24 bytes
        b"BSIZ", BodyTextCount, u32;
        b"BTXT", BodyText, LocalizedString;
        b"ISIZ", MenuItemCount, u32;
        b"WBDT", WorkbenchData, u8;
    ]
}