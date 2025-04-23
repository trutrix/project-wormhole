use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"ACTI",
    Activator, [
        b"EDID", EditorId, ESMString;
        b"OBND", ObjectBounds, ObjectBounds;
    ]
}