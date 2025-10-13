use crate::dev::*;

define_record! {
    b"CAMS",
    CameraShot, [
        EditorId;
        AllModelData;
        b"CTDA", Condition, u8; // TODO: unknown struct
        b"CIS1", ConditionParam1, u8; // TODO: unknown
        b"CIS2", ConditionParam2, u8; // TODO: unknown
        b"DATA", CameraShotData, u8; // TODO: unknown struct
        b"MNAM", ImageSpaceModifier, FormId;
    ]
}