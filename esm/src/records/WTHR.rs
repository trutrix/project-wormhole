use crate::dev::*;

// TODO: this record needs many structs created just to access the data properly

define_record2! {
    b"WTHR",
    Weather, [
        EditorId;
        ModelData;
        b"00TX", Texture0, ESMString;
        b"10TX", Texture1, ESMString;
        b"20TX", Texture2, ESMString;
        b"30TX", Texture3, ESMString;
        b"40TX", Texture4, ESMString;
        b"50TX", Texture5, ESMString;
        b"60TX", Texture6, ESMString;
        b"70TX", Texture7, ESMString;
        b"80TX", Texture8, ESMString;
        b"90TX", Texture9, ESMString;
        b":0TX", Texture10, ESMString;
        b";0TX", Texture11, ESMString;
        b"<0TX", Texture12, ESMString;
        b"=0TX", Texture13, ESMString;
        b">0TX", Texture14, ESMString;
        b"?0TX", Texture15, ESMString;
        b"@0TX", Texture16, ESMString;
        b"A0TX", Texture17, ESMString;
        b"B0TX", Texture18, ESMString;
        b"C0TX", Texture19, ESMString;
        b"D0TX", Texture20, ESMString;
        b"E0TX", Texture21, ESMString;
        b"F0TX", Texture22, ESMString;
        b"G0TX", Texture23, ESMString;
        b"H0TX", Texture24, ESMString;
        b"I0TX", Texture25, ESMString;
        b"J0TX", Texture26, ESMString;
        b"K0TX", Texture27, ESMString;
        b"L0TX", Texture28, ESMString;
        b"M0TX", Texture29, ESMString;
        b"N0TX", Texture30, ESMString;
        b"O0TX", Texture31, ESMString;
        b"LNAM", Unknown1, u32; // TODO: flags or counts?
        b"MNAM", Rain, FormId;
        b"NNAM", Effect, FormId;
        b"RNAM", CloudYSpeeds, [u8;32]; // These are scalars compressed to u8 (127 = 0.0, 255 = 1.0, 0 = -1.0)
        b"QNAM", CloudXSpeeds, [u8;32]; 
        b"PNAM", CloudColors, WeatherCloudColors;
        b"JNAM", CloudAlphas, EmptyParser; // TODO: struct (512 and 1024 bytes)
        b"NAM0", WeatherColors, EmptyParser; // TODO: struct (272, 544, 608 bytes)
        b"NAM1", DisabledCloudLayers, u32; // TODO: bitfield
        b"NAM4", Unknown2, [f32;32]; // TODO: verify purpose
        b"FNAM", FogDistance, EmptyParser; // TODO: struct (72, 56, 32 bytes)
        b"DATA", Data, EmptyParser; // TODO: struct (20, 19 bytes)
        b"VNAM", VolatilityMultiplier, f32;
        b"UNAM", Magic, EmptyParser; // TODO: struct (12, 24 bytes) list?
        b"GNAM", SunGlareLensFlare, FormId;
        b"WNAM", VisibilityMultiplier, f32;
        b"IMSP", ImageSpaces, EmptyParser; // TODO: struct (32 and 16 bytes) list?
        b"DALC", DirectionalAmbientLightColors, EmptyParser; // TODO: struct (32 bytes)
        b"TNAM", SkyStatic, FormId;
        b"SNAM", Sound, (FormId, u32); // TODO: struct with FormId and enum?
        b"WGDR", Godrays, [FormId;8]; // TODO: wrapper struct?
    ]
}

#[derive(Debug, NomLE)]
pub struct WeatherCloudColors {
    // TODO: fill out - length 512 and 1024 bytes
}


