use crate::dev::*;

define_record2! {
    b"OVIS",
    ObjectVisibility, [
        // No EDID, single record
        b"INDX", Index, FormId;
        b"DATA", ObjectBounds, [f32; 6]; // x1, y1, z1, x2, y2, z2
    ]
}