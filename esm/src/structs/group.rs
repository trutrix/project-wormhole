use std::fmt::Debug;

use crate::records::all::*;
use crate::dev::*;
use crate::structs::cell::CellEntry;
use crate::structs::world::WorldEntry;
use super::record::VersionControl;


// ====================================================================================================


#[derive(Debug)]
pub struct GroupHeader {
    pub iden: FourCC, // Always 'GRUP'
    pub size: u32, // Size INCLUDING header, unlike RecordHeader,
    pub label: GroupLabel, // 8 bytes, reversed process
    pub version_control: VersionControl // TODO: Unsure if records and groups share the same version information
}

impl Parse<&[u8]> for GroupHeader {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, iden) = FourCC::parse(i)?;

        #[cfg(debug_assertions)]
        if &iden.0 != b"GRUP" {
             panic!("Invalid group header: {:?}", iden);
        }

        let (i, size) = le_u32(i)?;
        let (i, label) = GroupLabel::parse(i)?;
        let (i, version_control) = VersionControl::parse(i)?;

        Ok((i, GroupHeader { iden, size, label, version_control }))
    }
}


// ====================================================================================================


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupLabel {
    Top(FourCC),
    WorldChildren(FormId),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock([i16;2]),
    ExteriorCellSubBlock([i16;2]),
    CellChildren(FormId),
    TopicChildren(u32),
    CellPersistentChildren(u32),
    CellTemporaryChildren(u32),
    CellVisibleDistantChildren(u32),
    Unknown(FourCC)
}

impl Parse<&[u8]> for GroupLabel {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, data) = FourCC::parse(i)?;
        let (i, label_type) = le_u32(i)?;

        match label_type {
            0 => { Ok((i, GroupLabel::Top(data))) }
            1 => { Ok((i, GroupLabel::WorldChildren(FormId(u32::from_le_bytes(data.0))))) }
            2 => { Ok((i, GroupLabel::InteriorCellBlock(i32::from_le_bytes(data.0)))) }
            3 => { Ok((i, GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(data.0)))) }
            4 => { Ok((i, GroupLabel::ExteriorCellBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
            5 => { Ok((i, GroupLabel::ExteriorCellSubBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
            6 => { Ok((i, GroupLabel::CellChildren(FormId(u32::from_le_bytes(data.0))))) }
            7 => { Ok((i, GroupLabel::TopicChildren(u32::from_le_bytes(data.0)))) }
            8 => { Ok((i, GroupLabel::CellPersistentChildren(u32::from_le_bytes(data.0)))) }
            9 => { Ok((i, GroupLabel::CellTemporaryChildren(u32::from_le_bytes(data.0)))) }
            10 => { Ok((i, GroupLabel::CellVisibleDistantChildren(u32::from_le_bytes(data.0)))) }
            _ => { Ok((i, GroupLabel::Unknown(data))) }
        }
    }
}


// ====================================================================================================


pub fn alloc_group(i: &[u8]) -> IResult<&[u8], (GroupHeader, &[u8])> {
    let (i, header) = GroupHeader::parse(i)?;
    let (i, raw) = take(header.size as usize - 24)(i)?;
    Ok((i, (header, raw)))
}


// ====================================================================================================


pub struct RawDataGroup<'esm> {
    pub header: GroupHeader,
    pub data: Vec<RawRecord<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawDataGroup<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, records) = many0(RawRecord::parse)(data)?;
        Ok((i, RawDataGroup { header, data: records }))
    }
}

impl Debug for RawDataGroup<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawGroup {{ header: {:?}, data: [{} bytes] }}", self.header, self.data.len())
    }
}


// ====================================================================================================


#[derive(Debug)]
pub struct Group<T> {
    pub header: GroupHeader,
    pub data: Vec<T>
}

