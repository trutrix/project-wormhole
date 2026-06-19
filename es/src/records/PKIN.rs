use crate::dev::*;

define_record3! {
    "iden": b"PKIN";
    "name": PackIn;
    "fields": [
        EditorId;
        ObjectBounds;
        b"VNAM", Version, u32;
        b"FLTR", PathFilter, ESMString;
        b"CNAM", Cell, FormId;
    ]
}