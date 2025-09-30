use crate::{dev::*, structs::geometry::ObjectBounds};

define_record! {
    b"AMMO",
    Ammo, [
        b"EDID", EditorId, ESMString;
        b"OBND", ObjectBounds, ObjectBounds;
    ]
}


// Field dump - {MODT, ZNAM, DESC, NAM1, MODL, KWDA, NAM2, OBND, PTRN, FULL, YNAM, ONAM, KSIZ, DATA}