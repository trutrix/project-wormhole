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
        ActivateTextOverride;
        b"NTRM", NativeTerminal, [b"TERM"];
        b"SNAM", SoundLooping, [b"SOUN"];
        

        // Compound destruction fields
        b"DAMC", DamageResistance, u8; // TODO: Find type
        b"FNAM", Flags, u8; // TODO: Find type
        b"FTYP", ForcedLocRefType, u8; // TODO: Find type
        b"PNAM", MarkerColor, u8; // TODO: Find type
        b"RADR", RadioReciever, u8;
        b"STCP", AnimationSound, u8; // TODO: Find type
        b"VNAM", SoundActivation, u8; // TODO: Find type
        b"WNAM", WaterType, u8; // TODO: Find type
    ],
    [
        // Flags - these are just guesses
        0x00000002, NeverFades;
        0x00000004, NonOccluder;
        0x00000040, HeadingMarker;
        0x00000080, MustUpdateAnims;
        0x00000100, HiddenFromLocalMap;
        0x00000200, HeadtrackMarker;
        0x00000400, UsedAsPlatform;
        0x00001000, HasDistantLOD;
        0x00002000, RandomAnimStart;
        0x00004000, Dangerous;
        0x00020000, IgnoreObjectInteraction;
        0x00080000, IsMarker;
        0x00200000, Obstacle;
        0x00400000, NavmeshFilter;
        0x00800000, NavmeshBoundingBox;
        0x02000000, ChildCanUse;
        0x04000000, NavmeshGround;
    ]
}

