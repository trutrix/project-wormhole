use core::panic;
use std::{fmt::Debug, io::Read};


use crate::{dev::*, records::all::*, traits::EditorId};

// ====================================================================================================

#[derive(Debug, NomLE)]
pub struct RecordHeader {
    pub iden: FourCC,
    pub size: u32, // Size NOT INCLUDING header, unlike GroupHeader
    pub flags: RecordFlags,
    pub form_id: FormId,
    pub version_control: VersionControl,
}

// ====================================================================================================

// The information contained in the version control structure appears to be used by a custom Perforce VCM
#[derive(Debug, NomLE)]
pub struct VersionControl {
    pub timestamp: ESMTimestamp,
    pub users: [u8; 2],
    pub form: u16,
    pub revision: u16,
}


// ====================================================================================================



// Assuming the timestamp is the same in Fallout 4 as SkyrimSE
#[derive(NomLE)]
pub struct ESMTimestamp(pub u16);

impl std::fmt::Debug for ESMTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        // Shift over to keep only the year
        let year = self.0   >>   9;

        // Shift over to keep only the month
        // Not working properly, some months are showing as 0
        let month = self.0  >> 5 & 0b00000001111;

        // Mask to keep only the day
        let day = self.0    &   0b0000000000011111;

        write!(f, "{:04}/{:02}/{:02}", year + 2000, month, day)
    }
}

// ====================================================================================================

// Flag positions

// 0x00000001	(TES4) Master (ESM) file
pub const TES4_MASTER: u32 = 0x1;

// 0x00000002
pub const UNKNOWN_FLAG_2: u32 = 0x2;

// 0x00000004
pub const UNKNOWN_FLAG_4: u32 = 0x4;

// 0x00000010	Deleted Group (bugged, see Groups)]
pub const DELETED_GROUP: u32 = 0x10;

// 0x00000020	Deleted Record
pub const DELETED_RECORD: u32 = 0x20;

// 0x00000040
// (GLOB) Constant
// (REFR) Hidden From Local Map (Needs Confirmation: Related to shields)
pub const GLOB_CONSTANT: u32 = 0x40;
pub const REFR_HIDDEN: u32 = 0x40;

// 0x00000080	(TES4) Localized - this will make Skyrim load the .STRINGS, .DLSTRINGS, and .ILSTRINGS files associated with the mod. If this flag is not set, lstrings are treated as zstrings.
pub const TES4_LOCALIZED: u32 = 0x80;

// 0x00000100	Must Update Anims
// (REFR) Inaccessible
pub const MUST_UPDATE_ANIMS: u32 = 0x100;
pub const REFR_INACCESSIBLE: u32 = 0x100;

// 0x00000200
// (TES4) Light Master (ESL) File. Data File
// (REFR) Hidden from local map
// (ACHR) Starts dead
// (REFR) MotionBlurCastsShadows
pub const TES4_LIGHT_MASTER: u32 = 0x200;
pub const REFR_HIDDEN2: u32 = 0x200;
pub const ACHR_STARTS_DEAD: u32 = 0x200;
pub const REFR_MOTION_BLUR_CASTS_SHADOWS: u32 = 0x200;

// 0x00000400
// Quest item
// Persistent reference
// (LSCR) Displays in Main Menu
pub const QUEST_ITEM: u32 = 0x400;
pub const PERSISTENT_REFERENCE: u32 = 0x400;
pub const LSCR_DISPLAYS_IN_MAIN_MENU: u32 = 0x400;

// 0x00000800	Initially disabled
pub const INITIALLY_DISABLED: u32 = 0x800;

// 0x00001000	Ignored
pub const IGNORED: u32 = 0x1000;

// 0x00002000
pub const UNKNOWN_FLAG_2000: u32 = 0x2000;

// 0x00008000	Visible when distant
pub const VISIBLE_WHEN_DISTANT: u32 = 0x8000;

// 0x00010000	(ACTI) Random Animation Start
pub const ACTI_RANDOM_ANIMATION_START: u32 = 0x10000;

// 0x00020000
// (ACTI) Dangerous
// Off limits (Interior cell)
// Dangerous Can't be set without Ignore Object Interaction
pub const ACTI_DANGEROUS: u32 = 0x20000;
pub const OFF_LIMITS: u32 = 0x20000;

// 0x00040000	Data is compressed
pub const COMPRESSED: u32 = 0x40000;

// 0x00080000	Can't wait
pub const CANT_WAIT: u32 = 0x80000;

