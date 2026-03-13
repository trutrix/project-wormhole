use crate::dev::*;

define_record3! {
    "iden": b"CLMT";
    "name": Climate;
    "fields": [
        EditorId;
        ModelData;
        b"WSLT", WeatherList, Vec<WeatherData>;
        b"FNAM", SunTexture, ESMString;
        b"GNAM", SunGlareTexture, ESMString;
        b"TNAM", SunTiming, u8; // TODO: find struct
    ]
}

#[derive(Debug, NomLE)]
pub struct WeatherData {
    pub weather_id: FormId,
    pub chance: u8,
    pub global_id: FormId
}