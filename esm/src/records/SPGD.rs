use crate::dev::*;

define_record3! {
    "iden": b"SPGD";
    "name": ShaderParticleGeometry;
    "fields": [
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