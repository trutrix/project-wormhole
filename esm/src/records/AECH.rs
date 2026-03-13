use crate::dev::*;


// This record reuses fields with the same names and puts them in an array
// May require special handling to keep things in order

define_record3! {
    "iden": b"AECH";
    "name": AudioEffectChain; 
    "fields": [
        EditorId;
        b"KNAM", Keyword, u32;
        b"DNAM", DNAM, ESMString;
    ]
}