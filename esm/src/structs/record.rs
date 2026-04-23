use std::{fmt::Debug, io::Read};

use crate::{dev::*, prelude::FormIdTrait};
use bitflags::bitflags;


// ====================================================================================================


/// Size NOT INCLUDING header, unlike [GroupHeader]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RecordHeader {
    pub iden: FourCC,
    pub size: u32,
    pub flags: RecordFlags2,
    pub form_id: FormId,
    pub version_control: VersionControl
}

// ====================================================================================================

impl Parse<&[u8]> for RecordHeader {
    fn parse<'esm>(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, iden) = FourCC::parse(i)?;
        let (i, size) = le_u32(i)?;
        let (i, flags) = RecordFlags2::parse(i)?;
        let (i, form_id) = FormId::parse(i)?;
        let (i, version_control) = VersionControl::parse(i)?;

        Ok((i, Self { iden, size, flags, form_id, version_control }))
    }
}

// ====================================================================================================

/// The information contained in the version control structure appears to be used by a custom Perforce VCM
#[derive(Debug, NomLE, PartialEq, Eq, Clone)]
pub struct VersionControl {
    pub timestamp: ESMTimestamp,
    pub users: [u8; 2],
    pub form: u16,
    pub revision: u16,
}

// ====================================================================================================

/// Assuming the timestamp is the same in Fallout 4 as SkyrimSE. Add 2000 to get full year
/// 
/// Binary format:  
/// ```text
///    YYYYYYY MMMM DDDDD
/// 0b 0000000 0000 11111
/// ```
/// 
#[derive(NomLE, PartialEq, Eq, Clone)]
pub struct ESMTimestamp(pub u16);

// ====================================================================================================

impl ESMTimestamp {
    
    /// Mask out first 11 bits so only day is remaining
    /// 
    /// `self.0 & 0b0000000000011111`
    pub fn day(&self) -> u16 {
        self.0 & 0b0000000000011111
    }

    /// Shift right 5 (to erase day), then mask out first 7 (to erase year)
    /// 
    /// `self.0 >> 5 & 0b00000001111`
    pub fn month(&self) -> u16 {
        self.0 >> 5 & 0b00000001111
    }


    /// Bitshift right 9 to keep only the year.
    /// 
    /// `self.0 >> 9`
    /// 
    /// Add 2000 to this to display the correct millenia
    pub fn year(&self) -> u16 {
        self.0 >> 9
    }
}

// ====================================================================================================

impl ESMTimestamp {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

// ====================================================================================================

impl std::fmt::Debug for ESMTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}/{:02}/{:02}", self.year(), self.month(), self.day())
    }
}

// ====================================================================================================

