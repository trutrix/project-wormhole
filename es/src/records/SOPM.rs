use crate::dev::*;

define_record3! {
    "iden": b"SOPM";
    "name": SoundOutputModel;
    "fields": [
        EditorId;
        b"VNAM", StaticAttenuation, u16; // TODO: maybe a half?
        b"ONAM", OutputValues, [u8;24]; // TODO: struct? always 24 bytes
        b"ATTN", AttenuationValues, [u16;12]; // TODO: struct? always 12 bytes, values are halfs?
        b"NAM1", Data, u32; // TODO: verify type
        b"MNAM", Type, u32; // TODO: enum?
        b"ENAM", EffectChain, FormId;
    ]
}