// impl<T> Parse<&[u8]> for Group<T> where T: for<'nom> Parse<&'nom[u8]> {
//     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
//         let (i, (header, data)) = alloc_group(i)?;
//         let (_, records) = many0(T::parse)(data)?;
//         Ok((i, Group { header, data: records }))
//     }
// }

impl<'esm, T> Parse<&'esm[u8]> for Group<T> where T: for<'nom> Parse<&'esm[u8]> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, records) = many0(T::parse)(data)?;
        Ok((i, Group { header, data: records }))
    }
}



// ====================================================================================================

pub struct RawWorldChildren<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawWorldChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}


impl Debug for RawWorldChildren<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawWorldChildren {{ header: {:?}, data: [{} bytes] }}", self.header, self.data.len())
    }
}

// ====================================================================================================


pub struct RawInteriorCellBlock<'esm> {
    pub header: GroupHeader,
    pub sub_blocks: Vec<RawInteriorCellSubBlock<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawInteriorCellBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, sub_blocks) = many0(RawInteriorCellSubBlock::parse)(data)?;
        Ok((i, Self { header, sub_blocks}))
    }
}

impl std::fmt::Debug for RawInteriorCellBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} bytes", self.header, self.sub_blocks.len())
    }
}

// ====================================================================================================

pub struct RawInteriorCellSubBlock<'esm> {
    pub header: GroupHeader,
    pub data: Vec<RawCellRecord<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawInteriorCellSubBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        // println!("  Parsing InteriorCellSubBlock: {:?}, {} bytes", header, data.len());
        let (_, records) = many0(RawCellRecord::parse)(data)?;
        // println!("  Finished parsing InteriorCellSubBlock: {:?}, {} records", header, records.len());
        Ok((i, Self { header, data: records }))
    }
}

impl std::fmt::Display for RawInteriorCellSubBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} records", self.header, self.data.len())
    }
}

impl std::fmt::Debug for RawInteriorCellSubBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} records", self.header, self.data.len())
    }
}



// ====================================================================================================

pub type CellGroup = Group<InteriorCellBlock>;

#[derive(Debug, NomLE)]
pub struct InteriorCellBlock(pub Group<InteriorCellSubBlock>);

#[derive(Debug, NomLE)]
pub struct InteriorCellSubBlock(pub Group<CellEntry>);


// ====================================================================================================


#[derive(Debug)]
pub struct RawExteriorCellBlock<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawExteriorCellBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct RawExteriorCellSubBlock<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawExteriorCellSubBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct RawTopicChildren<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawTopicChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawCellPersistantChildren<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawCellPersistantChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}


// ====================================================================================================


#[derive(Debug)]
pub struct RawCellTemporaryChildren<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawCellTemporaryChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct RawCellVisibleDistantChildren<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawCellVisibleDistantChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}


// ====================================================================================================

