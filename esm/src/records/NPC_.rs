use project_wormhole_proc::define_record3;

use crate::dev::*;

define_record3! {
    "iden": b"NPC_";
    "name": NonPlayerCharacter;
    "fields": [
        EditorId;
        Keyword;
        VirtualMachineAdapter;
        FullName;
        Destructible;
        ObjectBounds;
        PreviewTransform;

        b"ACBS", Config, EmptyParser;
        b"AIDT", AiData, EmptyParser;
        b"ANAM", FarAwayModel, EmptyParser;
        b"APPR", AttachParentSlots, EmptyParser;
        b"ATKR", AttackRace, FormId;
        b"CNAM", Class, FormId;
        b"CNTO", Item, (FormId, u32);
        b"COCT", ItemCount, u32;
        b"CRIF", CrimeFaction, FormId;
        b"CS2D", Sound, FormId;
        b"CS2E", SoundEndMarker, EmptyParser;
        b"CS2F", SoundFinalize, u8;
        b"CS2H", SoundCount, u32;
        b"CS2K", SoundKeyword, FormId;
        b"CSCR", SoundInherit, FormId;
        b"DATA", DataMarker, EmptyParser;
        b"DNAM", Data, [u16;4];
        b"DOFT", DefaultOutfit, FormId;
        b"DPLT", DefaultPackageList, FormId;
        b"ECOR", CombatOverridePackageList, FormId;
        b"FMIN", FaceMorphIntensity, f32;
        b"FMRI", FaceMorphIndex, u32;
        b"FMRS", FaceMorphValues, [f32;9]; // TODO: make struct
        b"FTST", FaceTexture, FormId;
        b"FTYP", ForcedLocationRefType, FormId;
        b"GWOR", GuardWarning, FormId; // TODO: Unverified
        b"HCLF", HairColor, FormId;
        b"INAM", DeathItem, FormId;
        b"LTPC", LegendaryChance, FormId; // GLOB
        b"LTPT", LegendaryTemplate, FormId;
        b"MRSV", BodyMorphRegionValues, [f32;5]; // TODO: make struct
        b"MSDK", MorphKeys, Vec<f32>;
        b"MSDV", MorphValues, Vec<f32>;
        b"MWGT", Weight, [f32;3];
        b"NAM4", HeightMax, f32;
        b"NAM5", Unknown1, u16;
        b"NAM6", HeightMix, f32;
        b"NAM8", SoundLevel, u32; // TODO: Enum?
        b"NTRM", NativeTerminal, FormId;
        b"OBTE", ObjectCount, u32;
        b"OBTF", ObjectEditorOnly, EmptyParser; // TODO: Always zero, marker?
        b"OBTS", ObjectTemplateItem, EmptyParser; // TODO: Robust struct
        b"PFRN", PowerArmorStand, FormId; // FURN
        b"PKID", Package, FormId; // PACK
        b"PNAM", HeadParts, FormId;
        b"PRKR", Perk, (FormId, u8); // Perk / level
        b"PRKZ", PerkCount, u32;
        Properties;
        b"QNAM", TextureLight, ([u32;3], f32);
        b"RCLR", FollowerElevatorList, FormId; // unverified
        b"RNAM", Race, FormId;
        b"SHRT", ShortName, LocalizedString;
        b"SNAM", Faction, (FormId, u8); // Faction / rank
        b"SOFT", SleepingOutfit, FormId;
        b"SPCT", EffectCount, u32;
        b"SPLO", Effect, FormId;
        b"STCP", AnimationSound, FormId;
        b"STOP", StopMarker, EmptyParser;
        b"TEND", FaceTintData, EmptyParser; // TODO: Data is 1 or 7 bytes long
        b"TETI", FaceTintIndex, [u16;2];
        b"TPLT", DefaultTemplate, FormId;
        b"TPTA", TemplateActors, Vec<FormId>;
        b"VTCK", Voice, FormId;
        b"WNAM", Skin, FormId;
        b"ZNAM", CombatStyle, FormId;
    ]
}
