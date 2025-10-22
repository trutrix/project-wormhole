use crate::dev::*;

define_record! {
    b"FLST",
    FormIdList, [
        EditorId;
        FullName;
        b"LNAM", ListItem, FormId;
    ]
}