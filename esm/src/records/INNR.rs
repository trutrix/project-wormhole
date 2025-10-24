use crate::dev::*;

define_record! {
    b"INNR",
    InstanceNamingRules, [
        EditorId;
        Keywords;
        b"VNAM", NamingCount, u32;
        b"WNAM", Text, LocalizedString;
        b"YNAM", Index, u16;
        b"UNAM", Target, u32; // Unsure if refers to a form ID
    ]
}