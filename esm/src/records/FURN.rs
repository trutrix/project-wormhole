use crate::dev::*;

// This has an empty stop record, implying special parsing is needed.


define_record! {
    b"FURN",
    Furniture, [
        EditorId;
        Keywords;
        AllModelData;
        Destructible;
        ObjectBounds;
        VirtualMachineAdapter;
        FullName;
        PreviewTransform;
        Condition;
        b"CITC", ConditionCount, u32;

        b"PRPS", Properties, Vec<(FormId, f32)>;
        b"NTRM", NativeTerminal, FormId;
        b"STOP", StopMark, EmptyParser; // Marker for end of object template
        b"ENAM", MarkerIndex, u32;
        b"WBDT", WorkbenchData, Vec<u8>; // One or two bytes
        b"NAM0", Unknown1, u32; // Always 4 bytes
        b"MNAM", ActiveMarker, (u16, u16); // Always 4 bytes - 2 values - TODO: validate
        b"FNAM", Flags, u16; // TODO: bitfield
        b"SNAM", MarkerParameters, EmptyParser; // TODO: variable length struct
        b"CNTO", Unknown2, EmptyParser; // TODO: 8 bytes
        b"PNAM", Unknown3, EmptyParser; // TODO: 4 bytes
        b"NAM1", Unknown4, EmptyParser; // TODO: 4 bytes
        b"ATTX", ActivateTextOverride, LocalizedString;
        b"XMRK", MarkerModel, ESMString;
        b"NVNM", Navmesh, EmptyParser; // TODO: 4 bytes
        b"FNPR", MarkerEntryPoint, EmptyParser; // TODO: small struct
        b"OBTS", ObjectModTemplate, EmptyParser; // TODO: variable length struct
        b"OBTE", ObjectModCount, u32;
        b"FTYP", ForcedLocationRefType, EmptyParser; // TODO: 4 bytes
        b"APPR", AttachParentRef, Vec<FormId>;
    ]
}