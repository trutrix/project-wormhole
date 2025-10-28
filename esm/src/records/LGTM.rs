use crate::dev::*;

define_record! {
    b"LGTM",
    LightingTemplate, [
        EditorId;
        b"DALC", DirectionalAmbientLightingColor, LightingDirectionalAmbientLightingColor; // TODO: 32 bytes
        b"DATA", Data, LightingTemplateData;
    ]
}


#[derive(Debug, NomLE)]
pub struct LightingTemplateData {

}

#[derive(Debug, NomLE)]
pub struct LightingDirectionalAmbientLightingColor {

}