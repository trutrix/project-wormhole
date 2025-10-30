use crate::dev::*;

define_record! {
    b"MSWP",
    MaterialSwap, [
        EditorId;
        b"FNAM", TreeFolder, ESMString;
        b"BNAM", OriginalMaterial, ESMString;
        b"SNAM", NewMaterial, ESMString;
        b"CNAM", ColorRemap, Color4;
    ]
}