bitflags! {
    /// Represents a set of flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecordFlags2: u32 {
        /// The value `A`, at bit position `0`.
        
        /// The data is compressed.
        const COMPRESSED = 0x00040000;

        const TES4_MASTER = 0x1;
        const UNKNOWN_FLAG_2 = 0x2;
        const UNKNOWN_FLAG_4 = 0x4;
        const DELETED_GROUP = 0x10;
        const DELETED_RECORD = 0x20;
        const GLOB_CONSTANT = 0x40;
        const REFR_HIDDEN = 0x40;
        const TES4_LOCALIZED = 0x80;
        const MUST_UPDATE_ANIMS = 0x100;
        const REFR_INACCESSIBLE = 0x100;
        const TES4_LIGHT_MASTER = 0x200;
        const REFR_HIDDEN2 = 0x200;
        const ACHR_STARTS_DEAD = 0x200;
        const REFR_MOTION_BLUR_CASTS_SHADOWS = 0x200;
        const QUEST_ITEM = 0x400;
        const PERSISTENT_REFERENCE = 0x400;
        const LSCR_DISPLAYS_IN_MAIN_MENU = 0x400;
        const INITIALLY_DISABLED = 0x800;
        const IGNORED = 0x1000;
        const UNKNOWN_FLAG_2000 = 0x2000;
        const VISIBLE_WHEN_DISTANT = 0x8000;
        const ACTI_RANDOM_ANIMATION_START = 0x10000;
        const ACTI_DANGEROUS = 0x20000;
        const OFF_LIMITS = 0x20000;
        const CANT_WAIT = 0x80000;
        const ACTI_IGNORE_OBJECT_INTERACTION = 0x100000;
        const IS_MARKER = 0x800000;
        const ACTI_OBSTACLE = 0x2000000;
        const REFR_NO_AI_ACQUIRE = 0x2000000;
        const NAVMESH_GEN_FILTER = 0x4000000;
        const NAVMESH_GEN_BOUNDING_BOX = 0x8000000;
        const FURN_MUST_EXIT_TO_TALK = 0x10000000;
        const REFR_REFLECTED_BY_AUTO_WATER = 0x10000000;
        const FURN_CHILD_CAN_USE = 0x20000000;
        const IDLM_CHILD_CAN_USE = 0x20000000;
        const REFR_DONT_HAVOK_SETTLE = 0x20000000;
        const NAVMESH_GEN_GROUND = 0x40000000;
        const REFR_NORESPAWN = 0x40000000;
        const REFR_MULTIBOUND = 0x80000000;
    }
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for RecordFlags2 {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, raw_flags) = le_u32::<&'esm[u8], nom::error::Error<&'esm[u8]>>(i)?;
        Ok((i, RecordFlags2::from_bits_retain(raw_flags)))
    }
}

// ====================================================================================================

impl RecordFlags2 {

    pub fn is_compressed(&self) -> bool {
        self.contains(RecordFlags2::COMPRESSED)
    }
}


// ====================================================================================================

pub struct RawRecord<'esm> {
    pub header: RecordHeader,
    // pub data: RawRecordData<'esm>,
    pub data: &'esm[u8]
}

// ====================================================================================================

impl<'esm> Parse<&'esm [u8]> for RawRecord<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_record(i)?;

        // #[cfg(debug_assertions)]
        // if &header.iden.0 == b"LVLN" {
        //     println!("{:?}", header)
        // }

        Ok((i, Self { header, data }))
    }
}

// ====================================================================================================

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

impl PartialEq for RawRecord<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.data.len() == other.data.len() && self.data == other.data
    }
}

// ====================================================================================================

pub fn alloc_record(i: &[u8]) -> IResult<&[u8], (RecordHeader, &[u8]), nom::error::Error<&[u8]>> {
    // Keep original pointer
    let orig = i;

    // Parse header
    let (i, header) = RecordHeader::parse(i)?;

    // Take size, not including header size
    let (i, raw) = take(header.size)(i)?;

    // Check if header is actually a group, which is an unrecoverable error
    if &header.iden.0 == b"GRUP" {
        let (_, gheader) = GroupHeader::parse(orig)?;
        panic!("alloc_record(): function encountered a group: {:?}", gheader);
    }
    // Return the values
    else {
        Ok((i, (header, raw)))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct Record<T> {
    pub header: RecordHeader,
    pub data: T
}

// ====================================================================================================

impl<T: for<'esm> Parse<&'esm[u8]>> Parse<&[u8]> for Record<T> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;

        if header.flags.is_compressed() {
            if let Ok(dec) = decompress_record(raw) {
                
                if let Ok((_, data)) = T::parse(&dec) {
                    Ok((i, Self { header, data }))
                } else {
                    Err(nom_derive::nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)))
                }
                
            } else {
                panic!("Could not decompress record: {:?}", header);
            }
            
        } else if let Ok((_, data)) = T::parse(raw) {
            Ok((i, Self { header, data }))
        } else {
            Err(nom_derive::nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)))
        }       
    }
}

// ====================================================================================================

impl<T> FormIdTrait for Record<T> {
    fn get_form_id(&self) -> &FormId {
        &self.header.form_id
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