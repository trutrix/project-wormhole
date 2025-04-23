use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"AMMO",
    Ammo, [
        b"EDID", EditorId, ESMString;
        b"OBND", ObjectBounds, ObjectBounds;
    ]
}