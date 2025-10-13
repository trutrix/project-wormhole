use crate::dev::*;

define_record! {
    b"AVIF",
    ActorValueInformation, [
        EditorId;
        FullName; // String ref?
        b"DESC", Description, ESMString; // String?
        b"ANAM", Abbreviation, ESMString; // String?
        b"NAM0", DefaultValue, f32; // TODO: Verify type
        b"NAM1", Type, u8; // TODO: Verify type
        b"AVFL", Flags, u32; // Bitfield?
    ]
}

// Field dump - {AVFL, DESC, NAM0, NAM1, FULL, ANAM}