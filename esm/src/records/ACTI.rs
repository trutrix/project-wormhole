use proc::define_record2;

use crate::dev::*;

define_record2! {
    b"ACTI",
    Activator, [

        EditorId;
        ModelData;
        ObjectBounds;
        PreviewTransform;
        Condition;
        VirtualMachineAdapter;
        Destructible;
        FullName;
        Keyword;
        Properties;

        b"ATTX", ActivateTextOverride, ESMString; // TODO: Verify
        b"CITC", ConditionCount, u8; // TODO: Find type

        // Compound destruction fields
        b"DAMC", DamageResistance, u8; // TODO: Find type
        b"DSTA", DestructionName, u8; // TODO: Verify
        b"FNAM", Flags, u8; // TODO: Find type
        b"FTYP", ForcedLocRefType, u8; // TODO: Find type
        b"NTRM", NativeTerminal, u8; // TODO: Find type
        b"PNAM", MarkerColor, u8; // TODO: Find type
        b"RADR", RadioReciever, u8;
        b"SNAM", SoundLooping, FormId; // TODO: Verify
        b"STCP", AnimationSound, u8; // TODO: Find type
        b"VNAM", SoundActivation, u8; // TODO: Find type
        b"WNAM", WaterType, u8; // TODO: Find type
    ]
}