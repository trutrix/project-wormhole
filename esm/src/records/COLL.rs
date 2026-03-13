use crate::dev::*;

define_record3! {
    "iden": b"COLL";
    "name": CollisionLayer;
    "fields": [
        EditorId;
        Description; // unsure if localized - seems to only be 4 bytes
        b"FNAM", Color, Color4;
        b"MNAM", Name, ESMString;
        b"INTV", InteractableCount, u32;
        b"CNAM", CollidesWith, Vec<FormId>; // references other layers
        b"BNAM", Index, u32;
        b"GNAM", Flags, u32; // TODO: bitfield
    ]
}