// 0x00100000
// (ACTI) Ignore Object Interaction
// Ignore Object Interaction Sets Dangerous Automatically
pub const ACTI_IGNORE_OBJECT_INTERACTION: u32 = 0x100000;

// 0x00800000	Is Marker
pub const IS_MARKER: u32 = 0x800000;

// 0x02000000
// (ACTI) Obstacle
// (REFR) No AI Acquire
pub const ACTI_OBSTACLE: u32 = 0x2000000;
pub const REFR_NO_AI_ACQUIRE: u32 = 0x2000000;

// 0x04000000	NavMesh Gen - Filter
pub const NAVMESH_GEN_FILTER: u32 = 0x4000000;

// 0x08000000	NavMesh Gen - Bounding Box
pub const NAVMESH_GEN_BOUNDING_BOX: u32 = 0x8000000;

// 0x10000000
// (FURN) Must Exit to Talk
// (REFR) Reflected By Auto Water
pub const FURN_MUST_EXIT_TO_TALK: u32 = 0x10000000;
pub const REFR_REFLECTED_BY_AUTO_WATER: u32 = 0x10000000;

// 0x20000000
// (FURN/IDLM) Child Can Use
// (REFR) Don't Havok Settle
pub const FURN_CHILD_CAN_USE: u32 = 0x20000000;
pub const IDLM_CHILD_CAN_USE: u32 = 0x20000000;
pub const REFR_DONT_HAVOK_SETTLE: u32 = 0x20000000;

// 0x40000000
// NavMesh Gen - Ground
// (REFR) NoRespawn
pub const NAVMESH_GEN_GROUND: u32 = 0x40000000;
pub const REFR_NORESPAWN: u32 = 0x40000000;

// 0x80000000	(REFR) MultiBound
pub const REFR_MULTIBOUND: u32 = 0x80000000;

#[derive(Clone, Copy, NomLE)]
pub struct RecordFlags(pub u32);

impl std::fmt::Debug for RecordFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_flags() {
            let mut flags = Vec::new();
            if self.has_flag(TES4_MASTER) {
                flags.push("TES4_MASTER")
            }
            if self.has_flag(UNKNOWN_FLAG_2) {
                flags.push("UNKNOWN_FLAG_2")
            }
            if self.has_flag(UNKNOWN_FLAG_4) {
                flags.push("UNKNOWN_FLAG_4")
            }
            if self.has_flag(DELETED_GROUP) {
                flags.push("DELETED_GROUP")
            }
            if self.has_flag(DELETED_RECORD) {
                flags.push("DELETED_RECORD")
            }
            if self.has_flag(GLOB_CONSTANT) {
                flags.push("GLOB_CONSTANT")
            }
            if self.has_flag(REFR_HIDDEN) {
                flags.push("REFR_HIDDEN")
            }
            if self.has_flag(TES4_LOCALIZED) {
                flags.push("TES4_LOCALIZED")
            }
            if self.has_flag(MUST_UPDATE_ANIMS) {
                flags.push("MUST_UPDATE_ANIMS")
            }
            if self.has_flag(REFR_INACCESSIBLE) {
                flags.push("REFR_INACCESSIBLE")
            }
            if self.has_flag(TES4_LIGHT_MASTER) {
                flags.push("TES4_LIGHT_MASTER")
            }
            if self.has_flag(REFR_HIDDEN2) {
                flags.push("REFR_HIDDEN2")
            }
            if self.has_flag(ACHR_STARTS_DEAD) {
                flags.push("ACHR_STARTS_DEAD")
            }
            if self.has_flag(REFR_MOTION_BLUR_CASTS_SHADOWS) {
                flags.push("REFR_MOTION_BLUR_CASTS_SHADOWS")
            }
            if self.has_flag(QUEST_ITEM) {
                flags.push("QUEST_ITEM")
            }
            if self.has_flag(PERSISTENT_REFERENCE) {
                flags.push("PERSISTENT_REFERENCE")
            }
            if self.has_flag(LSCR_DISPLAYS_IN_MAIN_MENU) {
                flags.push("LSCR_DISPLAYS_IN_MAIN_MENU")
            }
            if self.has_flag(INITIALLY_DISABLED) {
                flags.push("INITIALLY_DISABLED")
            }
            if self.has_flag(IGNORED) {
                flags.push("IGNORED")
            }
            if self.has_flag(UNKNOWN_FLAG_2000) {
                flags.push("UNKNOWN_FLAG_2000")
            }
            if self.has_flag(VISIBLE_WHEN_DISTANT) {
                flags.push("VISIBLE_WHEN_DISTANT")
            }
            if self.has_flag(ACTI_RANDOM_ANIMATION_START) {
                flags.push("ACTI_RANDOM_ANIMATION_START")
            }
            if self.has_flag(ACTI_DANGEROUS) {
                flags.push("ACTI_DANGEROUS")
            }
            if self.has_flag(OFF_LIMITS) {
                flags.push("OFF_LIMITS")
            }
            if self.has_flag(COMPRESSED) {
                flags.push("COMPRESSED")
            }
            if self.has_flag(CANT_WAIT) {
                flags.push("CANT_WAIT")
            }
            if self.has_flag(ACTI_IGNORE_OBJECT_INTERACTION) {
                flags.push("ACTI_IGNORE_OBJECT_INTERACTION")
            }

