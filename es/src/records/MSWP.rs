use crate::dev::*;

define_record3! {
    "iden": b"MSWP";
    "name": MaterialSwap;
    "fields": [
        EditorId;
        b"FNAM", TreeFolder, ESMString;
        b"BNAM", OriginalMaterial, ESMString;
        b"SNAM", NewMaterial, ESMString;
        b"CNAM", ColorRemap, Color4;
    ]
}