use crate::dev::*;

define_record2! {
    b"LTEX",
    LandscapeTexture, [
        EditorId;
        b"TNAM", TextureSet, FormId;
        b"MNAM", MaterialSet, FormId;
        b"HNAM", HavokData, [u8;2]; // Havok Friction / Restitution data
        b"SNAM", SpecularExponent, u8;
        b"GNAM", Grass, FormId;
    ]
}