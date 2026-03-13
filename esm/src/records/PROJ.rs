use crate::dev::*;

define_record3! {
    "iden": b"PROJ";
    "name": Projectile;
    "fields": [
        EditorId;
        ObjectBounds;
        FullName;
        ModelData;
        Destructible;
        b"DNAM", Data, ProjectileData;
        b"VNAM", SoundLevel, u32; // TODO: enum?
        b"DATA", EmptyData, EmptyParser; // Always zero, maybe a marker
        b"NAM1", SubModelFilePath, ESMString;
        b"NAM2", SubModelInfo, EmptyParser;
    ]
}

#[derive(Debug, NomLE)]
pub struct ProjectileData {
    // TODO: fill out - length 93 bytes (odd number is weird)
}