// "WTHR": {
//     "DALC": [
//       32
//     ],
//     "90TX": [
//       26,
//       30,
//       28
//     ],
//     "F0TX": [
//       30
//     ],
//     ":0TX": [
//       17,
//       28,
//       15
//     ],
//     "@0TX": [
//       30,
//       26
//     ],
//     "JNAM": [
//       512,
//       1024
//     ],
//     "NAM4": [
//       128
//     ],
//     "<0TX": [
//       28,
//       25,
//       21
//     ],
//     "30TX": [
//       23,
//       35,
//       29,
//       26,
//       30,
//       28,
//       14
//     ],
//     "A0TX": [
//       22,
//       30
//     ],
//     "SNAM": [
//       8
//     ],
//     "TNAM": [
//       4
//     ],
//     "NAM1": [
//       4
//     ],
//     "=0TX": [
//       31,
//       28
//     ],
//     "IMSP": [
//       32,
//       16
//     ],
//     "QNAM": [
//       32
//     ],
//     "B0TX": [
//       22
//     ],
//     "50TX": [
//       17,
//       15
//     ],
//     ">0TX": [
//       28,
//       29
//     ],
//     "WGDR": [
//       32
//     ],
//     "E0TX": [
//       30
//     ],
//     "FNAM": [
//       72,
//       56,
//       32
//     ],
//     "80TX": [
//       28,
//       30,
//       26
//     ],
//     "DATA": [
//       20,
//       19
//     ],
//     ";0TX": [
//       15,
//       17,
//       25,
//       28
//     ],
//     "UNAM": [
//       12,
//       24
//     ],
//     "MODL": [
//       24,
//       26,
//       21
//     ],
//     "00TX": [
//       24,
//       34,
//       35,
//       23,
//       14,
//       28
//     ],
//     "40TX": [
//       25,
//       28,
//       17,
//       18,
//       15
//     ],
//     "RNAM": [
//       32
//     ],
//     "GNAM": [
//       4
//     ],
//     "K0TX": [
//       28
//     ],
//     "EDID": [
//       30,
//       20,
//       24,
//       27,
//       25,
//       29,
//       15,
//       11,
//       26,
//       28,
//       13,
//       22,
//       23,
//       12,
//       14,
//       48,
//       35,
//       16,
//       34,
//       33,
//       18,
//       17,
//       21,
//       19
//     ],
//     "D0TX": [
//       26
//     ],
//     "WNAM": [
//       4
//     ],
//     "MODT": [
//       20,
//       16
//     ],
//     "10TX": [
//       28,
//       32,
//       14,
//       24
//     ],
//     "60TX": [
//       17,
//       15
//     ],
//     "PNAM": [
//       512,
//       1024
//     ],
//     "20TX": [
//       24,
//       14,
//       28
//     ],
//     "LNAM": [
//       4
//     ],
//     "NAM0": [
//       272,
//       544,
//       608
//     ],
//     "VNAM": [
//       4
//     ],
//     "70TX": [
//       17,
//       15
//     ],
//     "L0TX": [
//       25
//     ],
//     "NNAM": [
//       4
//     ],
//     "MNAM": [
//       4
//     ],
//     "?0TX": [
//       30
//     ],
//     "C0TX": [
//       26
//     ]
//   },
//   "SMBN": {
//     "CITC": [
//       4
//     ],
//     "EDID": [
//       12,
//       21,
//       29,
//       24,
//       9,
//       33,
//       16,
//       6,
//       23,
//       10,
//       36,
//       8,
//       30,
//       26,
//       20,
//       19,
//       17,
//       14,
//       25,
//       18,
//       13,
//       28,
//       15,
//       11,
//       22,
//       27
//     ],
//     "CIS2": [
//       22
//     ],
//     "SNAM": [
//       4
//     ],
//     "XNAM": [
//       4
//     ],
//     "PNAM": [
//       4
//     ],
//     "DNAM": [
//       4
//     ],
//     "CTDA": [
//       32
//     ]
//   },
//   "PACK": {
//     "CITC": [
//       4
//     ],
//     "PLDT": [
//       16,
//       12
//     ],
//     "POBA": [
//       0
//     ],
//     "IDLF": [
//       1
//     ],
//     "PFO2": [
//       16
//     ],
//     "INAM": [
//       4
//     ],
//     "IDLT": [
//       4
//     ],
//     "POEA": [
//       0
//     ],
//     "UNAM": [
//       1
//     ],
//     "PKC2": [
//       1
//     ],
//     "CIS2": [
//       32,
//       23,
//       21,
//       19,
//       34,
//       22,
//       27,
//       14,
//       24,
//       30,
//       15,
//       18,
//       20,
//       17,
//       29,
//       16,
//       28,
//       25,
//       26,
//       31
//     ],
//     "CIS1": [
//       15,
//       30,
//       18,
//       24,
//       19,
//       31
//     ],
//     "EDID": [
//       19,
//       47,
//       41,
//       7,
//       38,
//       5,
//       29,
//       64,
//       32,
//       15,
//       45,
//       49,
//       56,
//       55,
//       37,
//       62,
//       18,
//       28,
//       25,
//       34,
//       24,
//       4,
//       46,
//       21,
//       11,
//       52,
//       17,
//       9,
//       23,
//       6,
//       12,
//       26,
//       33,
//       31,
//       43,
//       36,
//       40,
//       50,
//       53,
//       30,
//       44,
//       39,
//       57,
//       14,
//       27,
//       20,
//       48,
//       51,
//       60,
//       13,
//       22,
//       10,
//       16,
//       61,
//       58,
//       54,
//       42,
//       8,
//       35
//     ],
//     "PNAM": [
//       9,
//       12,
//       14,
//       4,
//       8,
//       6,
//       13,
//       5,
//       10,
//       11,
//       7
//     ],
//     "PTDA": [
//       12
//     ],
//     "IDLC": [
//       1
//     ],
//     "POCA": [
//       0
//     ],
//     "PKDT": [
//       12
//     ],
//     "FNAM": [
//       4
//     ],
//     "PKCU": [
//       12
//     ],
//     "PRCB": [
//       8
//     ],
//     "ANAM": [
//       13,
//       10,
//       5,
//       15,
//       9,
//       4,
//       11,
//       8,
//       7,
//       6
//     ],
//     "IDLA": [
//       4,
//       8
//     ],
//     "BNAM": [
//       21,
//       31,
//       26,
//       19,
//       32,
//       7,
//       42,
//       10,
//       34,
//       22,
//       6,
//       13,
//       23,
//       4,
//       28,
//       17,
//       15,
//       38,
//       18,
//       59,
//       41,
//       44,
//       30,
//       61,
//       5,
//       12,
//       14,
//       9,
//       3,
//       29,
//       8,
//       24,
//       40,
//       20,
//       33,
//       56,
//       11,
//       36,
//       27,
//       63,
//       25,
//       16
//     ],
//     "CTDA": [
//       32
//     ],
//     "VMAD": [
//       188,
//       184,
//       140,
//       150,
//       142,
//       154,
//       152,
//       155,
//       292,
//       169,
//       214,
//       145,
//       167,
//       136,
//       243,
//       165,
//       156,
//       130,
//       171,
//       205,
//       170,
//       162,
//       159,
//       247,
//       179,
//       124,
//       183,
//       138,
//       163,
//       220,
//       256,
//       58,
//       218,
//       217,
//       241,
//       132,
//       251,
//       172,
//       160,
//       182,
//       239,
//       189,
//       173,
//       148,
//       144,
//       226
//     ],
//     "QNAM": [
//       4
//     ],
//     "XNAM": [
//       1
//     ],
//     "PSDT": [
//       12
//     ],
//     "CNAM": [
//       1,
//       4
//     ],
//     "PDTO": [
//       8
//     ]
//   },