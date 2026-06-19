use crate::dev::*;

define_record3! {
    "iden": b"MUSC";
    "name": MusicType;
    "fields": [
        EditorId;
        b"WNAM", FadeDuration, f32;
        b"PNAM", PriorityDucking, (u16, u16); // TODO: ducking appears to be a half-float
        b"TNAM", MusicTracks, Vec<FormId>;
        b"FNAM", Flags, u32;
    ]
}