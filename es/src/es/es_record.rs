use std::ops::BitAnd;

use nom_derive::nom::error;

use crate::{dev::*, es::{es_group::ESGroupHeader, es_object::ESObjectTrait}, records::*, traits::{ParseAllocated, ParseAllocated2, record::FormIdTrait}};
//use bitflags::bitflags;

#[derive(Debug)]
pub enum ESRecordTyped {
    Unhandled(ESRecordHeader),
    AACT(AACT::Action),
    ACHR(ACHR::ActorReference),
    ACTI(ACTI::Activator),
    ADDN(ADDN::AddonNode),
    AECH(AECH::AudioEffectChain),
    ALCH(ALCH::Alchemy),
    AMDL(AMDL::AimModel),
    AMMO(AMMO::Ammo),
    ANIO(ANIO::AnimatedObject),
    AORU(AORU::AttractionRule),
    ARMA(ARMA::ArmorAddon),
    ARMO(ARMO::Armor),
    ARTO(ARTO::ArtObject),
    ASPC(ASPC::AcousticSpace),
    ASTP(ASTP::AssociationType),
    AVIF(AVIF::ActorValueInformation),
    BNDS(BNDS::BendableSpline),
    BOOK(BOOK::Book),
    BPTD(BPTD::BodyPartData),
    CAMS(CAMS::CameraShot),
    CELL(CELL::Cell),
    CLAS(CLAS::Class),
    CLFM(CLFM::Color),
    CLMT(CLMT::Climate),
    CMPO(CMPO::Component),
    COBJ(COBJ::ConstructibleObject),
    COLL(COLL::CollisionLayer),
    CONT(CONT::Container),
    CPTH(CPTH::CameraPath),
    CSTY(CSTY::CombatStyle),
    DEBR(DEBR::Debris),
    DFOB(DFOB::DefaultObject),
    DIAL(DIAL::Dialog),
    DLBR(DLBR::DialogBranch),
    DLVW(DLVW::DialogView),
    DMGT(DMGT::DamageType),
    DOBJ(DOBJ::DefaultObjects),
    DOOR(DOOR::Door),
    ECZN(ECZN::EncounterZone),
    EFSH(EFSH::EffectShader),
    ENCH(ENCH::ObjectEffect),
    EQUP(EQUP::EquipType),
    EXPL(EXPL::Explosion),
    FACT(FACT::Faction),
    FLOR(FLOR::Flora),
    FLST(FLST::FormIdList),
    FSTP(FSTP::Footstep),
    FSTS(FSTS::FootstepSet),
    FURN(FURN::Furniture),
    GDRY(GDRY::Godray),
    GLOB(GLOB::Global),
    GMST(GMST::GameSetting),
    GRAS(GRAS::Grass),
    HAZD(HAZD::Hazard),
    HDPT(HDPT::HeadPart),
    IDLE(IDLE::IdleAnimation),
    IDLM(IDLM::IdleMarker),
    IMAD(IMAD::ImageSpaceAdapter),
    IMGS(IMGS::ImageSpace),
    INGR(INGR::Ingredient),
    INNR(INNR::InstanceNamingRules),
    IPCT(IPCT::Impact),
    IPDS(IPDS::ImpactDataSet),
    KEYM(KEYM::Key),
    KSSM(KSSM::KeywordSoundMapping),
    KYWD(KYWD::Keyword),
    LAND(LAND::Landscape),
    LAYR(LAYR::Layer),
    LCRT(LCRT::LocationReferenceType),
    LCTN(LCTN::Location),
    LENS(LENS::LensFlare),
    LGTM(LGTM::LightingTemplate),
    LIGH(LIGH::Light),
    LSCR(LSCR::LoadingScreen),
    LTEX(LTEX::LandscapeTexture),
    LVLI(LVLI::LeveledItem),
    LVLN(LVLN::LeveledNPC),
    MATO(MATO::MaterialObject),
    MATT(MATT::MaterialType),
    MESG(MESG::Message),
    MGEF(MGEF::MagicEffect),
    MISC(MISC::MiscItem),
    MOVT(MOVT::MovementType),
    MSTT(MSTT::MoveableStatic),
    MSWP(MSWP::MaterialSwap),
    MUSC(MUSC::MusicType),
    MUST(MUST::MusicTrack),
    NAVI(NAVI::NavigationMeshInfoMap),
    NAVM(NAVM::NavigationMesh),
    NOCM(NOCM::NavObstacleManager),
    NOTE(NOTE::Note),
    NPC_(NPC_::NonPlayerCharacter),
    OMOD(OMOD::ObjectModification),
    OTFT(OTFT::Outfit),
    OVIS(OVIS::ObjectVisibility),
    PACK(PACK::Package),
    PERK(PERK::Perk),
    PKIN(PKIN::PackIn),
    PGRE(PGRE::PlacedGrenade),
    PHZD(PHZD::PlayerHazard),
    PMIS(PMIS::PlacedMissle),
    PROJ(PROJ::Projectile),
    QUST(QUST::Quest),
    RACE(RACE::Race),
    REFR(REFR::RecordReference),
    REGN(REGN::Region),
    RELA(RELA::Relationship),
    REVB(REVB::Reverb),
    RFCT(RFCT::VisualEffect),
    RFGP(RFGP::ReferenceGroup),
    SCCO(SCCO::SceneCollection),
    SCOL(SCOL::StaticCollection),
    SCSN(SCSN::AudioCategorySnapshot),
    SMBN(SMBN::StoryManagerBranchNode),
    SMEN(SMEN::StoryManagerEventNode),
    SMQN(SMQN::StoryManagerQuestNode),
    SNCT(SNCT::SoundCategory),
    SNDR(SNDR::SoundDescriptor),
    SOPM(SOPM::SoundOutputModel),
    SOUN(SOUN::SoundMarker),
    SPEL(SPEL::Spell),
    SPGD(SPGD::ShaderParticleGeometry),
    STAG(STAG::SoundTag),
    STAT(STAT::Static),
    TACT(TACT::TalkingActivator),
    TERM(TERM::Terminal),
    TES4(TES4::FileHeader),
    TREE(TREE::Tree),
    TRNS(TRNS::Transform),
    TXST(TXST::TextureSet),
    VTYP(VTYP::VoiceType),
    WATR(WATR::Water),
    WEAP(WEAP::Weapon),
    WRLD(WRLD::Worldspace),
    WTHR(WTHR::Weather),
    ZOOM(ZOOM::Zoom),
}

