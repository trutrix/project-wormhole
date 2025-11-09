use crate::dev::*;

define_record! {
    b"SPGD",
    ShaderParticleGeometry, [
        EditorId;
        b"DATA", Data, ShaderParticleGeometryData;
        b"MNAM", MaterialPath, ESMString;
    ]
}


// size: 96 bytes (low sample size)
#[derive(Debug, NomLE)]
pub struct ShaderParticleGeometryData {
    // TODO: fill in
}