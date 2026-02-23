
#![allow(non_snake_case)]
pub mod AACT;
pub mod ACHR;
pub mod ACTI;
pub mod ADDN;
pub mod AECH;
pub mod ALCH;
pub mod AMDL;
pub mod AMMO;
pub mod ANIO;
pub mod AORU;
pub mod ARMA;
pub mod ARMO;
pub mod ARTO;
pub mod ASPC;
pub mod ASTP;
pub mod AVIF;
pub mod BNDS;
pub mod BOOK;
pub mod BPTD;
pub mod CAMS;
pub mod CELL;
pub mod CLAS;
pub mod CLFM;
pub mod CLMT;
pub mod CMPO;
pub mod COBJ;
pub mod COLL;
pub mod CONT;
pub mod CPTH;
pub mod CSTY;
pub mod DEBR;
pub mod DFOB;
pub mod DLVW;
pub mod DMGT;
pub mod DOBJ;
pub mod DOOR;
pub mod ECZN;
pub mod EFSH;
pub mod ENCH;
pub mod EQUP;
pub mod EXPL;
pub mod FACT;
pub mod FLOR;
pub mod FLST;
pub mod FSTP;
pub mod FSTS;
pub mod FURN;
pub mod GDRY;
pub mod GLOB;
pub mod GMST;
pub mod GRAS;
pub mod HAZD;
pub mod HDPT;
pub mod IDLE;
pub mod IDLM;
pub mod IMAD;
pub mod IMGS;
pub mod INGR;
pub mod INNR;
pub mod IPCT;
pub mod IPDS;
pub mod KEYM;
pub mod KSSM;
pub mod KYWD;
pub mod LAND;
pub mod LAYR;
pub mod LCRT;
pub mod LCTN;
pub mod LENS;
pub mod LGTM;
pub mod LIGH;
pub mod LSCR;
pub mod LTEX;
pub mod LVLI;
pub mod LVLN;
pub mod MATO;
pub mod MATT;
pub mod MESG;
pub mod MGEF;
pub mod MISC;
pub mod MOVT;
pub mod MSTT;
pub mod MSWP;
pub mod MUSC;
pub mod MUST;
pub mod NAVI;
pub mod NAVM;
pub mod NOCM;
pub mod NOTE;
pub mod NPC_;
pub mod OMOD;
pub mod OTFT;
pub mod OVIS;
pub mod PACK;
pub mod PERK;
pub mod PHZD;
pub mod PKIN;
pub mod PROJ;
pub mod QUST;
pub mod RACE;
pub mod REFR;
pub mod REGN;
pub mod RELA;
pub mod REVB;
pub mod RFCT;
pub mod RFGP;
pub mod SCCO;
pub mod SCOL;
pub mod SCSN;
pub mod SMBN;
pub mod SMEN;
pub mod SMQN;
pub mod SNCT;
pub mod SNDR;
pub mod SOPM;
pub mod SOUN;
pub mod SPEL;
pub mod SPGD;
pub mod STAG;
pub mod STAT;
pub mod TACT;
pub mod TERM;
pub mod TES4;
pub mod TREE;
pub mod TRNS;
pub mod TXST;
pub mod VTYP;
pub mod WATR;
pub mod WEAP;
pub mod WRLD;
pub mod WTHR;
pub mod ZOOM;