// ===================================================================================================

impl nom_derive::Parse<&[u8]> for ESRecordTyped {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;
        match &header.iden.0 {
            _ => {
                Ok((i, ESRecordTyped::Unhandled(header)))
            }
        }
    }
}

// ====================================================================================================

impl ParseAllocated<ESRecordHeader, &[u8]> for ESRecordTyped {
    fn parse_allocated(header: ESRecordHeader, raw: &[u8]) -> Result<Self, nom::error::Error<&[u8]>> {
        match &header.iden.0 {
            _ => { Ok(ESRecordTyped::Unhandled(header)) }
        }
    }
}

// ===================================================================================================

impl ESObjectTrait for ESRecordTyped {
    fn object_count(&self) -> &usize {
        &1usize
    }

    fn object_size(&self) -> &u32 {
        match self {
            ESRecordTyped::Unhandled(header) => {
                &header.size
            }
            _ => {
                todo!("Cannot get type on unsupported record types yet")
            }
        }
    }
}

// ===================================================================================================

/// Size NOT INCLUDING header, unlike [ESGroupHeader]
#[derive(Debug, Eq, PartialEq, NomLE)]
#[cfg_attr(feature = "speedy", derive(Readable, Writable))]
pub struct ESRecordHeader {
    pub iden: FourCC,
    pub size: u32,
    pub flags: ESRecordFlags,
    pub form_id: FormId,
    pub version_control: ESVersionControl
}


