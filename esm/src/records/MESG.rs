use crate::dev::*;

define_record! {
    b"MESG",
    Message, [
        EditorId;
        Condition;
        FullName;
        Description;

        b"SNAM", SWFFile, ESMString;
        b"QNAM", OwnerQuest, FormId; // QUST
        b"INAM", Icon, FormId; // ICON
        b"DNAM", Flags, u32;
        b"NNAM", ShortName, LocalizedString;
        b"ITXT", ButtonText, LocalizedString;
        b"TNAM", DisplayTime, u32;
    ]
}