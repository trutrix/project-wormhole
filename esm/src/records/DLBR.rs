use crate::{dev::*, groups::prelude::CellVisibleDistantChildren};


define_record3! {
    "iden": b"DLBR";
    "name": DialogBranch;
    "child_type": CellVisibleDistantChildren;
    "fields": [
        EditorId;
    ]
}