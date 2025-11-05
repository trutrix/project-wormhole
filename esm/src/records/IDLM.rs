use crate::dev::*;

define_record! {
    b"IDLM",
    IdleMarker, [
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