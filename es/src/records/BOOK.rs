use crate::dev::*;

define_record3! {
    "iden": b"BOOK";
    "name": Book;
    "fields": [
        EditorId;
        ObjectBounds;
        PreviewTransform;
        ModelData;
        Keyword;
        VirtualMachineAdapter;
        FullName;
        Description;
        PickUpPutDown;
        b"CNAM", Description2, ESMString;
        b"INAM", InventoryArt, FormId;
        b"FIMD", FeaturedItemMessage, u8; // TODO: unknown
        b"DNAM", BookData, u8; // TODO: unknown
        b"DATA", BookData2, u8; // TODO: unknown
    ]
}