use crate::dev::*;

define_record! {
    b"DMGT",
    DamageType, [
        EditorId;
        b"DNAM", Data, (u32, u32); // Confusing struct - always 8 bytes but is either a ref or value
    ]
}