            if self.has_flag(IS_MARKER) {
                flags.push("IS_MARKER")
            }
            if self.has_flag(ACTI_OBSTACLE) {
                flags.push("ACTI_OBSTACLE")
            }
            if self.has_flag(REFR_NO_AI_ACQUIRE) {
                flags.push("REFR_NO_AI_ACQUIRE")
            }
            if self.has_flag(NAVMESH_GEN_FILTER) {
                flags.push("NAVMESH_GEN_FILTER")
            }
            if self.has_flag(NAVMESH_GEN_BOUNDING_BOX) {
                flags.push("NAVMESH_GEN_BOUNDING_BOX")
            }
            if self.has_flag(FURN_MUST_EXIT_TO_TALK) {
                flags.push("FURN_MUST_EXIT_TO_TALK")
            }
            if self.has_flag(REFR_REFLECTED_BY_AUTO_WATER) {
                flags.push("REFR_REFLECTED_BY_AUTO_WATER")
            }
            if self.has_flag(FURN_CHILD_CAN_USE) {
                flags.push("FURN_CHILD_CAN_USE")
            }
            if self.has_flag(IDLM_CHILD_CAN_USE) {
                flags.push("IDLM_CHILD_CAN_USE")
            }
            if self.has_flag(REFR_DONT_HAVOK_SETTLE) {
                flags.push("REFR_DONT_HAVOK_SETTLE")
            }
            if self.has_flag(NAVMESH_GEN_GROUND) {
                flags.push("NAVMESH_GEN_GROUND")
            }
            if self.has_flag(REFR_NORESPAWN) {
                flags.push("REFR_NORESPAWN")
            }
            if self.has_flag(REFR_MULTIBOUND) {
                flags.push("REFR_MULTIBOUND")
            }
            if flags.is_empty() {
                panic!("Unrecognized flags: {:x}", self.0)
            } else {
                write!(f, "{}", flags.join(", "))
            }
        } else {
            write!(f, "NONE")
        }
    }
}

impl RecordFlags {
    pub fn has_flags(&self) -> bool {
        self.0 != 0
    }
    pub fn has_flag(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }
    pub fn is_compressed(&self) -> bool {
        self.has_flag(COMPRESSED)
    }
}


// ====================================================================================================

pub struct RawRecord<'esm> {
    pub header: RecordHeader,
    pub data: &'esm[u8],
}

impl<'nom> Parse<&'nom [u8]> for RawRecord<'nom> {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self> {
        let (i, (header, data)) = alloc_record(i)?;
        Ok((i, Self{ header, data }))
    }
}


impl Debug for RawRecord<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RawRecord {{ header: {:?}, data: [{} bytes]}}",
            self.header,
            self.data.len()
        )
    }
}



// ====================================================================================================

pub fn alloc_record(i: &[u8]) -> IResult<&[u8], (RecordHeader, &[u8]), nom::error::Error<&[u8]>> {
    let orig = i;
    let (i, header) = RecordHeader::parse(i)?;
    let (i, raw) = take(header.size)(i)?;
    if &header.iden.0 == b"GRUP" {
        let (_, gheader) = GroupHeader::parse(orig)?;
        panic!("alloc_record(): function encountered a group: {:?}", gheader);
    } else {
        Ok((i, (header, raw)))
    }
    
}


// ====================================================================================================


#[derive(Debug)]
pub struct Record<T> {
    pub header: RecordHeader,
    pub fields: Vec<T>
}


