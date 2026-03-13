use crate::dev::*;

define_record3! {
    "iden": b"FSTP";
    "name": Footstep;
    "fields": [
        EditorId;
        b"DATA", ImpactData, FormId;
        b"ANAM", Tag, ESMString;
    ]
}