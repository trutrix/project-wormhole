use crate::dev::*;

define_record3! {
    "iden": b"LIGH";
    "name": Light;
    "fields": [
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


#[derive(Debug, NomLE, PartialEq)]
pub struct LightData {
    // TODO: fill out
}