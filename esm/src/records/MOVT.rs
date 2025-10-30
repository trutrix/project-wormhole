use crate::dev::*;

define_record! {
    b"MOVT",
    MovementType, [
        EditorId;
        b"MNAM", Name, ESMString;
        b"JNAM", FloatHeight, f32;
        b"LNAM", FlightAngleGain, f32;
        b"INAM", AnimChangeThresholds, [f32;3]; // Directional, Movement, Rotation
        b"SPED", Data, MovementTypeData;
    ]
}


#[derive(Debug, NomLE)]
pub struct MovementTypeData {
    //TODO: fill out
}