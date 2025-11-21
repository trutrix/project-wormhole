use crate::dev::*;

define_record2! {
    b"EFSH",
    EffectShader, [
        EditorId;
        ModelData;
        b"DNAM", Data, EffectShaderData;
        b"ICON", FillTexture, ESMString;
        b"ICO2", ParticleShaderTexture, ESMString;
        b"DATA", UnusedData, UnusedData;
        b"NAM7", HolesTexture, ESMString;
        b"NAM8", MembraneTexture, ESMString;
        b"NAM9", ParticlePaletteTexture, ESMString; // Sometimes these are empty
    ]
}


// TODO: Fill this out later
#[derive(Debug, NomLE)]
pub struct EffectShaderData;