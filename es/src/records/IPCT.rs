use crate::dev::*;

define_record3! {
    "iden": b"IPCT";
    "name": Impact;
    "fields": [
        EditorId;
        ModelData;
        b"DATA", Data, ImpactData;
        b"DNAM", TextureSet1, FormId;
        b"ENAM", TextureSet2, FormId;
        b"FNAM", FootstepMaxParticleDistribution, u8; // Figure out actual type
        b"DODT", DecalData, u8; // TODO: Figure out actual structure 36 bytes
        b"SNAM", Sound1, FormId;
        b"NAM1", Sound2, FormId;
        b"NAM2", SoundHazard, FormId;
        b"NAM3", FootstepExplosion, FormId;
    ]
}



// TODO: Fill out ImpactData structure - Size 24 bytes
#[derive(Debug, NomLE, PartialEq)]
pub struct ImpactData;