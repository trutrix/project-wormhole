use crate::dev::*;

define_record! {
    b"BOOK",
    Book, [
        EditorId;
        ObjectBounds;
        PreviewTransform;
        AllModelData;
        Keywords;
        VirtualMachineAdapter;
        b"FULL", FullName, ESMString;
        b"DESC", Description, ESMString;
        b"YNAM", SoundPickUp, FormId;
        b"ZNAM", SoundPutDown, FormId;
        b"CNAM", Description2, ESMString;
        b"INAM", InventoryArt, FormId;
        b"FIMD", FeaturedItemMessage, u8; // TODO: unknown
        b"DNAM", BookData, u8; // TODO: unknown
        b"DATA", BookData2, u8; // TODO: unknown
    ]
}