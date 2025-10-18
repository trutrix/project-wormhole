use crate::dev::*;

define_record! {
    b"ACTI",
    Activator, [

        EditorId;
        AllModelData;
        ObjectBounds;
        PreviewTransform;

        b"ATTX", ActivateTextOverride, ESMString; // TODO: Verify
        b"CIS1", Condition1, u8; // TODO: Find type
        b"CIS2", Condition2, u8; // TODO: Find type
        b"CITC", ConditionCount, u8; // TODO: Find type
        b"CTDA", UnknownConditionData, u8; // TODO: Find type

        // Compound destruction fields
        b"DAMC", DamageResistance, u8; // TODO: Find type
        b"DEST", Destructable, u8; // TODO: Find type
        b"DMDL", DamageModel, FormId; // TODO: Verify
        b"DMDT", DamageModelData, u8; // TODO: Find type
        b"DSTA", DestructionName, u8; // TODO: Verify
        b"DSTD", DestructionStageData, u8; // TODO: Find type
        b"DSTF", DestructionFlags, u8; // TODO: Find type


        
        b"FNAM", Flags, u8; // TODO: Find type
        b"FTYP", ForcedLocRefType, u8; // TODO: Find type
        FullName;

        // Keywords
        b"KSIZ", KeywordCount, u32; // TODO: Find type
        b"KWDA", Keywords, u8; // TODO: Find type
        

        b"NTRM", NativeTerminal, u8; // TODO: Find type
       
        b"PNAM", MarkerColor, u8; // TODO: Find type
        b"PRPS", Properties, u8; // TODO: Find type
        
        b"RADR", RadioReciever, u8;
        b"SNAM", SoundLooping, FormId; // TODO: Verify
        b"STCP", AnimationSound, u8; // TODO: Find type
        VirtualMachineAdapter;
        b"VNAM", SoundActivation, u8; // TODO: Find type
        b"WNAM", WaterType, u8; // TODO: Find type
    ]
}