impl<T: for<'nom> Parse<&'nom[u8]>> Parse<&[u8]> for Record<T> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;

        if header.flags.is_compressed() {
            if let Ok(dec) = decompress_record(raw) {
                
                if let Ok((_, fields)) = many0(complete(T::parse))(&dec) {
                    return Ok((i, Self { header, fields }));
                } else {
                    return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
                }
                
            } else {
                panic!("Could not decompress record: {:?}", header);
            }
            
        } else {
            if let Ok((_, fields)) = many0(complete(T::parse))(&raw) {
                return Ok((i, Self { header, fields }));
            } else {
                return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
            }
        }       
    }
}


// ====================================================================================================

/// Parse the u32 for the real size, then decompress the zlib
pub fn decompress_record(i: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    
    if let Ok((i, real_size)) = le_u32::<&[u8], nom::error::Error<&[u8]>>(i) {
        let mut buf = Vec::with_capacity(real_size as usize);
        let mut dec = flate2::bufread::ZlibDecoder::new(i);
        dec.read_to_end(&mut buf)?;

        Ok(buf)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "decompress_record(): could not get real size"))
    }

}


// ====================================================================================================

#[derive(Debug)]
pub struct RawCellRecord<'esm> {
    pub cell: RawRecord<'esm>,
    pub cell_children: Option<RawCellChildren<'esm>>
}

impl RawCellRecord<'_> {
    pub fn has_children(&self) -> bool {
        self.cell_children.is_some()
    }
}

impl <'esm, 'nom> Parse<&'nom[u8]> for RawCellRecord<'esm> where 'nom:'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, cell) = RawRecord::parse(i)?;
        println!("{:?}", cell);
        let (_, ghead) = GroupHeader::parse(i)?;

        match ghead.label {
            GroupLabel::CellChildren(_) => {
                let (i, cell_children) = RawCellChildren::parse(i)?;
                return Ok((i, Self { cell, cell_children: Some(cell_children) }));
            }
            _ => {
                return Ok((i, Self { cell, cell_children: None }))
            }
        }

    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawWorldRecord<'esm> {
    pub world: RawRecord<'esm>,
    pub world_children: Option<RawWorldChildren<'esm>>
}

impl RawWorldRecord<'_> {
    pub fn has_children(&self) -> bool {
        self.world_children.is_some()
    }
}


impl <'esm> Parse<&'esm[u8]> for RawWorldRecord<'esm>  {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {
        let (i, world) = RawRecord::parse(i)?;

        let (_, ghead) = GroupHeader::parse(i)?;

        match ghead.label {
            GroupLabel::WorldChildren(_) => {
                let (i, world_children) = RawWorldChildren::parse(i)?;
                return Ok((i, Self { world, world_children: Some(world_children) }));
            }
            _ => {
                return Ok((i, Self { world, world_children: None }))
            }
        }

    }
}

impl EditorId for RawWorldRecord<'_> {
    fn try_get_editor_id(&self) -> Option<ESMString> {
        let mut edid = None;
        let (_, fields) = many0(complete(RawField::parse))(self.world.data).expect("Could not parse fields from world record.");
        for field in fields {
            match &field.header.iden().0 {
                b"EDID" => {
                    let (_, s) = ESMString::parse(field.data).unwrap();
                    edid = Some(s);
                }
                _ => {}
            }
        }
        edid
    }
}

// ====================================================================================================


#[derive(Debug)]
pub struct RawQuestRecord<'esm> {
    pub quest: RawRecord<'esm>,
    pub quest_children: Option<RawCellVisibleDistantChildren<'esm>>
}
impl RawQuestRecord<'_> {
    pub fn has_children(&self) -> bool {
        self.quest_children.is_some()
    }
}

impl <'esm> Parse<&'esm[u8]> for RawQuestRecord<'esm>  {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {
        
        // Parse the quest record first
        let (i, quest) = RawRecord::parse(i)?;
        


        // If the next thing isn't a group, return immediately
        let (_, next_id) = FourCC::parse(i)?;
        if &next_id.0 != b"GRUP" {
            return Ok((i, Self { quest, quest_children: None }));
        }

        // Treat next as a group and check if it belongs to this quest record
        let (_, ghead) = GroupHeader::parse(i)?;
        match ghead.label {
            GroupLabel::CellVisibleDistantChildren(_) => {
                let (i, quest_children) = RawCellVisibleDistantChildren::parse(i)?;
                return Ok((i, Self { quest, quest_children: Some(quest_children) }));
            }
            _ => {
                return Ok((i, Self { quest, quest_children: None }))
            }
        }
    }
}


// ====================================================================================================