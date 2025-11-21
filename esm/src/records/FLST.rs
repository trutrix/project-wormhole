use crate::dev::*;

define_record2! {
    b"FLST",
    FormIdList, [
        EditorId;
        FullName;
        b"LNAM", ListItem, FormId;
    ]
}