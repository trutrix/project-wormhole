use nom_derive::nom::error;

use crate::{dev::*, es::{es_group::ESGroupHeader, es_object::{ESHeader, ESObjectTraits}}, records::*, traits::{ParseAllocated, record::FormIdTrait}};
//use bitflags::bitflags;

#[derive(Debug)]
pub enum ESRecord {
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

impl nom_derive::Parse<&[u8]> for ESRecord {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;
        match &header.iden.0 {
            b"AACT" => { 
                let (_, value) = Vec::parse(raw)?;
                Ok((i, ESRecord::AACT(ESGenericRecord { header, data: value }) ))
            }

            b"ACHR" => {
                let (_, value) = Vec::parse(raw)?;
                Ok((i, ESRecord::ACHR(ESGenericRecord { header, data: value }) ))
            }

            _ => {
                Ok((i, ESRecord::Unhandled(header)))
            }
        }
    }
}

// ===================================================================================================

#[cfg(feature = "speedy")]
impl<'a, C: speedy::Context> Readable<'a, C> for ESRecord {
    fn read_from< R: speedy::Reader< 'a, C > >( reader: &mut R ) -> Result< Self, <C as speedy::Context>::Error > {
        let header: ESRecordHeader = reader.read_value()?;

        match header.iden.0 {
            _ => {
                panic!("{:?}", header);
            }
        }
    }
}

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

#[derive(Debug, NomLE)]
pub struct ESGenericRecord<T> {
    pub header: ESRecordHeader,
    pub data: T
}

// ====================================================================================================

impl<'a, T: Parse<&'a[u8]>> ParseAllocated<ESRecordHeader, &'a[u8]> for ESGenericRecord<T> {
    fn parse_allocated(header: ESRecordHeader, raw: &'a[u8]) -> Result<Self, error::Error<&'a[u8]>> {
        if let Ok((_, data)) = T::parse(raw) {
            Ok(ESGenericRecord { header, data })
        } else {
            Err(error::Error::new(raw, error::ErrorKind::Fail))
        }
    }
}

// ====================================================================================================

impl<T> FormIdTrait for ESGenericRecord<T> {
    fn get_form_id(&self) -> &FormId {
        &self.header.form_id
    }
}

// ====================================================================================================

