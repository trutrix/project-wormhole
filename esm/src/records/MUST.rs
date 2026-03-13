use crate::dev::*;

define_record3! {
    "iden": b"MUST";
    "name": MusicTrack;
    "fields": [
        EditorId;
        Condition;
        b"ANAM", TrackFilePath, ESMString;
        b"LNAM", LoopData, MusicTrackLoopData;
        b"FLTV", Duration, f32;
        b"DNAM", FadeOutTime, f32;
        b"SNAM", Tracks, Vec<FormId>;
        b"FNAM", CuePoints, Vec<f32>;
        b"CNAM", TrackType, u32;
        b"BNAM", FinaleFilePath, ESMString;
    ]
}

#[derive(Debug, NomLE)]
pub struct MusicTrackLoopData {
    pub start: f32,
    pub end: f32,
    pub count: u32
}