#[derive(Debug)]
pub struct RawCellGroup<'esm> {
    pub header: GroupHeader,
    pub cells: Vec<RawCellRecord<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawCellGroup<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, cells) = many0(RawCellRecord::parse)(data)?;
        Ok((i, Self { header, cells }))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawWorldGroup<'esm> {
    pub header: GroupHeader,
    pub worlds: Vec<RawWorldRecord<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawWorldGroup<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, worlds) = many0(RawWorldRecord::parse)(data)?;
        Ok((i, Self { header, worlds }))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawQuestGroup<'esm> {
    pub header: GroupHeader,
    pub quests: Vec<RawQuestRecord<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawQuestGroup<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, quests) = many0(RawQuestRecord::parse)(data)?;
        Ok((i, Self { header, quests }))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub enum TopGroup {
    Unhandled(Group<RawRecord<'static>>),
    Empty(Group<RawRecord<'static>>),

    AACT(ActionGroup),
    ACTI(ActivatorGroup),
    ADDN(AddonNodeGroup),
    AECH(AudioEffectChainGroup),
    ALCH(AlchemyGroup),
    AMDL(AimModelGroup),
    AMMO(AmmoGroup),
    ANIO(AnimatedObjectGroup),
    AORU(AttractionRuleGroup),
    ARMA(ArmorAddonGroup),
    ARMO(ArmorGroup),
    ARTO(ArtObjectGroup),
    ASPC(AcousticSpaceGroup),
    ASTP(AssociationTypeGroup),
    AVIF(ActorValueInformationGroup),
    BNDS(BendableSplineGroup),
    BOOK(BookGroup),
    BPTD(BodyPartDataGroup),
    CAMS(CameraShotGroup),
    CELL(CellGroup),
    CLAS(ClassGroup),
    CLFM(ColorGroup),
    CLMT(ClimateGroup),
    CMPO(ComponentGroup),
    COBJ(ConstructibleObjectGroup),
    COLL(CollisionLayerGroup),
    CONT(ContainerGroup),
    CPTH(CameraPathGroup),
    CSTY(CombatStyleGroup),
    DEBR(DebrisGroup),
    DFOB(DefaultObjectGroup),
    DLVW(DialogViewGroup),
    DMGT(DamageTypeGroup),
    DOBJ(DefaultObjectsGroup),
    DOOR(DoorGroup),
    ECZN(EncounterZoneGroup),
    EFSH(EffectShaderGroup),
    ENCH(ObjectEffectGroup),
    EQUP(EquipTypeGroup),
    EXPL(ExplosionGroup),
    FACT(FactionGroup),
    FLOR(FloraGroup),
    FLST(FormIdListGroup),
    FSTP(FootstepGroup),
    FSTS(FootstepSetGroup),
    FURN(FurnitureGroup),
    GMST(GameSettingGroup),
    GDRY(GodrayGroup),
    GLOB(GlobalGroup),
    GRAS(GrassGroup),
    HAZD(HazardGroup),
    HDPT(HeadPartGroup),
    IDLE(IdleAnimationGroup),
    IDLM(IdleMarkerGroup),
    IMAD(ImageSpaceAdapterGroup),
    IMGS(ImageSpaceGroup),
    INGR(IngredientGroup),
    INNR(InstanceNamingRulesGroup),
    IPCT(ImpactGroup),
    IPDS(ImpactDataSetGroup),
    KEYM(KeyGroup),
    KYWD(KeywordGroup),
    KSSM(KeywordSoundMappingGroup),
    LAYR(LayerGroup),
    LCRT(LocationReferenceTypeGroup),
    LCTN(LocationGroup),
    LENS(LensFlareGroup),
    LGTM(LightingTemplateGroup),
    LIGH(LightGroup),
    LSCR(LoadingScreenGroup),
    LTEX(LandscapeTextureGroup),
    LVLI(LeveledItemGroup),
    LVLN(LeveledNPCGroup),
    MATO(MaterialObjectGroup),
    MATT(MaterialTypeGroup),
    MESG(MessageGroup),
    MGEF(MagicEffectGroup),
    MISC(MiscItemGroup),
    MOVT(MovementTypeGroup),
    MSTT(MoveableStaticGroup),
    MSWP(MaterialSwapGroup),
    MUSC(MusicTypeGroup),
    MUST(MusicTrackGroup),
    NAVI(NavigationMeshInfoMapGroup),
    NOCM(NavObstacleManagerGroup),
    NOTE(NoteGroup),
    NPC_(NonPlayerCharacterGroup),
    OMOD(ObjectModificationGroup),
    OTFT(OutfitGroup),
    OVIS(ObjectVisibilityGroup),
    PACK(PackageGroup),
    PERK(PerkGroup),
    PKIN(PackInGroup),
    PROJ(ProjectileGroup),
    QUST(QuestGroup),
    RACE(RaceGroup),
    REGN(RegionGroup),
    RELA(RelationshipGroup),
    REVB(ReverbGroup),
    RFCT(VisualEffectGroup),
    RFGP(ReferenceGroupGroup),
    SCCO(SceneCollectionGroup),
    SCOL(StaticCollectionGroup),
    SCSN(AudioCategorySnapshotGroup),
    SMBN(StoryManagerBranchNodeGroup),
    SMEN(StoryManagerEventNodeGroup),
    SMQN(StoryManagerQuestNodeGroup),
    SNCT(SoundCategoryGroup),
    SNDR(SoundDescriptorGroup),
    SOPM(SoundOutputModelGroup),
    SOUN(SoundMarkerGroup),
    SPEL(SpellGroup),
    SPGD(ShaderParticleGeometryGroup),
    STAG(SoundTagGroup),
    STAT(StaticGroup),
    TACT(TalkingActivatorGroup),
    TERM(TerminalGroup),
    TREE(TreeGroup),
    TRNS(TransformGroup),
    TXST(TextureSetGroup),
    VTYP(VoiceTypeGroup),
    WATR(WaterGroup),
    WEAP(WeaponGroup),
    WRLD(Group<WorldEntry>),
    WTHR(WeatherGroup),
    ZOOM(ZoomGroup),
}

impl Parse<&[u8]> for TopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let orig = i;
        let (i, (header, _)) = alloc_group(i)?;

        // println!("Parsing TopGroup: {:?}", header.label);

        if header.size == 0 {
            return Ok((i, TopGroup::Empty(Group { header, data: Vec::new() })));
        }

        
        match header.label {
            GroupLabel::Top(label) => {
                match &label.0 {
                    b"AACT" => { Ok((i, TopGroup::AACT(Group::parse(orig)?.1))) }
                    b"ACTI" => { Ok((i, TopGroup::ACTI(Group::parse(orig)?.1))) }
                    b"ADDN" => { Ok((i, TopGroup::ADDN(Group::parse(orig)?.1))) }
                    b"AECH" => { Ok((i, TopGroup::AECH(Group::parse(orig)?.1))) }
                    b"ALCH" => { Ok((i, TopGroup::ALCH(Group::parse(orig)?.1))) }
                    b"AMDL" => { Ok((i, TopGroup::AMDL(Group::parse(orig)?.1))) }
                    b"AMMO" => { Ok((i, TopGroup::AMMO(Group::parse(orig)?.1))) }
                    b"ANIO" => { Ok((i, TopGroup::ANIO(Group::parse(orig)?.1))) }
                    b"AORU" => { Ok((i, TopGroup::AORU(Group::parse(orig)?.1))) }
                    b"ARMA" => { Ok((i, TopGroup::ARMA(Group::parse(orig)?.1))) }
                    b"ARMO" => { Ok((i, TopGroup::ARMO(Group::parse(orig)?.1))) }
                    b"ARTO" => { Ok((i, TopGroup::ARTO(Group::parse(orig)?.1))) }
                    b"ASPC" => { Ok((i, TopGroup::ASPC(Group::parse(orig)?.1))) }
                    b"ASTP" => { Ok((i, TopGroup::ASTP(Group::parse(orig)?.1))) }
                    b"AVIF" => { Ok((i, TopGroup::AVIF(Group::parse(orig)?.1))) }
                    b"BOOK" => { Ok((i, TopGroup::BOOK(Group::parse(orig)?.1))) }
                    b"BPTD" => { Ok((i, TopGroup::BPTD(Group::parse(orig)?.1))) }
                    b"BNDS" => { Ok((i, TopGroup::BNDS(Group::parse(orig)?.1))) }
                    b"CAMS" => { Ok((i, TopGroup::CAMS(Group::parse(orig)?.1))) }
                    b"CELL" => { Ok((i, TopGroup::CELL(Group::parse(orig)?.1))) }
                    b"CLAS" => { Ok((i, TopGroup::CLAS(Group::parse(orig)?.1))) }
                    b"CLFM" => { Ok((i, TopGroup::CLFM(Group::parse(orig)?.1))) }
                    b"CLMT" => { Ok((i, TopGroup::CLMT(Group::parse(orig)?.1))) }
                    b"CMPO" => { Ok((i, TopGroup::CMPO(Group::parse(orig)?.1))) }
                    b"COBJ" => { Ok((i, TopGroup::COBJ(Group::parse(orig)?.1))) }
                    b"COLL" => { Ok((i, TopGroup::COLL(Group::parse(orig)?.1))) }
                    b"CONT" => { Ok((i, TopGroup::CONT(Group::parse(orig)?.1))) }
                    b"CPTH" => { Ok((i, TopGroup::CPTH(Group::parse(orig)?.1))) }
                    b"CSTY" => { Ok((i, TopGroup::CSTY(Group::parse(orig)?.1))) }
                    b"DEBR" => { Ok((i, TopGroup::DEBR(Group::parse(orig)?.1))) }   
                    b"DFOB" => { Ok((i, TopGroup::DFOB(Group::parse(orig)?.1))) }
                    b"DLVW" => { Ok((i, TopGroup::DLVW(Group::parse(orig)?.1))) }
                    b"DMGT" => { Ok((i, TopGroup::DMGT(Group::parse(orig)?.1))) }
                    b"DOBJ" => { Ok((i, TopGroup::DOBJ(Group::parse(orig)?.1))) }
                    b"DOOR" => { Ok((i, TopGroup::DOOR(Group::parse(orig)?.1))) }
                    b"ECZN" => { Ok((i, TopGroup::ECZN(Group::parse(orig)?.1))) }
                    b"EFSH" => { Ok((i, TopGroup::EFSH(Group::parse(orig)?.1))) }
                    b"ENCH" => { Ok((i, TopGroup::ENCH(Group::parse(orig)?.1))) }
                    b"EQUP" => { Ok((i, TopGroup::EQUP(Group::parse(orig)?.1))) }
                    b"EXPL" => { Ok((i, TopGroup::EXPL(Group::parse(orig)?.1))) }
                    b"FACT" => { Ok((i, TopGroup::FACT(Group::parse(orig)?.1))) }
                    b"FLOR" => { Ok((i, TopGroup::FLOR(Group::parse(orig)?.1))) }
                    b"FLST" => { Ok((i, TopGroup::FLST(Group::parse(orig)?.1))) }
                    b"FSTP" => { Ok((i, TopGroup::FSTP(Group::parse(orig)?.1))) }
                    b"FSTS" => { Ok((i, TopGroup::FSTS(Group::parse(orig)?.1))) }
                    b"FURN" => { Ok((i, TopGroup::FURN(Group::parse(orig)?.1))) }
                    b"GMST" => { Ok((i, TopGroup::GMST(Group::parse(orig)?.1))) }
                    b"GDRY" => { Ok((i, TopGroup::GDRY(Group::parse(orig)?.1))) }
                    b"GLOB" => { Ok((i, TopGroup::GLOB(Group::parse(orig)?.1))) }
                    b"GRAS" => { Ok((i, TopGroup::GRAS(Group::parse(orig)?.1))) }
                    b"HAZD" => { Ok((i, TopGroup::HAZD(Group::parse(orig)?.1))) }
                    b"HDPT" => { Ok((i, TopGroup::HDPT(Group::parse(orig)?.1))) }
                    b"IDLE" => { Ok((i, TopGroup::IDLE(Group::parse(orig)?.1))) }
                    b"IDLM" => { Ok((i, TopGroup::IDLM(Group::parse(orig)?.1))) }
                    b"IMAD" => { Ok((i, TopGroup::IMAD(Group::parse(orig)?.1))) }
                    b"IMGS" => { Ok((i, TopGroup::IMGS(Group::parse(orig)?.1))) }
                    b"INGR" => { Ok((i, TopGroup::INGR(Group::parse(orig)?.1))) }
                    b"INNR" => { Ok((i, TopGroup::INNR(Group::parse(orig)?.1))) }
                    b"IPCT" => { Ok((i, TopGroup::IPCT(Group::parse(orig)?.1))) }
                    b"IPDS" => { Ok((i, TopGroup::IPDS(Group::parse(orig)?.1))) }
                    b"KEYM" => { Ok((i, TopGroup::KEYM(Group::parse(orig)?.1))) }
                    b"KYWD" => { Ok((i, TopGroup::KYWD(Group::parse(orig)?.1))) }
                    b"KSSM" => { Ok((i, TopGroup::KSSM(Group::parse(orig)?.1))) }
                    b"LAYR" => { Ok((i, TopGroup::LAYR(Group::parse(orig)?.1))) }
                    b"LCRT" => { Ok((i, TopGroup::LCRT(Group::parse(orig)?.1))) }
                    b"LCTN" => { Ok((i, TopGroup::LCTN(Group::parse(orig)?.1))) }
                    b"LENS" => { Ok((i, TopGroup::LENS(Group::parse(orig)?.1))) }
                    b"LGTM" => { Ok((i, TopGroup::LGTM(Group::parse(orig)?.1))) }
                    b"LIGH" => { Ok((i, TopGroup::LIGH(Group::parse(orig)?.1))) }
                    b"LSCR" => { Ok((i, TopGroup::LSCR(Group::parse(orig)?.1))) }
                    b"LTEX" => { Ok((i, TopGroup::LTEX(Group::parse(orig)?.1))) }
                    b"LVLI" => { Ok((i, TopGroup::LVLI(Group::parse(orig)?.1))) }
                    b"LVLN" => { Ok((i, TopGroup::LVLN(Group::parse(orig)?.1))) }
                    b"MATO" => { Ok((i, TopGroup::MATO(Group::parse(orig)?.1))) }
                    b"MATT" => { Ok((i, TopGroup::MATT(Group::parse(orig)?.1))) }
                    b"MESG" => { Ok((i, TopGroup::MESG(Group::parse(orig)?.1))) }
                    b"MGEF" => { Ok((i, TopGroup::MGEF(Group::parse(orig)?.1))) }
                    b"MISC" => { Ok((i, TopGroup::MISC(Group::parse(orig)?.1))) }
                    b"MOVT" => { Ok((i, TopGroup::MOVT(Group::parse(orig)?.1))) }
                    b"MSTT" => { Ok((i, TopGroup::MSTT(Group::parse(orig)?.1))) }
                    b"MSWP" => { Ok((i, TopGroup::MSWP(Group::parse(orig)?.1))) }
                    b"MUSC" => { Ok((i, TopGroup::MUSC(Group::parse(orig)?.1))) }
                    b"MUST" => { Ok((i, TopGroup::MUST(Group::parse(orig)?.1))) }
                    b"NAVI" => { 
                        Ok((i, TopGroup::NAVI(Group { header, data: Vec::new()}))) 
                    }
                    b"NOCM" => { Ok((i, TopGroup::NOCM(Group::parse(orig)?.1))) }
                    b"NOTE" => { Ok((i, TopGroup::NOTE(Group::parse(orig)?.1))) }
                    b"NPC_" => { Ok((i, TopGroup::NPC_(Group::parse(orig)?.1))) }
                    b"OMOD" => { Ok((i, TopGroup::OMOD(Group::parse(orig)?.1))) }
                    b"OTFT" => { Ok((i, TopGroup::OTFT(Group::parse(orig)?.1))) }
                    b"OVIS" => { Ok((i, TopGroup::OVIS(Group::parse(orig)?.1))) }
                    b"PACK" => { Ok((i, TopGroup::PACK(Group::parse(orig)?.1))) }
                    b"PERK" => { Ok((i, TopGroup::PERK(Group::parse(orig)?.1))) }
                    b"PKIN" => { Ok((i, TopGroup::PKIN(Group::parse(orig)?.1))) }
                    b"PROJ" => { Ok((i, TopGroup::PROJ(Group::parse(orig)?.1))) }
                    b"QUST" => {
                        //let (_, (header, raw)) = alloc_group(i)?;
                        Ok((i, TopGroup::QUST(Group { header, data: Vec::new()})))
                    }
                    b"RACE" => { Ok((i, TopGroup::RACE(Group::parse(orig)?.1))) }
                    b"REGN" => { Ok((i, TopGroup::REGN(Group::parse(orig)?.1))) }
                    b"RELA" => { Ok((i, TopGroup::RELA(Group::parse(orig)?.1))) }
                    b"REVB" => { Ok((i, TopGroup::REVB(Group::parse(orig)?.1))) }
                    b"RFCT" => { Ok((i, TopGroup::RFCT(Group::parse(orig)?.1))) }
                    b"RFGP" => { Ok((i, TopGroup::RFGP(Group::parse(orig)?.1))) }
                    b"SCCO" => { Ok((i, TopGroup::SCCO(Group::parse(orig)?.1))) }
                    b"SCOL" => { Ok((i, TopGroup::SCOL(Group::parse(orig)?.1))) }
                    b"SCSN" => { Ok((i, TopGroup::SCSN(Group::parse(orig)?.1))) }
                    b"SMBN" => { Ok((i, TopGroup::SMBN(Group::parse(orig)?.1))) }
                    b"SMEN" => { Ok((i, TopGroup::SMEN(Group::parse(orig)?.1))) }
                    b"SMQN" => { Ok((i, TopGroup::SMQN(Group::parse(orig)?.1))) }
                    b"SNCT" => { Ok((i, TopGroup::SNCT(Group::parse(orig)?.1))) }
                    b"SNDR" => { Ok((i, TopGroup::SNDR(Group::parse(orig)?.1))) }
                    b"SOPM" => { Ok((i, TopGroup::SOPM(Group::parse(orig)?.1))) }
                    b"SOUN" => { Ok((i, TopGroup::SOUN(Group::parse(orig)?.1))) }
                    b"SPEL" => { Ok((i, TopGroup::SPEL(Group::parse(orig)?.1))) }
                    b"SPGD" => { Ok((i, TopGroup::SPGD(Group::parse(orig)?.1))) }
                    b"STAG" => { Ok((i, TopGroup::STAG(Group::parse(orig)?.1))) }
                    b"STAT" => { Ok((i, TopGroup::STAT(Group::parse(orig)?.1))) }
                    b"TACT" => { Ok((i, TopGroup::TACT(Group::parse(orig)?.1))) }
                    b"TERM" => { Ok((i, TopGroup::TERM(Group::parse(orig)?.1))) }
                    b"TREE" => { Ok((i, TopGroup::TREE(Group::parse(orig)?.1))) }
                    b"TRNS" => { Ok((i, TopGroup::TRNS(Group::parse(orig)?.1))) }
                    b"TXST" => { Ok((i, TopGroup::TXST(Group::parse(orig)?.1))) }
                    b"VTYP" => { Ok((i, TopGroup::VTYP(Group::parse(orig)?.1))) }
                    b"WATR" => { Ok((i, TopGroup::WATR(Group::parse(orig)?.1))) }
                    b"WEAP" => { Ok((i, TopGroup::WEAP(Group::parse(orig)?.1))) }
                    b"WRLD" => { Ok((i, TopGroup::WRLD(Group::parse(orig)?.1))) }
                    b"WTHR" => { Ok((i, TopGroup::WTHR(Group::parse(orig)?.1))) }
                    b"ZOOM" => { Ok((i, TopGroup::ZOOM(Group::parse(orig)?.1))) }

                    _ => {

                        #[cfg(debug_assertions)]
                        println!("Top group {} not implemented", label);

                        Ok((i, TopGroup::Unhandled(Group { header, data: Vec::new() })))
                    }
                }
            }
            _ => {
                Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Tag)))
            }
        }



    }
}