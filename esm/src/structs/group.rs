use std::fmt::Debug;

use crate::records::all::*;
use crate::dev::*;
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
    WorldChildren(u32),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock([i16;2]),
    ExteriorCellSubBlock([i16;2]),
    CellChildren(u32),
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
            1 => { Ok((i, GroupLabel::WorldChildren(u32::from_le_bytes(data.0)))) }
            2 => { Ok((i, GroupLabel::InteriorCellBlock(i32::from_le_bytes(data.0)))) }
            3 => { Ok((i, GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(data.0))))}
            4 => { Ok((i, GroupLabel::ExteriorCellBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
            5 => { Ok((i, GroupLabel::ExteriorCellSubBlock([i16::from_le_bytes(data.0[0..2].try_into().unwrap()), i16::from_le_bytes(data.0[2..4].try_into().unwrap())]))) }
            6 => { Ok((i, GroupLabel::CellChildren(u32::from_le_bytes(data.0)))) }
            7 => { Ok((i, GroupLabel::TopicChildren(u32::from_le_bytes(data.0))))}
            8 => { Ok((i, GroupLabel::CellPersistentChildren(u32::from_le_bytes(data.0)))) }
            9 => { Ok((i, GroupLabel::CellTemporaryChildren(u32::from_le_bytes(data.0)))) }
            10 => { Ok((i, GroupLabel::CellVisibleDistantChildren(u32::from_le_bytes(data.0))))}
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
pub struct GroupVec<T> {
    pub header: GroupHeader,
    pub data: Vec<T>
}

impl<T: for<'esm> Parse<&'esm[u8]>> Parse<&[u8]> for GroupVec<T> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, records) = many0(T::parse)(data)?;
        Ok((i, GroupVec { header, data: records }))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct Group<T> {
    pub header: GroupHeader,
    pub data: T
}

impl<T: for<'esm> Parse<&'esm[u8]>> Parse<&[u8]> for Group<T> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, data) = T::parse(data)?;
        Ok((i, Group { header, data }))
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
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawInteriorCellBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}

impl std::fmt::Debug for RawInteriorCellBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} bytes", self.header, self.data.len())
    }
}


// ====================================================================================================


#[derive(Debug)]
pub struct RawInteriorCellSubBlock<'esm> {
    pub header: GroupHeader,
    pub data: &'esm [u8]
}

