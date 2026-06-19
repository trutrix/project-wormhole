use crate::dev::*;

define_record3! {
    "iden": b"DMGT";
    "name": DamageType;
    "fields": [
        EditorId;
        b"DNAM", Data, (u32, u32); // Confusing struct - always 8 bytes but is either a ref or value
    ]
}