#[derive(Debug, NomLE, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "speedy", derive(Readable, Writable))]
pub struct ESVersionControl {
    pub timestamp: ESTimestamp,
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
#[cfg_attr(feature = "speedy", derive(Readable, Writable))]
pub struct ESTimestamp(pub u16);

// ====================================================================================================

impl ESTimestamp {
    
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

impl ESTimestamp {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

// ====================================================================================================

impl std::fmt::Debug for ESTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}/{:02}/{:02}", self.year(), self.month(), self.day())
    }
}

// ====================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NomLE)]
#[cfg_attr(feature = "speedy", derive(Readable, Writable))]
pub struct ESRecordFlags(pub u32);

// ====================================================================================================

impl ESRecordFlags {
    // TODO: Clippy is saying this will never equal 1, not familiar enough with bitmasks to know why
    // Im guessing it works because the compressed records dont use other flags currently
    // Upon further review, this appears correct and I am unsure why this is happening
    pub fn compressed(&self) -> bool { self.0.bitand(0x00040000) == 1 }
}


// ====================================================================================================

// Bitflags has problems with derives, just keep the basic compressed flag for now
// Each record has its own flags anyway

// bitflags! {
//     /// Represents a set of flags.
//     #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
//     pub struct ESRecordFlags: u32 {
//         /// The value `A`, at bit position `0`.
        
//         /// The data is compressed.
//         const COMPRESSED = 0x00040000;

//         const TES4_MASTER = 0x1;
//         const UNKNOWN_FLAG_2 = 0x2;
//         const UNKNOWN_FLAG_4 = 0x4;
//         const DELETED_GROUP = 0x10;
//         const DELETED_RECORD = 0x20;
//         const GLOB_CONSTANT = 0x40;
//         const REFR_HIDDEN = 0x40;
//         const TES4_LOCALIZED = 0x80;
//         const MUST_UPDATE_ANIMS = 0x100;
//         const REFR_INACCESSIBLE = 0x100;
//         const TES4_LIGHT_MASTER = 0x200;
//         const REFR_HIDDEN2 = 0x200;
//         const ACHR_STARTS_DEAD = 0x200;
//         const REFR_MOTION_BLUR_CASTS_SHADOWS = 0x200;
//         const QUEST_ITEM = 0x400;
//         const PERSISTENT_REFERENCE = 0x400;
//         const LSCR_DISPLAYS_IN_MAIN_MENU = 0x400;
//         const INITIALLY_DISABLED = 0x800;
//         const IGNORED = 0x1000;
//         const UNKNOWN_FLAG_2000 = 0x2000;
//         const VISIBLE_WHEN_DISTANT = 0x8000;
//         const ACTI_RANDOM_ANIMATION_START = 0x10000;
//         const ACTI_DANGEROUS = 0x20000;
//         const OFF_LIMITS = 0x20000;
//         const CANT_WAIT = 0x80000;
//         const ACTI_IGNORE_OBJECT_INTERACTION = 0x100000;
//         const IS_MARKER = 0x800000;
//         const ACTI_OBSTACLE = 0x2000000;
//         const REFR_NO_AI_ACQUIRE = 0x2000000;
//         const NAVMESH_GEN_FILTER = 0x4000000;
//         const NAVMESH_GEN_BOUNDING_BOX = 0x8000000;
//         const FURN_MUST_EXIT_TO_TALK = 0x10000000;
//         const REFR_REFLECTED_BY_AUTO_WATER = 0x10000000;
//         const FURN_CHILD_CAN_USE = 0x20000000;
//         const IDLM_CHILD_CAN_USE = 0x20000000;
//         const REFR_DONT_HAVOK_SETTLE = 0x20000000;
//         const NAVMESH_GEN_GROUND = 0x40000000;
//         const REFR_NORESPAWN = 0x40000000;
//         const REFR_MULTIBOUND = 0x80000000;
//     }
// }