impl<'esm> Parse<&'esm[u8]> for RawInteriorCellSubBlock<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, Self { header, data }))
    }
}


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
    Unhandled(GroupVec<RawRecord<'static>>),
    AACT(GroupVec<Action>),
    ACTI(GroupVec<Activator>),
    ADDN(GroupVec<AddonNode>),
    AECH(GroupVec<AudioEffectChain>),
    ALCH(GroupVec<Alchemy>),
    AMDL(GroupVec<AimModel>),
    AMMO(GroupVec<Ammo>),
    ANIO(GroupVec<AnimatedObject>),
    AORU(GroupVec<AttractionRule>),
    ARMA(GroupVec<ArmorAddon>),
    ARMO(GroupVec<Armor>),
    ARTO(GroupVec<ArtObject>),
    ASPC(GroupVec<AcousticSpace>),
    ASTP(GroupVec<AssociationType>),
    AVIF(GroupVec<ActorValueInformation>),
    BNDS(GroupVec<BendableSpline>),
    BOOK(GroupVec<Book>),
    BPTD(GroupVec<BodyPartData>),
    CAMS(GroupVec<CameraShot>),
    CELL(GroupVec<Cell>),
    CLAS(GroupVec<Class>),
    CLFM(GroupVec<Color>),
    CLMT(GroupVec<Climate>),
    CMPO(GroupVec<Component>),
    COBJ(GroupVec<ConstructibleObject>),
    COLL(GroupVec<CollisionLayer>),
    CONT(GroupVec<Container>),
    CPTH(GroupVec<CameraPath>),
    CSTY(GroupVec<CombatStyle>),
    DEBR(GroupVec<Debris>),
    DFOB(GroupVec<DefaultObject>),
    DLVW(GroupVec<DialogView>),
    DMGT(GroupVec<DamageType>),
    DOBJ(GroupVec<DefaultObjects>),
    DOOR(GroupVec<Door>),
    ECZN(GroupVec<EncounterZone>),
    EFSH(GroupVec<EffectShader>),
    ENCH(GroupVec<ObjectEffect>),
    EQUP(GroupVec<EquipType>),
    EXPL(GroupVec<Explosion>),
    FACT(GroupVec<Faction>),
    FLOR(GroupVec<Flora>),
    FLST(GroupVec<FormIdList>),
    FSTP(GroupVec<Footstep>),
    FSTS(GroupVec<FootstepSet>),
    FURN(GroupVec<Furniture>),
    GMST(GroupVec<GameSetting>),
    GDRY(GroupVec<Godray>),
    GLOB(GroupVec<Global>),
    GRAS(GroupVec<Grass>),
    HAZD(GroupVec<Hazard>),
    HDPT(GroupVec<HeadPart>),
    IDLE(GroupVec<IdleAnimation>),
    IDLM(GroupVec<IdleMarker>),
    IMAD(GroupVec<ImageSpaceAdapter>),
    IMGS(GroupVec<ImageSpace>),
    INGR(GroupVec<Ingredient>),
    INNR(GroupVec<InstanceNamingRules>),
    IPCT(GroupVec<Impact>),
    IPDS(GroupVec<ImpactDataSet>),
    KEYM(GroupVec<Key>),
    KYWD(GroupVec<Keyword>),
    KSSM(GroupVec<KeywordSoundMapping>),
    // LAND is not a top-level group
    LAYR(GroupVec<Layer>),
    LCRT(GroupVec<LocationReferenceType>),
    LCTN(GroupVec<Location>),
    LENS(GroupVec<LensFlare>),
    LGTM(GroupVec<LightingTemplate>),
    LIGH(GroupVec<Light>),
    LSCR(GroupVec<LoadingScreen>),
    LTEX(GroupVec<LandscapeTexture>),
    LVLI(GroupVec<LeveledItem>),
    LVLN(GroupVec<LeveledNPC>),
    MATO(GroupVec<MaterialObject>),
    MATT(GroupVec<MaterialType>),
    MESG(GroupVec<Message>),
    MGEF(GroupVec<MagicEffect>),
    MISC(GroupVec<MiscItem>),
    MOVT(GroupVec<MovementType>),
    MSTT(GroupVec<MoveableStatic>),
    MSWP(GroupVec<MaterialSwap>),
    MUSC(GroupVec<MusicType>),
    MUST(GroupVec<MusicTrack>),
    NAVI(GroupVec<NavigationMeshInfoMap>),
    NOCM(GroupVec<NavObstacleManager>),
    NOTE(GroupVec<Note>),
    NPC_(GroupVec<NonPlayerCharacter>),
    OMOD(GroupVec<ObjectModification>),
    OTFT(GroupVec<Outfit>),
    OVIS(GroupVec<ObjectVisibility>),
    PACK(GroupVec<Package>),
    PERK(GroupVec<Perk>),
    PKIN(GroupVec<PackIn>),
    PROJ(GroupVec<Projectile>),
    QUST(GroupVec<Quest>),
    RACE(GroupVec<Race>),
    REGN(GroupVec<Region>),
    RELA(GroupVec<Relationship>),
    REVB(GroupVec<Reverb>),
    RFCT(GroupVec<VisualEffect>),
    RFGP(GroupVec<ReferenceGroup>),
    SCCO(GroupVec<SceneCollection>),
    SCOL(GroupVec<StaticCollection>),
    SCSN(GroupVec<AudioCategorySnapshot>),
    SMBN(GroupVec<StoryManagerBranchNode>),
    SMEN(GroupVec<StoryManagerEventNode>),
    SMQN(GroupVec<StoryManagerQuestNode>),
    SNCT(GroupVec<SoundCategory>),
    SNDR(GroupVec<SoundDescriptor>),
    SOPM(GroupVec<SoundOutputModel>),
    SOUN(GroupVec<SoundMarker>),
    SPEL(GroupVec<Spell>),
    SPGD(GroupVec<ShaderParticleGeometry>),
    STAG(GroupVec<SoundTag>),
    STAT(GroupVec<Static>),
    TACT(GroupVec<TalkingActivator>),
    TERM(GroupVec<Terminal>),
    TREE(GroupVec<Tree>),
    TRNS(GroupVec<Transform>),
    TXST(GroupVec<TextureSet>),
    VTYP(GroupVec<VoiceType>),
    WATR(GroupVec<Water>),
    WEAP(GroupVec<Weapon>),
    WRLD(GroupVec<WorldEntry>),
    WTHR(GroupVec<Weather>),
    ZOOM(GroupVec<Zoom>),
}

