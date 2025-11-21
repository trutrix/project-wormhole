use crate::dev::*;

define_record2! {
    b"INNR",
    InstanceNamingRules, [
        EditorId;
        Keyword;
        b"VNAM", NamingCount, u32;
        b"WNAM", Text, LocalizedString;
        b"YNAM", Index, u16;
        b"UNAM", Target, u32; // Unsure if refers to a form ID
    ]
}