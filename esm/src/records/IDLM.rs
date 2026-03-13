use crate::dev::*;

define_record3! {
    "iden": b"IDLM";
    "name": IdleMarker;
    "fields": [
        EditorId;
        Keyword;
        ModelData;
        ObjectBounds;
        b"IDLA", Animations, Vec<FormId>;
        b"IDLF", Flags, u8;
        b"IDLC", AnimationCount, u8;
        b"IDLT", IdleTimerSetting, f32;
        b"QNAM", Unknown1, FormId; // TODO: verify type
    ]
}