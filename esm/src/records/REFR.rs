use crate::{dev::*, structs::{geometry::LocationRotation, vectors::Vec2}};


define_record! {
    b"REFR", Reference, [
            EditorId;
            b"XOWN", Owner, FormId;
            b"NAME", BaseObject, FormId;
            b"XSCL", Scale, f32;
            b"DATA", LocationRotation, LocationRotation;
            b"XTEL", DoorTeleport, DoorTeleport;
            b"XNDP", DoorPivot, DoorPivot;
            b"XLRT", LocationReferenceType, FormId;
            b"XLKR", LinkedReference, LinkedReference;
            VirtualMachineAdapter;
            b"XPRM", Primitive, Primitive;
            b"XRFG", ReferenceGroup, FormId;
            b"XLYR", Layer, FormId;
            b"XBSD", Spline, Spline;
            b"XPLK", SplineConnection, FormId;
            b"XRDS", Radius, f32;
            b"XPRD", IdleTime, f32;
            b"XPPA", PatrolMarker, EmptyParser; // Unknown
            b"INAM", Idle, FormId;
            b"PDTO", Unknown1, EmptyParser; // Unknown - size 8 bytes
            b"XPDD", ProjectedDecal, Vec2<f32>;
            b"XATP", ActivationPoint, EmptyParser; // Unknown
            b"XATR", AttachmentReference, FormId;
            b"XESP", EnableParent, EmptyParser; // Shared field
            b"XRGD", Ragdoll, EmptyParser; // Shared field
            b"XEMI", Emitter, FormId;
            b"XLOC", Lock, EmptyParser;
            b"XACT", Action, EmptyParser; // Unknown if this is actually an action
            b"XMSP", MaterialSwap, FormId;
            b"ONAM", Open, EmptyParser;
            b"XIS2", IgnoredSandbox, EmptyParser;
            b"XLIG", LightData, EmptyParser;
            b"XEZN", EncounterZone, FormId;
            b"XRDO", Radio, EmptyParser;
            b"XTRI", CollisionLayer, FormId;
            b"XAPD", ActivateParents, EmptyParser;
            b"XAPR", ActivateParentsReference, EmptyParser;
            b"XLIB", LevelItemBaseObject, FormId;
            b"XWCN", Unknown2, EmptyParser;
            b"XWCU", WaterVelocity, EmptyParser;
            b"XRNK", OwnerFactionRank, i32;
            b"XLKT", LinkedRefTrans, EmptyParser;
            b"XTNM", TeleportLocationName, EmptyParser;
            b"XLCN", PersistantLocation, FormId;
            b"XWPG", PowerGrid1, EmptyParser;
            b"XWPN", PowerGrid2, EmptyParser;
            b"XLCM", LevelModifier, i32;
            b"XHTW", HeadTrackingWeight, f32;
            b"XFVC", FavorCost, f32;
            b"XALP", Alpha, Vec2<u8>;
            b"XAMC", AmmoCount, u32;
            b"XCNT", ItemCount, i32;
            b"XCVL", Unknown3, EmptyParser;
            b"MNAM", PowerComments, ESMString;
            b"XHLT", HealthPercentage, u32;
            b"XMRK", MapMarkerData, EmptyParser; // TODO: Compounded
            b"FNAM", MM1, EmptyParser;
            b"FULL", MM2, EmptyParser; // unsure if this is actually FULL
            b"TNAM", MM3, EmptyParser;
            b"XSPC", SpawnContainer, FormId;
            b"XOCP", OcculsionPlane, EmptyParser; // Size, Loc, Rot
            b"XCZC", CurrentZoneCell, FormId;
            b"XCZA", Unknown4, EmptyParser;
            b"XASP", AcousticRestriction, FormId;
        ]
}


#[derive(Debug, NomLE)]
pub struct DoorTeleport {
    pub door: FormId,
    pub location_rotation: LocationRotation,
    pub flags: u32
}

#[derive(Debug, NomLE)]
pub struct DoorPivot {
    pub nav_mesh: FormId,
    pub triangle_index: u16,
    pub padding: u16
}

#[derive(Debug, NomLE)]
pub struct LinkedReference {
    pub refr: FormId,
    // TODO: it has been reported that this field is sometimes only 4 bytes
}


#[derive(Debug, NomLE)]
pub struct Primitive {
    pub bounds: [f32;3], // Divide by 2
    pub color: [f32;3], // Divide by 255
    pub unknown: f32, // Alpha? 
    pub unknown2: u32 // Visibility flags?
}


#[derive(Debug, NomLE)]
pub struct Spline {
    pub slack: f32,
    pub thickness: f32,
    pub extent_x: f32,
    pub extent_y: f32,
    pub extent_z: f32,
    pub wind: u8
}