pub mod all {
    pub use super::AACT::*;
    pub use super::ACHR::*;
    pub use super::ACTI::*;
    pub use super::ADDN::*;
    pub use super::AECH::*;
    pub use super::ALCH::*;
    pub use super::AMDL::*;
    pub use super::AMMO::*;
    pub use super::ANIO::*;
    pub use super::AORU::*;
    pub use super::ARMA::*;
    pub use super::ARMO::*;
    pub use super::ARTO::*;
    pub use super::ASPC::*;
    pub use super::ASTP::*;
    pub use super::AVIF::*;
    pub use super::BNDS::*;
    pub use super::BOOK::*;
    pub use super::BPTD::*;
    pub use super::CAMS::*;
    pub use super::CELL::*;
    pub use super::CLAS::*;
    pub use super::CLFM::*;
    pub use super::CLMT::*;
    pub use super::CMPO::*;
    pub use super::COBJ::*;
    pub use super::COLL::*;
    pub use super::CONT::*;
    pub use super::CPTH::*;
    pub use super::CSTY::*;
    pub use super::DEBR::*;
    pub use super::DFOB::*;
    pub use super::DLVW::*;
    pub use super::DMGT::*;
    pub use super::DOBJ::*;
    pub use super::DOOR::*;
    pub use super::ECZN::*;
    pub use super::EFSH::*;
    pub use super::ENCH::*;
    pub use super::EQUP::*;
    pub use super::EXPL::*;
    pub use super::FACT::*;
    pub use super::FLOR::*;
    pub use super::FLST::*;
    pub use super::FSTP::*;
    pub use super::FSTS::*;
    pub use super::FURN::*;
    pub use super::GDRY::*;
    pub use super::GLOB::*;
    pub use super::GMST::*;
    pub use super::GRAS::*;
    pub use super::HAZD::*;
    pub use super::HDPT::*;
    pub use super::IDLE::*;
    pub use super::IDLM::*;
    pub use super::IMAD::*;
    pub use super::IMGS::*;
    pub use super::INGR::*;
    pub use super::INNR::*;
    pub use super::IPCT::*;
    pub use super::IPDS::*;
    pub use super::KEYM::*;
    pub use super::KSSM::*;
    pub use super::KYWD::*;
    pub use super::LAND::*;
    pub use super::LAYR::*;
    pub use super::LCRT::*;
    pub use super::LCTN::*;
    pub use super::LENS::*;
    pub use super::LGTM::*;
    pub use super::LIGH::*;
    pub use super::LSCR::*;
    pub use super::LTEX::*;
    pub use super::LVLI::*;
    pub use super::LVLN::*;
    pub use super::MATO::*;
    pub use super::MATT::*;
    pub use super::MESG::*;
    pub use super::MGEF::*;
    pub use super::MISC::*;
    pub use super::MOVT::*;
    pub use super::MSTT::*;
    pub use super::MSWP::*;
    pub use super::MUSC::*;
    pub use super::MUST::*;
    pub use super::NAVI::*;
    pub use super::NAVM::*;
    pub use super::NOCM::*;
    pub use super::NOTE::*;
    pub use super::NPC_::*;
    pub use super::OMOD::*;
    pub use super::OTFT::*;
    pub use super::OVIS::*;
    pub use super::PACK::*;
    pub use super::PERK::*;
    pub use super::PHZD::*;
    pub use super::PKIN::*;
    pub use super::PROJ::*;
    pub use super::QUST::*;
    pub use super::RACE::*;
    pub use super::REFR::*;
    pub use super::REGN::*;
    pub use super::RELA::*;
    pub use super::REVB::*;
    pub use super::RFCT::*;
    pub use super::RFGP::*;
    pub use super::SCCO::*;
    pub use super::SCOL::*;
    pub use super::SCSN::*;
    pub use super::SMBN::*;
    pub use super::SMEN::*;
    pub use super::SMQN::*;
    pub use super::SNCT::*;
    pub use super::SNDR::*;
    pub use super::SOPM::*;
    pub use super::SOUN::*;
    pub use super::SPEL::*;
    pub use super::SPGD::*;
    pub use super::STAG::*;
    pub use super::STAT::*;
    pub use super::TACT::*;
    pub use super::TERM::*;
    pub use super::TES4::*;
    pub use super::TREE::*;
    pub use super::TRNS::*;
    pub use super::TXST::*;
    pub use super::VTYP::*;
    pub use super::WATR::*;
    pub use super::WEAP::*;
    pub use super::WRLD::*;
    pub use super::WTHR::*;
    pub use super::ZOOM::*;
}


#[derive(Debug)]
pub enum SingleRecord {
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
    PHZD(PHZD::PlayerHazard),
    PROJ(PROJ::Projectile),
    QUST(QUST::Quest),
    RACE(RACE::Race),
    REFR(REFR::Reference),
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