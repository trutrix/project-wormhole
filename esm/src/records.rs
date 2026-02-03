
#![allow(non_snake_case)]

use std::rc::Rc;
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


#[derive(Debug)]
pub enum SingleRecordRef {
    AACT(Rc<AACT::Action>),
    ACHR(Rc<ACHR::ActorReference>),
    ACTI(Rc<ACTI::Activator>),
    ADDN(Rc<ADDN::AddonNode>),
    AECH(Rc<AECH::AudioEffectChain>),
    ALCH(Rc<ALCH::Alchemy>),
    AMDL(Rc<AMDL::AimModel>),
    AMMO(Rc<AMMO::Ammo>),
    ANIO(Rc<ANIO::AnimatedObject>),
    AORU(Rc<AORU::AttractionRule>),
    ARMA(Rc<ARMA::ArmorAddon>),
    ARMO(Rc<ARMO::Armor>),
    ARTO(Rc<ARTO::ArtObject>),
    ASPC(Rc<ASPC::AcousticSpace>),
    ASTP(Rc<ASTP::AssociationType>),
    AVIF(Rc<AVIF::ActorValueInformation>),
    BNDS(Rc<BNDS::BendableSpline>),
    BOOK(Rc<BOOK::Book>),
    BPTD(Rc<BPTD::BodyPartData>),
    CAMS(Rc<CAMS::CameraShot>),
    CELL(Rc<CELL::Cell>),
    CLAS(Rc<CLAS::Class>),
    CLFM(Rc<CLFM::Color>),
    CLMT(Rc<CLMT::Climate>),
    CMPO(Rc<CMPO::Component>),
    COBJ(Rc<COBJ::ConstructibleObject>),
    COLL(Rc<COLL::CollisionLayer>),
    CONT(Rc<CONT::Container>),
    CPTH(Rc<CPTH::CameraPath>),
    CSTY(Rc<CSTY::CombatStyle>),
    DEBR(Rc<DEBR::Debris>),
    DFOB(Rc<DFOB::DefaultObject>),
    DLVW(Rc<DLVW::DialogView>),
    DMGT(Rc<DMGT::DamageType>),
    DOBJ(Rc<DOBJ::DefaultObjects>),
    DOOR(Rc<DOOR::Door>),
    ECZN(Rc<ECZN::EncounterZone>),
    EFSH(Rc<EFSH::EffectShader>),
    ENCH(Rc<ENCH::ObjectEffect>),
    EQUP(Rc<EQUP::EquipType>),
    EXPL(Rc<EXPL::Explosion>),
    FACT(Rc<FACT::Faction>),
    FLOR(Rc<FLOR::Flora>),
    FLST(Rc<FLST::FormIdList>),
    FSTP(Rc<FSTP::Footstep>),
    FSTS(Rc<FSTS::FootstepSet>),
    FURN(Rc<FURN::Furniture>),
    GDRY(Rc<GDRY::Godray>),
    GLOB(Rc<GLOB::Global>),
    GMST(Rc<GMST::GameSetting>),
    GRAS(Rc<GRAS::Grass>),
    HAZD(Rc<HAZD::Hazard>),
    HDPT(Rc<HDPT::HeadPart>),
    IDLE(Rc<IDLE::IdleAnimation>),
    IDLM(Rc<IDLM::IdleMarker>),
    IMAD(Rc<IMAD::ImageSpaceAdapter>),
    IMGS(Rc<IMGS::ImageSpace>),
    INGR(Rc<INGR::Ingredient>),
    INNR(Rc<INNR::InstanceNamingRules>),
    IPCT(Rc<IPCT::Impact>),
    IPDS(Rc<IPDS::ImpactDataSet>),
    KEYM(Rc<KEYM::Key>),
    KSSM(Rc<KSSM::KeywordSoundMapping>),
    KYWD(Rc<KYWD::Keyword>),
    LAND(Rc<LAND::Landscape>),
    LAYR(Rc<LAYR::Layer>),
    LCRT(Rc<LCRT::LocationReferenceType>),
    LCTN(Rc<LCTN::Location>),
    LENS(Rc<LENS::LensFlare>),
    LGTM(Rc<LGTM::LightingTemplate>),
    LIGH(Rc<LIGH::Light>),
    LSCR(Rc<LSCR::LoadingScreen>),
    LTEX(Rc<LTEX::LandscapeTexture>),
    LVLI(Rc<LVLI::LeveledItem>),
    LVLN(Rc<LVLN::LeveledNPC>),
    MATO(Rc<MATO::MaterialObject>),
    MATT(Rc<MATT::MaterialType>),
    MESG(Rc<MESG::Message>),
    MGEF(Rc<MGEF::MagicEffect>),
    MISC(Rc<MISC::MiscItem>),
    MOVT(Rc<MOVT::MovementType>),
    MSTT(Rc<MSTT::MoveableStatic>),
    MSWP(Rc<MSWP::MaterialSwap>),
    MUSC(Rc<MUSC::MusicType>),
    MUST(Rc<MUST::MusicTrack>),
    NAVI(Rc<NAVI::NavigationMeshInfoMap>),
    NAVM(Rc<NAVM::NavigationMesh>),
    NOCM(Rc<NOCM::NavObstacleManager>),
    NOTE(Rc<NOTE::Note>),
    NPC_(Rc<NPC_::NonPlayerCharacter>),
    OMOD(Rc<OMOD::ObjectModification>),
    OTFT(Rc<OTFT::Outfit>),
    OVIS(Rc<OVIS::ObjectVisibility>),
    PACK(Rc<PACK::Package>),
    PERK(Rc<PERK::Perk>),
    PKIN(Rc<PKIN::PackIn>),
    PROJ(Rc<PROJ::Projectile>),
    QUST(Rc<QUST::Quest>),
    RACE(Rc<RACE::Race>),
    REFR(Rc<REFR::Reference>),
    REGN(Rc<REGN::Region>),
    RELA(Rc<RELA::Relationship>),
    REVB(Rc<REVB::Reverb>),
    RFCT(Rc<RFCT::VisualEffect>),
    RFGP(Rc<RFGP::ReferenceGroup>),
    SCCO(Rc<SCCO::SceneCollection>),
    SCOL(Rc<SCOL::StaticCollection>),
    SCSN(Rc<SCSN::AudioCategorySnapshot>),
    SMBN(Rc<SMBN::StoryManagerBranchNode>),
    SMEN(Rc<SMEN::StoryManagerEventNode>),
    SMQN(Rc<SMQN::StoryManagerQuestNode>),
    SNCT(Rc<SNCT::SoundCategory>),
    SNDR(Rc<SNDR::SoundDescriptor>),
    SOPM(Rc<SOPM::SoundOutputModel>),
    SOUN(Rc<SOUN::SoundMarker>),
    SPEL(Rc<SPEL::Spell>),
    SPGD(Rc<SPGD::ShaderParticleGeometry>),
    STAG(Rc<STAG::SoundTag>),
    STAT(Rc<STAT::Static>),
    TACT(Rc<TACT::TalkingActivator>),
    TERM(Rc<TERM::Terminal>),
    TES4(Rc<TES4::FileHeader>),
    TREE(Rc<TREE::Tree>),
    TRNS(Rc<TRNS::Transform>),
    TXST(Rc<TXST::TextureSet>),
    VTYP(Rc<VTYP::VoiceType>),
    WATR(Rc<WATR::Water>),
    WEAP(Rc<WEAP::Weapon>),
    WRLD(Rc<WRLD::Worldspace>),
    WTHR(Rc<WTHR::Weather>),
    ZOOM(Rc<ZOOM::Zoom>),
}