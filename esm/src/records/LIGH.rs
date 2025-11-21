use crate::dev::*;

define_record2! {
    b"LIGH",
    Light, [
        EditorId;
        ObjectBounds;
        PreviewTransform;
        VirtualMachineAdapter;
        ModelData;
        Keyword;
        Destructible;
        FullName;
        Properties;
        b"DATA", Data, LightData;
        b"FNAM", FadeValue, f32;
        b"NAM0", Gobo, ESMString;
        b"WGDR", GodRays, FormId;
        b"LNAM", Lens, FormId;
    ]
}


#[derive(Debug, NomLE)]
pub struct LightData {
    // TODO: fill out
}