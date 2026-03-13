use crate::dev::*;

define_record3! {
    "iden": b"TXST";
    "name": TextureSet;
    "fields": [
        EditorId;
        ObjectBounds;
        b"DODT", DecalData, EmptyParser; // TODO: common decal data struct
        b"TX00", TextureDiffuse, ESMString;
        b"TX01", TextureNormal, ESMString;
        b"TX02", TextureGlow, ESMString;
        b"TX03", TextureHeightmap, ESMString;
        b"TX04", TextureEnvironment, ESMString;
        b"TX05", TextureWrinkles, ESMString;
        b"TX06", TextureMultilayer, ESMString;
        b"TX07", TextureSmoothSpecular, ESMString;
        b"MNAM", MaterialPath, ESMString;
        b"DNAM", Flags, TextureSetFlags;
    ]
}


#[derive(Debug, NomLE)]
pub struct TextureSetFlags(pub u16); // TODO: bitflags