// ====================================================================================================

pub fn alloc_record(i: &[u8]) -> IResult<&[u8], (ESRecordHeader, &[u8]), nom::error::Error<&[u8]>> {
    // Keep original pointer
    let orig = i;

    // Parse header
    let (i, header) = ESRecordHeader::parse(i)?;

    // Take size, not including header size
    let (i, raw) = take(header.size)(i)?;

    #[cfg(debug_assertions)]
    // Check if header is actually a group, which is an unrecoverable error
    if &header.iden.0 == b"GRUP" {
        let (_, gheader) = ESGroupHeader::parse(orig)?;
        panic!("alloc_record(): function encountered a group: {:?}", gheader);
    }
    // Return the values
    else {
        Ok((i, (header, raw)))
    }

    #[cfg(not(debug_assertions))]
    Ok((i, (header, raw)))
}

// ====================================================================================================

#[derive(Debug)]
pub struct ESRecord<T> {
    pub header: ESRecordHeader,
    pub data: Vec<T>
}

// ====================================================================================================

impl<'a, T: Parse<&'a[u8]>> ParseAllocated<ESRecordHeader, &'a[u8]> for ESRecord<T> {
    fn parse_allocated(header: ESRecordHeader, raw: &'a[u8]) -> Result<Self, error::Error<&'a[u8]>> {
        if let Ok((_, data)) = many0(T::parse)(raw) {
            Ok(ESRecord { header, data })
        } else {
            Err(error::Error::new(raw, error::ErrorKind::Fail))
        }
    }
}

impl<'a, T: Parse<&'a[u8]>> ParseAllocated2<ESRecordHeader, &'a[u8]> for ESRecord<T> {
    fn parse_allocated2(header: ESRecordHeader, raw: &'a[u8]) -> IResult<&'a[u8], Self> {
        let (_, data) = many0(T::parse)(raw)?;
        Ok((&[], ESRecord { header, data }))
    }
}

// ====================================================================================================


impl<'a, T> Parse<&'a[u8]> for ESRecord<T> where T: Parse<&'a[u8]> {
    fn parse(i: &'a[u8]) -> IResult<&'a[u8], Self, error::Error<&'a[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;
        let (_, result) = ESRecord::parse_allocated2(header, raw)?;
        Ok((i, result))
    }
}


// ====================================================================================================

/// Common functions for all records
pub trait ESRecordTrait {
    fn record_iden(&self) -> &FourCC;
    fn record_form_id(&self) -> &FormId;
    fn record_size(&self) -> &u32;
}


// ====================================================================================================

/// Implement [ESObject] for anything that implements [ESRecord]
impl ESObjectTrait for dyn ESRecordTrait {
    fn object_count(&self) -> &usize { &1usize }
    fn object_size(&self) -> &u32 { self.record_size() }
}

impl<T> ESRecordTrait for ESRecord<T> {
    fn record_iden(&self) -> &FourCC {
        &self.header.iden
    }

    fn record_form_id(&self) -> &FormId {
        &self.header.form_id
    }

    fn record_size(&self) -> &u32 {
        &self.header.size
    }
}

impl<'a, T> ESRecord<T> where T: Parse<&'a[u8]> + 'static {
    pub fn parse_as_object(i: &'a[u8]) -> IResult<&'a[u8], Box<dyn ESObjectTrait>> {
        let (i, (header, raw)) = alloc_record(i)?;
        let (_, result) = Self::parse_allocated(header, raw)?;
        Ok((i, Box::new(result)))
    }

    pub fn parse_allocated(header: ESRecordHeader, raw: &'a[u8]) -> IResult<&'a[u8], Self> {
        let (_, data) = many0(T::parse)(raw)?;
        Ok((&[], ESRecord { header, data }))
    }
}

impl<T> ESObjectTrait for ESRecord<T> {
    fn object_count(&self) -> &usize { &1usize }
    fn object_size(&self) -> &u32 { &self.header.size }
}