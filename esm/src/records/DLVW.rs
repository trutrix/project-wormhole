use crate::dev::*;


// May be depreciated
// Only a single record appears in Fallout4.esm
// TODO: Research this record more thoroughly

define_record2! {
    b"DLVW",
    DialogView, [
        EditorId;
        b"BNAM", Branches, Vec<FormId>;
        b"QNAM", Quest, FormId;
        b"ENAM", Unknown1, u32;
        b"DNAM", Unknown2, u8;
    ]
}