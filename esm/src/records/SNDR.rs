use crate::dev::*;

define_record3! {
    "iden": b"SNDR";
    "name": SoundDescriptor;
    "fields": [
        EditorId;
        Condition;
        b"SNAM", AlternateSound, FormId;
        b"ANAM", SoundFilePath, ESMString;
        b"ITME", MarkerEnd, EmptyParser;
        b"ITMS", MarkerStart, EmptyParser;
        b"LNAM", Values, u32; // TODO: bitfields
        b"GNAM", Category, FormId;
        b"BNAM", Data, EmptyParser; // TODO: sizes 4 and 6 observed
        b"CNAM", DescType, u32; // TODO: enum
        b"ONAM", OutputModel, FormId;
        b"FNAM", Unknown1, ESMString; // TODO: seems to be always 72 bytes and higher with no alignment
        b"DNAM", Descriptor, FormId;
        b"INTV", Unknown2, u32; // Always 4 bytes
        b"ITMC", ItemCount, u32; // TODO: verify type
    ]
}