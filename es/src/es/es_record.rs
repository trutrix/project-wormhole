use nom_derive::nom::error;

use crate::{dev::*, es::{es_group::ESGroupHeader, es_object::{ESObject}}, records::*, traits::{ParseAllocated, record::FormIdTrait}};
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

impl ESObject for ESRecordTyped {
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

    fn try_get_form_id(&self) -> Option<&FormId> {
        unimplemented!("Need to flesh out records more")
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
    pub fn compressed(&self) -> bool { self.0 & 0x00040000 == 1 }
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

    // Check if header is actually a group, which is an unrecoverable error
    if &header.iden.0 == b"GRUP" {
        let (_, gheader) = ESGroupHeader::parse(orig)?;
        panic!("alloc_record(): function encountered a group: {:?}", gheader);
    }
    // Return the values
    else {
        Ok((i, (header, raw)))
    }
}

// ====================================================================================================

// #[derive(Debug)]
// pub struct ESRecord<T> {
//     pub header: ESRecordHeader,
//     pub data: Vec<T>
// }

// ====================================================================================================

// impl<'a, T: Parse<&'a[u8]>> ParseAllocated<ESRecordHeader, &'a[u8]> for ESRecord<T> {
//     fn parse_allocated(header: ESRecordHeader, raw: &'a[u8]) -> Result<Self, error::Error<&'a[u8]>> {
//         if let Ok((_, data)) = many0(T::parse)(raw) {
//             Ok(ESRecord { header, data })
//         } else {
//             Err(error::Error::new(raw, error::ErrorKind::Fail))
//         }
//     }
// }

// ====================================================================================================

// impl<'es, T> Parse<&'es[u8]> for ESRecord<T> where T: ParseAllocated<ESRecordHeader, &'es[u8]> {
//     fn parse(i: &'es[u8]) -> IResult<&'es[u8], Self, error::Error<&'es[u8]>> {
//         let (i, (header, raw)) = alloc_record(i)?;
//         if let Ok(data) = T::parse_allocated(header, raw) {
//             Ok((i, ESRecord { header, data }))
//         } else {

//         }
//     }
// }

// ====================================================================================================

// impl<T> FormIdTrait for ESRecord<T> {
//     fn get_form_id(&self) -> &FormId {
//         &self.header.form_id
//     }
// }

// ====================================================================================================

pub trait ESRecordTraits {
    fn get_header(&self) -> &ESRecordHeader;
}

// ====================================================================================================

impl ESRecordTraits for ESRecordTyped {
    fn get_header(&self) -> &ESRecordHeader {
        match self {
            // ESRecordTyped::Unhandled(r) => r,
            // ESRecordTyped::AACT(r) => &r.header,
            // ESRecordTyped::ACHR(r) => &r.header,
            // ESRecordTyped::ACTI(r) => &r.header,
            // ESRecordTyped::ADDN(r) => &r.header,
            // ESRecordTyped::AECH(r) => &r.header,
            // ESRecordTyped::ALCH(r) => &r.header,
            // ESRecordTyped::AMDL(r) => &r.header,
            // ESRecordTyped::AMMO(r) => &r.header,
            // ESRecordTyped::ANIO(r) => &r.header,
            // ESRecordTyped::AORU(r) => &r.header,
            // ESRecordTyped::ARMA(r) => &r.header,
            // ESRecordTyped::ARMO(r) => &r.header,
            // ESRecordTyped::ARTO(r) => &r.header,
            // ESRecordTyped::ASPC(r) => &r.header,
            // ESRecordTyped::ASTP(r) => &r.header,
            // ESRecordTyped::AVIF(r) => &r.header,
            // ESRecordTyped::BNDS(r) => &r.header,
            // ESRecordTyped::BOOK(r) => &r.header,
            // ESRecordTyped::BPTD(r) => &r.header,
            // ESRecordTyped::CAMS(r) => &r.header,
            // ESRecordTyped::CELL(r) => &r.record.header,
            // ESRecordTyped::CLAS(r) => &r.header,
            // ESRecordTyped::CLFM(r) => &r.header,
            // ESRecordTyped::CLMT(r) => &r.header,
            // ESRecordTyped::CMPO(r) => &r.header,
            // ESRecordTyped::COBJ(r) => &r.header,
            // ESRecordTyped::COLL(r) => &r.header,
            // ESRecordTyped::CONT(r) => &r.header,
            // ESRecordTyped::CPTH(r) => &r.header,
            // ESRecordTyped::CSTY(r) => &r.header,
            // ESRecordTyped::DEBR(r) => &r.header,
            // ESRecordTyped::DFOB(r) => &r.header,
            // ESRecordTyped::DIAL(r) => &r.record.header,
            // ESRecordTyped::DLBR(r) => &r.header,
            // ESRecordTyped::DLVW(r) => &r.header,
            // ESRecordTyped::DMGT(r) => &r.header,
            // ESRecordTyped::DOBJ(r) => &r.header,
            // ESRecordTyped::DOOR(r) => &r.header,
            // ESRecordTyped::ECZN(r) => &r.header,
            // ESRecordTyped::EFSH(r) => &r.header,
            // ESRecordTyped::ENCH(r) => &r.header,
            // ESRecordTyped::EQUP(r) => &r.header,
            // ESRecordTyped::EXPL(r) => &r.header,
            // ESRecordTyped::FACT(r) => &r.header,
            // ESRecordTyped::FLOR(r) => &r.header,
            // ESRecordTyped::FLST(r) => &r.header,
            // ESRecordTyped::FSTP(r) => &r.header,
            // ESRecordTyped::FSTS(r) => &r.header,
            // ESRecordTyped::FURN(r) => &r.header,
            // ESRecordTyped::GDRY(r) => &r.header,
            // ESRecordTyped::GLOB(r) => &r.header,
            // ESRecordTyped::GMST(r) => &r.header,
            // ESRecordTyped::GRAS(r) => &r.header,
            // ESRecordTyped::HAZD(r) => &r.header,
            // ESRecordTyped::HDPT(r) => &r.header,
            // ESRecordTyped::IDLE(r) => &r.header,
            // ESRecordTyped::IDLM(r) => &r.header,
            // ESRecordTyped::IMAD(r) => &r.header,
            // ESRecordTyped::IMGS(r) => &r.header,
            // ESRecordTyped::INGR(r) => &r.header,
            // ESRecordTyped::INNR(r) => &r.header,
            // ESRecordTyped::IPCT(r) => &r.header,
            // ESRecordTyped::IPDS(r) => &r.header,
            // ESRecordTyped::KEYM(r) => &r.header,
            // ESRecordTyped::KSSM(r) => &r.header,
            // ESRecordTyped::KYWD(r) => &r.header,
            // ESRecordTyped::LAND(r) => &r.header,
            // ESRecordTyped::LAYR(r) => &r.header,
            // ESRecordTyped::LCRT(r) => &r.header,
            // ESRecordTyped::LCTN(r) => &r.header,
            // ESRecordTyped::LENS(r) => &r.header,
            // ESRecordTyped::LGTM(r) => &r.header,
            // ESRecordTyped::LIGH(r) => &r.header,
            // ESRecordTyped::LSCR(r) => &r.header,
            // ESRecordTyped::LTEX(r) => &r.header,
            // ESRecordTyped::LVLI(r) => &r.header,
            // ESRecordTyped::LVLN(r) => &r.header,
            // ESRecordTyped::MATO(r) => &r.header,
            // ESRecordTyped::MATT(r) => &r.header,
            // ESRecordTyped::MESG(r) => &r.header,
            // ESRecordTyped::MGEF(r) => &r.header,
            // ESRecordTyped::MISC(r) => &r.header,
            // ESRecordTyped::MOVT(r) => &r.header,
            // ESRecordTyped::MSTT(r) => &r.header,
            // ESRecordTyped::MSWP(r) => &r.header,
            // ESRecordTyped::MUSC(r) => &r.header,
            // ESRecordTyped::MUST(r) => &r.header,
            // ESRecordTyped::NAVI(r) => &r.header,
            // ESRecordTyped::NAVM(r) => &r.header,
            // ESRecordTyped::NOCM(r) => &r.header,
            // ESRecordTyped::NOTE(r) => &r.header,
            // ESRecordTyped::NPC_(r) => &r.header,
            // ESRecordTyped::OMOD(r) => &r.header,
            // ESRecordTyped::OTFT(r) => &r.header,
            // ESRecordTyped::OVIS(r) => &r.header,
            // ESRecordTyped::PACK(r) => &r.header,
            // ESRecordTyped::PERK(r) => &r.header,
            // ESRecordTyped::PKIN(r) => &r.header,
            // ESRecordTyped::PGRE(r) => &r.header,
            // ESRecordTyped::PHZD(r) => &r.header,
            // ESRecordTyped::PMIS(r) => &r.header,
            // ESRecordTyped::PROJ(r) => &r.header,
            // ESRecordTyped::QUST(r) => &r.record.header,
            // ESRecordTyped::RACE(r) => &r.header,
            // ESRecordTyped::REFR(r) => &r.header,
            // ESRecordTyped::REGN(r) => &r.header,
            // ESRecordTyped::RELA(r) => &r.header,
            // ESRecordTyped::REVB(r) => &r.header,
            // ESRecordTyped::RFCT(r) => &r.header,
            // ESRecordTyped::RFGP(r) => &r.header,
            // ESRecordTyped::SCCO(r) => &r.header,
            // ESRecordTyped::SCOL(r) => &r.header,
            // ESRecordTyped::SCSN(r) => &r.header,
            // ESRecordTyped::SMBN(r) => &r.header,
            // ESRecordTyped::SMEN(r) => &r.header,
            // ESRecordTyped::SMQN(r) => &r.header,
            // ESRecordTyped::SNCT(r) => &r.header,
            // ESRecordTyped::SNDR(r) => &r.header,
            // ESRecordTyped::SOPM(r) => &r.header,
            // ESRecordTyped::SOUN(r) => &r.header,
            // ESRecordTyped::SPEL(r) => &r.header,
            // ESRecordTyped::SPGD(r) => &r.header,
            // ESRecordTyped::STAG(r) => &r.header,
            // ESRecordTyped::STAT(r) => &r.header,
            // ESRecordTyped::TACT(r) => &r.header,
            // ESRecordTyped::TERM(r) => &r.header,
            // ESRecordTyped::TES4(r) => &r.header,
            // ESRecordTyped::TREE(r) => &r.header,
            // ESRecordTyped::TRNS(r) => &r.header,
            // ESRecordTyped::TXST(r) => &r.header,
            // ESRecordTyped::VTYP(r) => &r.header,
            // ESRecordTyped::WATR(r) => &r.header,
            // ESRecordTyped::WEAP(r) => &r.header,
            // ESRecordTyped::WRLD(r) => &r.record.header,
            // ESRecordTyped::WTHR(r) => &r.header,
            // ESRecordTyped::ZOOM(r) => &r.header,
            _ => todo!()
        }
    }
}


pub trait ESRecord {
    fn get_iden(&self) -> &FourCC;
    fn get_form_id(&self) -> &FormId;
    fn get_size(&self) -> &u32;
}


impl<T> ESObject for T where T: ESRecord {
    fn object_count(&self) -> &usize {
        &1usize
    }

    fn object_size(&self) -> &u32 {
        self.get_size()
    }

    fn try_get_form_id(&self) -> Option<&FormId> {
        Some(self.get_form_id())
    }
}