use crate::dev::*;

define_record3! {
    "iden": b"CAMS";
    "name": CameraShot;
    "fields": [
        EditorId;
        ModelData;
        Condition;
        b"DATA", CameraShotData, u8; // TODO: unknown struct
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}