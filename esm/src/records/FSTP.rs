use crate::dev::*;

define_record2! {
    b"FSTP",
    Footstep, [
        EditorId;
        b"DATA", ImpactData, FormId;
        b"ANAM", Tag, ESMString;
    ]
}