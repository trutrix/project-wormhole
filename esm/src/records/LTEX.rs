use crate::dev::*;

define_record3! {
    "iden": b"LTEX";
    "name": LandscapeTexture;
    "fields": [
        EditorId;
        b"TNAM", TextureSet, FormId;
        b"MNAM", MaterialSet, FormId;
        b"HNAM", HavokData, [u8;2]; // Havok Friction / Restitution data
        b"SNAM", SpecularExponent, u8;
        b"GNAM", Grass, FormId;
    ]
}