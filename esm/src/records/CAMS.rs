use crate::dev::*;

define_record! {
    b"CAMS",
    CameraShot, [
        EditorId;
        ModelData;
        Condition;
        b"DATA", CameraShotData, u8; // TODO: unknown struct
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}