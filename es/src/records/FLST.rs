use crate::dev::*;

define_record3! {
    "iden": b"FLST";
    "name": FormIdList;
    "fields": [
        EditorId;
        FullName;
        b"LNAM", ListItem, FormId;
    ]
}