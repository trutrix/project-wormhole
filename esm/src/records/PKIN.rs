use crate::dev::*;

define_record2! {
    b"PKIN",
    PackIn, [
        EditorId;
        ObjectBounds;
        b"VNAM", Version, u32;
        b"FLTR", PathFilter, ESMString;
        b"CNAM", Cell, FormId;
    ]
}