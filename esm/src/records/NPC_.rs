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
    ]
}

/* {NPC_: {DOFT, QNAM, TEND, CS2F, FMIN, 
           EDID, RNAM, AIDT, KSIZ, LTPT, 
           FMRI, SHRT, KWDA, NAM5, DPLT, 
           STCP, CSCR, PRPS, CNAM, DSTD, 
           RCLR, MSDK, DNAM, VMAD, CS2D, 
           MWGT, TPLT, NAM8, HCLF, CS2E, 
           ZNAM, NAM4, FMRS, OBTS, MSDV, 
           FULL, TETI, NAM6, PFRN, DSTF, 
           ECOR, CNTO, PRKR, PRKZ, DATA, 
           APPR, VTCK, CS2H, PTRN, CRIF, 
           PNAM, ATKR, FTST, OBTE, OBTF, 
           SNAM, COCT, OBND, MRSV, WNAM, 
           PKID, SOFT, LTPC, CS2K, FTYP, 
           ACBS, GWOR, NTRM, INAM, DEST, 
           SPCT, ANAM, TPTA, SPLO, STOP
}}*/