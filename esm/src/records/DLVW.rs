use crate::dev::*;


define_record3! {
    "iden": b"DLVW";
    "name": DialogView;
    "fields": [
        EditorId;
        b"BNAM", Branches, Vec<FormId>;
        b"QNAM", ParentQuest, FormId;
        b"TNAM", Topics, FormId;
        b"ENAM", Unknown1, u32;
        b"DNAM", ShowAllText, u8;
    ]
}