impl Parse<&[u8]> for TopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;

        if data.is_empty() {
            return Ok((i, TopGroup::Unhandled(GroupVec { header, data: Vec::new() })));
        }

        //println!("Parsing TopGroup: {:?}", header.label);
        match header.label {
            GroupLabel::Top(label) => {
                match &label.0 {
                    b"AACT" => {
                        let (_, group) = many0(Action::parse)(data)?;
                        Ok((i, TopGroup::AACT(GroupVec { header, data: group })))
                    }
                    b"ACTI" => {
                        let (_, group) = many0(Activator::parse)(data)?;
                        Ok((i, TopGroup::ACTI(GroupVec { header, data: group })))
                    }
                    b"ADDN" => {
                        let (_, group) = many0(AddonNode::parse)(data)?;
                        Ok((i, TopGroup::ADDN(GroupVec { header, data: group })))
                    }
                    b"AECH" => {
                        let (_, group) = many0(AudioEffectChain::parse)(data)?;
                        Ok((i, TopGroup::AECH(GroupVec { header, data: group })))
                    }
                    b"ALCH" => {
                        let (_, group) = many0(Alchemy::parse)(data)?;
                        Ok((i, TopGroup::ALCH(GroupVec { header, data: group })))
                    }
                    b"AMDL" => {
                        let (_, group) = many0(AimModel::parse)(data)?;
                        Ok((i, TopGroup::AMDL(GroupVec { header, data: group })))
                    }
                    b"AMMO" => {
                        let (_, group) = many0(Ammo::parse)(data)?;
                        Ok((i, TopGroup::AMMO(GroupVec { header, data: group })))
                    }
                    b"ANIO" => {
                        let (_, group) = many0(AnimatedObject::parse)(data)?;
                        Ok((i, TopGroup::ANIO(GroupVec { header, data: group })))
                    }
                    b"AORU" => {
                        let (_, group) = many0(AttractionRule::parse)(data)?;
                        Ok((i, TopGroup::AORU(GroupVec { header, data: group })))
                    }
                    b"ARMA" => {
                        let (_, group) = many0(ArmorAddon::parse)(data)?;
                        Ok((i, TopGroup::ARMA(GroupVec { header, data: group })))
                    }
                    b"ARMO" => {
                        let (_, group) = many0(Armor::parse)(data)?;
                        Ok((i, TopGroup::ARMO(GroupVec { header, data: group })))
                    }
                    b"ARTO" => {
                        let (_, group) = many0(ArtObject::parse)(data)?;
                        Ok((i, TopGroup::ARTO(GroupVec { header, data: group })))
                    }
                    b"ASPC" => {
                        let (_, group) = many0(AcousticSpace::parse)(data)?;
                        Ok((i, TopGroup::ASPC(GroupVec { header, data: group })))
                    }
                    b"ASTP" => {
                        let (_, group) = many0(AssociationType::parse)(data)?;
                        Ok((i, TopGroup::ASTP(GroupVec { header, data: group })))
                    }
                    b"AVIF" => {
                        let (_, group) = many0(ActorValueInformation::parse)(data)?;
                        Ok((i, TopGroup::AVIF(GroupVec { header, data: group })))
                    }
                    b"BOOK" => {
                        let (_, group) = many0(Book::parse)(data)?;
                        Ok((i, TopGroup::BOOK(GroupVec { header, data: group })))
                    }
                    b"BPTD" => {
                        let (_, group) = many0(BodyPartData::parse)(data)?;
                        Ok((i, TopGroup::BPTD(GroupVec { header, data: group })))
                    }
                    b"BNDS" => {
                        let (_, group) = many0(BendableSpline::parse)(data)?;
                        Ok((i, TopGroup::BNDS(GroupVec { header, data: group })))
                    }
                    b"CAMS" => {
                        let (_, group) = many0(CameraShot::parse)(data)?;
                        Ok((i, TopGroup::CAMS(GroupVec { header, data: group })))
                    }
                    b"CELL" => {
                        //let (i, (header, group)) = alloc_group(i)?;
                        Ok((i, TopGroup::CELL(GroupVec { header, data: Vec::new()})))
                    }
                    b"CLAS" => {
                        let (_, group) = many0(Class::parse)(data)?;
                        Ok((i, TopGroup::CLAS(GroupVec { header, data: group })))
                    }
                    b"CLFM" => {
                        let (_, group) = many0(Color::parse)(data)?;
                        Ok((i, TopGroup::CLFM(GroupVec { header, data: group })))
                    }
                    b"CLMT" => {
                        let (_, group) = many0(Climate::parse)(data)?;
                        Ok((i, TopGroup::CLMT(GroupVec { header, data: group })))
                    }
                    b"CMPO" => {
                        let (_, group) = many0(Component::parse)(data)?;
                        Ok((i, TopGroup::CMPO(GroupVec { header, data: group })))
                    }
                    b"COBJ" => {
                        let (_, group) = many0(ConstructibleObject::parse)(data)?;
                        Ok((i, TopGroup::COBJ(GroupVec { header, data: group })))
                    }
                    b"COLL" => {
                        let (_, group) = many0(CollisionLayer::parse)(data)?;
                        Ok((i, TopGroup::COLL(GroupVec { header, data: group })))
                    }
                    b"CONT" => {
                        let (_, group) = many0(Container::parse)(data)?;
                        Ok((i, TopGroup::CONT(GroupVec { header, data: group })))
                    }
                    b"CPTH" => {
                        let (_, group) = many0(CameraPath::parse)(data)?;
                        Ok((i, TopGroup::CPTH(GroupVec { header, data: group })))
                    }
                    b"CSTY" => {
                        let (_, group) = many0(CombatStyle::parse)(data)?;
                        Ok((i, TopGroup::CSTY(GroupVec { header, data: group })))
                    }
                    b"DEBR" => {
                        let (_, group) = many0(Debris::parse)(data)?;
                        Ok((i, TopGroup::DEBR(GroupVec { header, data: group })))
                    }   
                    b"DFOB" => {
                        let (_, group) = many0(DefaultObject::parse)(data)?;
                        Ok((i, TopGroup::DFOB(GroupVec { header, data: group })))
                    }
                    b"DLVW" => {
                        let (_, group) = many0(DialogView::parse)(data)?;
                        Ok((i, TopGroup::DLVW(GroupVec { header, data: group })))
                    }
                    b"DMGT" => {
                        let (_, group) = many0(DamageType::parse)(data)?;
                        Ok((i, TopGroup::DMGT(GroupVec { header, data: group })))
                    }
                    b"DOBJ" => {
                        let (_, group) = many0(DefaultObjects::parse)(data)?;
                        Ok((i, TopGroup::DOBJ(GroupVec { header, data: group })))
                    }
                    b"DOOR" => {
                        let (_, group) = many0(Door::parse)(data)?;
                        Ok((i, TopGroup::DOOR(GroupVec { header, data: group })))
                    }
                    b"ECZN" => {
                        let (_, group) = many0(EncounterZone::parse)(data)?;
                        Ok((i, TopGroup::ECZN(GroupVec { header, data: group })))
                    }
                    b"EFSH" => {
                        let (_, group) = many0(EffectShader::parse)(data)?;
                        Ok((i, TopGroup::EFSH(GroupVec { header, data: group })))
                    }
                    b"ENCH" => {
                        let (_, group) = many0(ObjectEffect::parse)(data)?;
                        Ok((i, TopGroup::ENCH(GroupVec { header, data: group })))
                    }
                    b"EQUP" => {
                        let (_, group) = many0(EquipType::parse)(data)?;
                        Ok((i, TopGroup::EQUP(GroupVec { header, data: group })))
                    }
                    b"EXPL" => {
                        let (_, group) = many0(Explosion::parse)(data)?;
                        Ok((i, TopGroup::EXPL(GroupVec { header, data: group })))
                    }
                    b"FACT" => {
                        let (_, group) = many0(Faction::parse)(data)?;
                        Ok((i, TopGroup::FACT(GroupVec { header, data: group })))
                    }
                    b"FLOR" => {
                        let (_, group) = many0(Flora::parse)(data)?;
                        Ok((i, TopGroup::FLOR(GroupVec { header, data: group })))
                    }
                    b"FLST" => {
                        let (_, group) = many0(FormIdList::parse)(data)?;
                        Ok((i, TopGroup::FLST(GroupVec { header, data: group })))
                    }
                    b"FSTP" => {
                        let (_, group) = many0(Footstep::parse)(data)?;
                        Ok((i, TopGroup::FSTP(GroupVec { header, data: group })))
                    }
                    b"FSTS" => {
                        let (_, group) = many0(FootstepSet::parse)(data)?;
                        Ok((i, TopGroup::FSTS(GroupVec { header, data: group })))
                    }
                    b"FURN" => {
                        let (_, group) = many0(Furniture::parse)(data)?;
                        Ok((i, TopGroup::FURN(GroupVec { header, data: group })))
                    }
                    b"GMST" => {
                        let (_, group) = many0(GameSetting::parse)(data)?;
                        Ok((i, TopGroup::GMST(GroupVec { header, data: group })))
                    }
                    b"GDRY" => {
                        let (_, group) = many0(Godray::parse)(data)?;
                        Ok((i, TopGroup::GDRY(GroupVec { header, data: group })))
                    }
                    b"GLOB" => {
                        let (_, group) = many0(Global::parse)(data)?;
                        Ok((i, TopGroup::GLOB(GroupVec { header, data: group })))
                    }
                    b"GRAS" => {
                        let (_, group) = many0(Grass::parse)(data)?;
                        Ok((i, TopGroup::GRAS(GroupVec { header, data: group })))
                    }
                    b"HAZD" => {
                        let (_, group) = many0(Hazard::parse)(data)?;
                        Ok((i, TopGroup::HAZD(GroupVec { header, data: group })))
                    }
                    b"HDPT" => {
                        let (_, group) = many0(HeadPart::parse)(data)?;
                        Ok((i, TopGroup::HDPT(GroupVec { header, data: group })))
                    }
                    b"IDLE" => {
                        let (_, group) = many0(IdleAnimation::parse)(data)?;
                        Ok((i, TopGroup::IDLE(GroupVec { header, data: group })))
                    }
                    b"IDLM" => {
                        let (_, group) = many0(IdleMarker::parse)(data)?;
                        Ok((i, TopGroup::IDLM(GroupVec { header, data: group })))
                    }
                    b"IMAD" => {
                        let (_, group) = many0(ImageSpaceAdapter::parse)(data)?;
                        Ok((i, TopGroup::IMAD(GroupVec { header, data: group })))
                    }
                    b"IMGS" => {
                        let (_, group) = many0(ImageSpace::parse)(data)?;
                        Ok((i, TopGroup::IMGS(GroupVec { header, data: group })))
                    }
                    b"INGR" => {
                        let (_, group) = many0(Ingredient::parse)(data)?;
                        Ok((i, TopGroup::INGR(GroupVec { header, data: group })))
                    }
                    b"INNR" => {
                        let (_, group) = many0(InstanceNamingRules::parse)(data)?;
                        Ok((i, TopGroup::INNR(GroupVec { header, data: group })))
                    }
                    b"IPCT" => {
                        let (_, group) = many0(Impact::parse)(data)?;
                        Ok((i, TopGroup::IPCT(GroupVec { header, data: group })))
                    }
                    b"IPDS" => {
                        let (_, group) = many0(ImpactDataSet::parse)(data)?;
                        Ok((i, TopGroup::IPDS(GroupVec { header, data: group })))
                    }
                    b"KEYM" => {
                        let (_, group) = many0(Key::parse)(data)?;
                        Ok((i, TopGroup::KEYM(GroupVec { header, data: group })))
                    }
                    b"KYWD" => {
                        let (_, group) = many0(Keyword::parse)(data)?;
                        Ok((i, TopGroup::KYWD(GroupVec { header, data: group })))
                    }
                    b"KSSM" => {
                        let (_, group) = many0(KeywordSoundMapping::parse)(data)?;
                        Ok((i, TopGroup::KSSM(GroupVec { header, data: group })))
                    }
                    b"LAYR" => {
                        let (_, group) = many0(Layer::parse)(data)?;
                        Ok((i, TopGroup::LAYR(GroupVec { header, data: group })))
                    }
                    b"LCRT" => {
                        let (_, group) = many0(LocationReferenceType::parse)(data)?;
                        Ok((i, TopGroup::LCRT(GroupVec { header, data: group })))
                    }
                    b"LCTN" => {
                        let (_, group) = many0(Location::parse)(data)?;
                        Ok((i, TopGroup::LCTN(GroupVec { header, data: group })))
                    }
                    b"LENS" => {
                        let (_, group) = many0(LensFlare::parse)(data)?;
                        Ok((i, TopGroup::LENS(GroupVec { header, data: group })))
                    }
                    b"LGTM" => {
                        let (_, group) = many0(LightingTemplate::parse)(data)?;
                        Ok((i, TopGroup::LGTM(GroupVec { header, data: group })))
                    }
                    b"LIGH" => {
                        let (_, group) = many0(Light::parse)(data)?;
                        Ok((i, TopGroup::LIGH(GroupVec { header, data: group })))
                    }
                    b"LSCR" => {
                        let (_, group) = many0(LoadingScreen::parse)(data)?;
                        Ok((i, TopGroup::LSCR(GroupVec { header, data: group })))
                    }
                    b"LTEX" => {
                        let (_, group) = many0(LandscapeTexture::parse)(data)?;
                        Ok((i, TopGroup::LTEX(GroupVec { header, data: group })))
                    }
                    b"LVLI" => {
                        let (_, group) = many0(LeveledItem::parse)(data)?;
                        Ok((i, TopGroup::LVLI(GroupVec { header, data: group })))
                    }
                    b"LVLN" => {
                        let (_, group) = many0(LeveledNPC::parse)(data)?;
                        Ok((i, TopGroup::LVLN(GroupVec { header, data: group })))
                    }
                    b"MATO" => {
                        let (_, group) = many0(MaterialObject::parse)(data)?;
                        Ok((i, TopGroup::MATO(GroupVec { header, data: group })))
                    }
                    b"MATT" => {
                        let (_, group) = many0(MaterialType::parse)(data)?;
                        Ok((i, TopGroup::MATT(GroupVec { header, data: group })))
                    }
                    b"MESG" => {
                        let (_, group) = many0(Message::parse)(data)?;
                        Ok((i, TopGroup::MESG(GroupVec { header, data: group })))
                    }
                    b"MGEF" => {
                        let (_, group) = many0(MagicEffect::parse)(data)?;
                        Ok((i, TopGroup::MGEF(GroupVec { header, data: group })))
                    }
                    b"MISC" => {
                        let (_, group) = many0(MiscItem::parse)(data)?;
                        Ok((i, TopGroup::MISC(GroupVec { header, data: group })))
                    }
                    b"MOVT" => {
                        let (_, group) = many0(MovementType::parse)(data)?;
                        Ok((i, TopGroup::MOVT(GroupVec { header, data: group })))
                    }
                    b"MSTT" => {
                        let (_, group) = many0(MoveableStatic::parse)(data)?;
                        Ok((i, TopGroup::MSTT(GroupVec { header, data: group } )))
                    }
                    b"MSWP" => {
                        let (_, group) = many0(MaterialSwap::parse)(data)?;
                        Ok((i, TopGroup::MSWP(GroupVec { header, data: group })))
                    }
                    b"MUSC" => {
                        let (_, group) = many0(MusicType::parse)(data)?;
                        Ok((i, TopGroup::MUSC(GroupVec { header, data: group })))
                    }
                    b"MUST" => {
                        let (_, group) = many0(MusicTrack::parse)(data)?;
                        Ok((i, TopGroup::MUST(GroupVec { header, data: group })))
                    }
                    b"NAVI" => {
                        let (_, group) = many0(NavigationMeshInfoMap::parse)(data)?;
                        Ok((i, TopGroup::NAVI(GroupVec { header, data: group })))
                    }
                    b"NOCM" => {
                        let (_, group) = many0(NavObstacleManager::parse)(data)?; 
                        Ok((i, TopGroup::NOCM(GroupVec { header, data: group })))
                    }
                    b"NOTE" => {
                        let (_, group) = many0(Note::parse)(data)?;
                        Ok((i, TopGroup::NOTE(GroupVec { header, data: group })))
                    }
                    b"NPC_" => {
                        let (_, group) = many0(NonPlayerCharacter::parse)(data)?;
                        Ok((i, TopGroup::NPC_(GroupVec { header, data: group })))
                    }
                    // b"NPC_" => {
                    //     let (i, (header, raw)) = alloc_group(i)?;
                    //     Ok((i, TopGroup::NPC_(Group { header, data: Vec::new()})))
                    // }
                    b"OMOD" => {
                        let (_, group) = many0(ObjectModification::parse)(data)?;
                        Ok((i, TopGroup::OMOD(GroupVec { header, data: group })))
                    }
                    b"OTFT" => {
                        let (_, group) = many0(Outfit::parse)(data)?;
                        Ok((i, TopGroup::OTFT(GroupVec { header, data: group })))
                    }
                    b"OVIS" => {
                        let (_, group) = many0(ObjectVisibility::parse)(data)?;
                        Ok((i, TopGroup::OVIS(GroupVec { header, data: group })))
                    }
                    b"PACK" => {
                        let (_, group) = many0(Package::parse)(data)?;
                        Ok((i, TopGroup::PACK(GroupVec { header, data: group })))
                    }
                    b"PERK" => {
                        let (_, group) = many0(Perk::parse)(data)?;
                        Ok((i, TopGroup::PERK(GroupVec { header, data: group })))
                    }
                    b"PKIN" => {
                        let (_, group) = many0(PackIn::parse)(data)?;
                        Ok((i, TopGroup::PKIN(GroupVec { header, data: group })))
                    }
                    b"PROJ" => {
                        let (_, group) = many0(Projectile::parse)(data)?;
                        Ok((i, TopGroup::PROJ(GroupVec { header, data: group })))
                    }
                    b"QUST" => {
                        //let (_, (header, raw)) = alloc_group(i)?;
                        Ok((i, TopGroup::QUST(GroupVec { header, data: Vec::new()})))
                    }
                    b"RACE" => {
                        let (_, group) = many0(Race::parse)(data)?;
                        Ok((i, TopGroup::RACE(GroupVec { header, data: group })))
                    }
                    b"REGN" => {
                        let (_, group) = many0(Region::parse)(data)?;
                        Ok((i, TopGroup::REGN(GroupVec { header, data: group })))
                    }
                    b"RELA" => {
                        let (_, group) = many0(Relationship::parse)(data)?;
                        Ok((i, TopGroup::RELA(GroupVec { header, data: group })))
                    }
                    b"REVB" => {
                        let (_, group) = many0(Reverb::parse)(data)?;
                        Ok((i, TopGroup::REVB(GroupVec { header, data: group })))
                    }
                    b"RFCT" => {
                        let (_, group) = many0(VisualEffect::parse)(data)?;
                        Ok((i, TopGroup::RFCT(GroupVec { header, data: group })))
                    }
                    b"RFGP" => {
                        let (_, group) = many0(ReferenceGroup::parse)(data)?;
                        Ok((i, TopGroup::RFGP(GroupVec { header, data: group })))
                    }
                    b"SCCO" => {
                        let (_, group) = many0(SceneCollection::parse)(data)?;
                        Ok((i, TopGroup::SCCO(GroupVec { header, data: group })))
                    }
                    b"SCOL" => {
                        let (_, group) = many0(StaticCollection::parse)(data)?;
                        Ok((i, TopGroup::SCOL(GroupVec { header, data: group })))
                    }
                    b"SCSN" => {
                        let (_, group) = many0(AudioCategorySnapshot::parse)(data)?;
                        Ok((i, TopGroup::SCSN(GroupVec { header, data: group })))
                    }
                    b"SMBN" => {
                        let (_, group) = many0(StoryManagerBranchNode::parse)(data)?;
                        Ok((i, TopGroup::SMBN(GroupVec { header, data: group })))
                    }
                    b"SMEN" => {
                        let (_, group) = many0(StoryManagerEventNode::parse)(data)?;
                        Ok((i, TopGroup::SMEN(GroupVec { header, data: group })))
                    }
                    b"SMQN" => {
                        let (_, group) = many0(StoryManagerQuestNode::parse)(data)?;
                        Ok((i, TopGroup::SMQN(GroupVec { header, data: group })))
                    }
                    b"SNCT" => {
                        let (_, group) = many0(SoundCategory::parse)(data)?;
                        Ok((i, TopGroup::SNCT(GroupVec { header, data: group })))
                    }
                    b"SNDR" => {
                        let (_, group) = many0(SoundDescriptor::parse)(data)?;
                        Ok((i, TopGroup::SNDR(GroupVec { header, data: group })))
                    }
                    b"SOPM" => {
                        let (_, group) = many0(SoundOutputModel::parse)(data)?;
                        Ok((i, TopGroup::SOPM(GroupVec { header, data: group })))
                    }
                    b"SOUN" => {
                        let (_, group) = many0(SoundMarker::parse)(data)?;
                        Ok((i, TopGroup::SOUN(GroupVec { header, data: group })))
                    }
                    b"SPEL" => {
                        let (_, group) = many0(Spell::parse)(data)?;
                        Ok((i, TopGroup::SPEL(GroupVec { header, data: group })))
                    }
                    b"SPGD" => {
                        let (_, group) = many0(ShaderParticleGeometry::parse)(data)?;
                        Ok((i, TopGroup::SPGD(GroupVec { header, data: group })))
                    }
                    b"STAG" => {
                        let (_, group) = many0(SoundTag::parse)(data)?;
                        Ok((i, TopGroup::STAG(GroupVec { header, data: group })))
                    }
                    b"STAT" => {
                        let (_, group) = many0(Static::parse)(data)?;
                        Ok((i, TopGroup::STAT(GroupVec { header, data: group })))
                    }
                    b"TACT" => {
                        let (_, group) = many0(TalkingActivator::parse)(data)?;
                        Ok((i, TopGroup::TACT(GroupVec { header, data: group })))
                    }
                    b"TERM" => {
                        let (_, group) = many0(Terminal::parse)(data)?;
                        Ok((i, TopGroup::TERM(GroupVec { header, data: group })))
                    }
                    b"TREE" => {
                        let (_, group) = many0(Tree::parse)(data)?;
                        Ok((i, TopGroup::TREE(GroupVec { header, data: group })))
                    }
                    b"TRNS" => {
                        let (_, group) = many0(Transform::parse)(data)?;
                        Ok((i, TopGroup::TRNS(GroupVec { header, data: group })))
                    }
                    b"TXST" => {
                        let (_, group) = many0(TextureSet::parse)(data)?;
                        Ok((i, TopGroup::TXST(GroupVec { header, data: group })))
                    }
                    b"VTYP" => {
                        let (_, group) = many0(VoiceType::parse)(data)?;
                        Ok((i, TopGroup::VTYP(GroupVec { header, data: group })))
                    }
                    b"WATR" => {
                        let (_, group) = many0(Water::parse)(data)?;
                        Ok((i, TopGroup::WATR(GroupVec { header, data: group })))
                    }
                    b"WEAP" => {
                        let (_, group) = many0(Weapon::parse)(data)?;
                        Ok((i, TopGroup::WEAP(GroupVec { header, data: group })))
                    }
                    b"WRLD" => {
                        let (_, we) = many0(WorldEntry::parse)(data)?;
                        Ok((i, TopGroup::WRLD(GroupVec { header, data: we })))
                    }
                    b"WTHR" => {
                        let (_, group) = many0(Weather::parse)(data)?;
                        Ok((i, TopGroup::WTHR(GroupVec { header, data: group })))
                    }
                    b"ZOOM" => {
                        let (_, group) = many0(Zoom::parse)(data)?;
                        Ok((i, TopGroup::ZOOM(GroupVec { header, data: group })))
                    }

                    _ => {
                        unimplemented!("Top group {} not implemented", label);
                    }
                }
            }
            _ => {
                Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Tag)))
            }
        }



    }
}