impl ESHeader<ESRecordHeader> for ESRecord {
    fn header(&self) -> &ESRecordHeader {
        match self {
            ESRecord::Unhandled(esrecord_header) => esrecord_header,
            ESRecord::AACT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ACHR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ACTI(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ADDN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::AECH(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ALCH(esgeneric_record) => &esgeneric_record.header,
            ESRecord::AMDL(esgeneric_record) => &esgeneric_record.header,
            ESRecord::AMMO(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ANIO(esgeneric_record) => &esgeneric_record.header,
            ESRecord::AORU(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ARMA(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ARMO(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ARTO(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ASPC(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ASTP(esgeneric_record) => &esgeneric_record.header,
            ESRecord::AVIF(esgeneric_record) => &esgeneric_record.header,
            ESRecord::BNDS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::BOOK(esgeneric_record) => &esgeneric_record.header,
            ESRecord::BPTD(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CAMS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CELL(cell) => todo!(),
            ESRecord::CLAS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CLFM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CLMT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CMPO(esgeneric_record) => &esgeneric_record.header,
            ESRecord::COBJ(esgeneric_record) => &esgeneric_record.header,
            ESRecord::COLL(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CONT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CPTH(esgeneric_record) => &esgeneric_record.header,
            ESRecord::CSTY(esgeneric_record) => &esgeneric_record.header,
            ESRecord::DEBR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::DFOB(esgeneric_record) => &esgeneric_record.header,
            ESRecord::DIAL(dialog) => todo!(),
            ESRecord::DLBR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::DLVW(esgeneric_record) => &esgeneric_record.header,
            ESRecord::DMGT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::DOBJ(esgeneric_record) => &esgeneric_record.header,
            ESRecord::DOOR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ECZN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::EFSH(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ENCH(esgeneric_record) => &esgeneric_record.header,
            ESRecord::EQUP(esgeneric_record) => &esgeneric_record.header,
            ESRecord::EXPL(esgeneric_record) => &esgeneric_record.header,
            ESRecord::FACT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::FLOR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::FLST(esgeneric_record) => &esgeneric_record.header,
            ESRecord::FSTP(esgeneric_record) => &esgeneric_record.header,
            ESRecord::FSTS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::FURN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::GDRY(esgeneric_record) => &esgeneric_record.header,
            ESRecord::GLOB(esgeneric_record) => &esgeneric_record.header,
            ESRecord::GMST(esgeneric_record) => &esgeneric_record.header,
            ESRecord::GRAS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::HAZD(esgeneric_record) => &esgeneric_record.header,
            ESRecord::HDPT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::IDLE(esgeneric_record) => &esgeneric_record.header,
            ESRecord::IDLM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::IMAD(esgeneric_record) => &esgeneric_record.header,
            ESRecord::IMGS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::INGR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::INNR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::IPCT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::IPDS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::KEYM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::KSSM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::KYWD(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LAND(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LAYR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LCRT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LCTN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LENS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LGTM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LIGH(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LSCR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LTEX(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LVLI(esgeneric_record) => &esgeneric_record.header,
            ESRecord::LVLN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MATO(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MATT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MESG(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MGEF(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MISC(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MOVT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MSTT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MSWP(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MUSC(esgeneric_record) => &esgeneric_record.header,
            ESRecord::MUST(esgeneric_record) => &esgeneric_record.header,
            ESRecord::NAVI(esgeneric_record) => &esgeneric_record.header,
            ESRecord::NAVM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::NOCM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::NOTE(esgeneric_record) => &esgeneric_record.header,
            ESRecord::NPC_(esgeneric_record) => &esgeneric_record.header,
            ESRecord::OMOD(esgeneric_record) => &esgeneric_record.header,
            ESRecord::OTFT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::OVIS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::PACK(esgeneric_record) => &esgeneric_record.header,
            ESRecord::PERK(esgeneric_record) => &esgeneric_record.header,
            ESRecord::PKIN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::PGRE(esgeneric_record) => &esgeneric_record.header,
            ESRecord::PHZD(esgeneric_record) => &esgeneric_record.header,
            ESRecord::PMIS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::PROJ(esgeneric_record) => &esgeneric_record.header,
            ESRecord::QUST(quest) => todo!(),
            ESRecord::RACE(esgeneric_record) => &esgeneric_record.header,
            ESRecord::REFR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::REGN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::RELA(esgeneric_record) => &esgeneric_record.header,
            ESRecord::REVB(esgeneric_record) => &esgeneric_record.header,
            ESRecord::RFCT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::RFGP(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SCCO(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SCOL(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SCSN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SMBN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SMEN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SMQN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SNCT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SNDR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SOPM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SOUN(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SPEL(esgeneric_record) => &esgeneric_record.header,
            ESRecord::SPGD(esgeneric_record) => &esgeneric_record.header,
            ESRecord::STAG(esgeneric_record) => &esgeneric_record.header,
            ESRecord::STAT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::TACT(esgeneric_record) => &esgeneric_record.header,
            ESRecord::TERM(esgeneric_record) => &esgeneric_record.header,
            ESRecord::TES4(esgeneric_record) => &esgeneric_record.header,
            ESRecord::TREE(esgeneric_record) => &esgeneric_record.header,
            ESRecord::TRNS(esgeneric_record) => &esgeneric_record.header,
            ESRecord::TXST(esgeneric_record) => &esgeneric_record.header,
            ESRecord::VTYP(esgeneric_record) => &esgeneric_record.header,
            ESRecord::WATR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::WEAP(esgeneric_record) => &esgeneric_record.header,
            ESRecord::WRLD(worldspace) => todo!(),
            ESRecord::WTHR(esgeneric_record) => &esgeneric_record.header,
            ESRecord::ZOOM(esgeneric_record) => &esgeneric_record.header,
        }
    }
}