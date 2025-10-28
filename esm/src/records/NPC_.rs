use crate::dev::*;

// This record is usually compressed
// TODO: Figure out how to decompress within current limits
// For now, just parse the header and skip the rest


define_record! {
    b"NPC_",
    NonPlayerCharacter, [
        EditorId;
        Keywords;
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
        b"PRPS", Property, (FormId, f32); // Property / value
        b"QNAM", TextureLight, ([u32;3], f32);
        b"RCLR", FollowerElevatorList, FormId; // unverified
        b"RNAM", RNAM, EmptyParser;
        b"SHRT", SHRT, EmptyParser;
        b"SNAM", SNAM, EmptyParser;
        b"SOFT", SOFT, EmptyParser;
        b"SPCT", SPCT, EmptyParser;
        b"SPLO", SPLO, EmptyParser;
        b"STCP", STCP, EmptyParser;
        b"STOP", STOP, EmptyParser;
        b"TEND", TEND, EmptyParser;
        b"TETI", TETI, EmptyParser;
        b"TPLT", TPLT, EmptyParser;
        b"TPTA", TPTA, EmptyParser;
        b"VTCK", VTCK, EmptyParser;
        b"WNAM", WNAM, EmptyParser;
        b"ZNAM", ZNAM, EmptyParser;
    ]
}

// "OBTE": [
//       4
//     ],
//     "TEND": [
//       1,
//       7
//     ],
//     "WNAM": [
//       4
//     ],
//     "CS2E": [
//       0
//     ],
//     "GWOR": [
//       4
//     ],
//     "QNAM": [
//       16
//     ],
//     "DSTF": [
//       0
//     ],
//     "VTCK": [
//       4
//     ],
//     "ATKR": [
//       4
//     ],
//     "FMRS": [
//       36
//     ],
//     "DEST": [
//       8
//     ],
//     "PTRN": [
//       4
//     ],
//     "DSTD": [
//       20
//     ],
//     "INAM": [
//       4
//     ],
//     "FULL": [
//       4
//     ],
//     "PKID": [
//       4
//     ],
//     "PFRN": [
//       4
//     ],
//     "DNAM": [
//       8
//     ],
//     "TPLT": [
//       4
//     ],
//     "PRKR": [
//       5
//     ],
//     "NAM8": [
//       4
//     ],
//     "CSCR": [
//       4
//     ],
//     "DOFT": [
//       4
//     ],
//     "ANAM": [
//       4
//     ],
//     "LTPC": [
//       4
//     ],
//     "KWDA": [
//       4,
//       8,
//       24,
//       28,
//       36,
//       20,
//       16,
//       12
//     ],
//     "NAM4": [
//       4
//     ],
//     "MWGT": [
//       12
//     ],
//     "HCLF": [
//       4
//     ],
//     "APPR": [
//       44,
//       12,
//       4,
//       28,
//       24,
//       8
//     ],
//     "DATA": [
//       0
//     ],
//     "CS2F": [
//       1
//     ],
//     "VMAD": [
//       86,
//       4111,
//       4666,
//       198,
//       158,
//       207,
//       54,
//       134,
//       71,
//       144,
//       126,
//       73,
//       97,
//       118,
//       72,
//       56,
//       4541,
//       50,
//       162,
//       30,
//       108,
//       219,
//       4818,
//       110,
//       404,
//       98,
//       269,
//       85,
//       432,
//       101,
//       100,
//       55,
//       87,
//       111,
//       109,
//       49,
//       57,
//       64,
//       174,
//       131,
//       243,
//       431,
//       133,
//       67,
//       52,
//       4854,
//       151,
//       4098,
//       44,
//       1007,
//       171,
//       102,
//       192,
//       750,
//       114,
//       61,
//       4535,
//       106,
//       149,
//       646,
//       154,
//       167,
//       163,
//       94,
//       312,
//       753,
//       53,
//       210,
//       331,
//       225,
//       29,
//       68,
//       262,
//       142,
//       37,
//       214,
//       658,
//       218,
//       244,
//       28,
//       66,
//       132,
//       27,
//       135,
//       4724,
//       268,
//       4529,
//       4568,
//       129,
//       65,
//       74,
//       104,
//       145,
//       756,
//       58,
//       69,
//       177,
//       38,
//       3789,
//       256,
//       136,
//       127,
//       193,
//       392,
//       79,
//       234,
//       182,
//       339,
//       175,
//       581,
//       32,
//       4570,
//       1000
//     ],
//     "PNAM": [
//       4
//     ],
//     "OBTS": [
//       179,
//       182,
//       113,
//       94,
//       154,
//       29,
//       25,
//       151,
//       123,
//       161,
//       144,
//       109,
//       116,
//       46,
//       39,
//       130,
//       115,
//       32,
//       91,
//       60,
//       77,
//       168,
//       158,
//       101,
//       74,
//       84,
//       66,
//       70,
//       99,
//       106,
//       186,
//       165,
//       88,
//       18,
//       137,
//       42,
//       102,
//       95,
//       53,
//       63
//     ],
//     "EDID": [
//       5,
//       4,
//       48,
//       34,
//       7,
//       38,
//       11,
//       33,
//       60,
//       32,
//       6,
//       25,
//       22,
//       23,
//       54,
//       12,
//       39,
//       42,
//       15,
//       10,
//       49,
//       52,
//       21,
//       16,
//       9,
//       8,
//       18,
//       53,
//       57,
//       40,
//       41,
//       30,
//       46,
//       45,
//       29,
//       24,
//       27,
//       14,
//       31,
//       55,
//       28,
//       44,
//       13,
//       35,
//       58,
//       17,
//       26,
//       20,
//       63,
//       50,
//       47,
//       36,
//       43,
//       51,
//       19,
//       37
//     ],
//     "MSDV": [
//       72,
//       40,
//       24,
//       32,
//       44,
//       28,
//       36,
//       4,
//       20,
//       16,
//       12,
//       8,
//       48
//     ],
//     "DPLT": [
//       4
//     ],
//     "STOP": [
//       0
//     ],
//     "MSDK": [
//       36,
//       4,
//       20,
//       32,
//       16,
//       40,
//       72,
//       44,
//       24,
//       48,
//       12,
//       8,
//       28
//     ],
//     "CRIF": [
//       4
//     ],
//     "NAM5": [
//       2
//     ],
//     "MRSV": [
//       20
//     ],
//     "CS2H": [
//       4
//     ],
//     "NTRM": [
//       4
//     ],
//     "CS2K": [
//       4
//     ],
//     "FTST": [
//       4
//     ],
//     "OBTF": [
//       0
//     ],
//     "LTPT": [
//       4
//     ],
//     "RCLR": [
//       4
//     ],
//     "CNAM": [
//       4
//     ],
//     "PRKZ": [
//       4
//     ],
//     "PRPS": [
//       80,
//       16,
//       40,
//       8,
//       88,
//       64,
//       24,
//       120,
//       104,
//       56,
//       96,
//       32,
//       72,
//       112,
//       48
//     ],
//     "AIDT": [
//       24
//     ],
//     "SNAM": [
//       5
//     ],
//     "CS2D": [
//       4
//     ],
//     "NAM6": [
//       4
//     ],
//     "CNTO": [
//       8
//     ],
//     "ACBS": [
//       20
//     ],
//     "OBND": [
//       12
//     ],
//     "SPLO": [
//       4
//     ],
//     "KSIZ": [
//       4
//     ],
//     "COCT": [
//       4
//     ],
//     "FMRI": [
//       4
//     ],
//     "TETI": [
//       4
//     ],
//     "FTYP": [
//       4
//     ],
//     "FMIN": [
//       4
//     ],
//     "SOFT": [
//       4
//     ],
//     "TPTA": [
//       52
//     ],
//     "ECOR": [
//       4
//     ],
//     "SPCT": [
//       4
//     ],
//     "RNAM": [
//       4
//     ],
//     "ZNAM": [
//       4
//     ],
//     "STCP": [
//       4
//     ],
//     "SHRT": [
//       4
//     ]