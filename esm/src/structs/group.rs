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
pub struct Group<T> {
    pub header: GroupHeader,
    pub data: Vec<T>
}

impl<T: for<'esm> Parse<&'esm[u8]>> Parse<&[u8]> for Group<T> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
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
    Unhandled(Group<RawRecord<'static>>),
    AACT(Group<Action>),
    ACTI(Group<Activator>),
    ADDN(Group<AddonNode>),
    AECH(Group<AudioEffectChain>),
    ALCH(Group<Alchemy>),
    AMDL(Group<AimModel>),
    AMMO(Group<Ammo>),
    ANIO(Group<AnimatedObject>),
    AORU(Group<AttractionRule>),
    ARMA(Group<ArmorAddon>),
    ARMO(Group<Armor>),
    ARTO(Group<ArtObject>),
    ASPC(Group<AcousticSpace>),
    ASTP(Group<AssociationType>),
    AVIF(Group<ActorValueInformation>),
    BNDS(Group<BendableSpline>),
    BOOK(Group<Book>),
    BPTD(Group<BodyPartData>),
    CAMS(Group<CameraShot>),
    CELL(Group<Cell>),
    CLAS(Group<Class>),
    CLFM(Group<Color>),
    CLMT(Group<Climate>),
    CMPO(Group<Component>),
    COBJ(Group<ConstructibleObject>),
    COLL(Group<CollisionLayer>),
    CONT(Group<Container>),
    CPTH(Group<CameraPath>),
    CSTY(Group<CombatStyle>),
    DEBR(Group<Debris>),
    DFOB(Group<DefaultObject>),
    DLVW(Group<DialogView>),
    DMGT(Group<DamageType>),
    DOBJ(Group<DefaultObjects>),
    DOOR(Group<Door>),
    ECZN(Group<EncounterZone>),
    EFSH(Group<EffectShader>),
    ENCH(Group<ObjectEffect>),
    EQUP(Group<EquipType>),
    EXPL(Group<Explosion>),
    FACT(Group<Faction>),
    FLOR(Group<Flora>),
    FLST(Group<FormIdList>),
    FSTP(Group<Footstep>),
    FSTS(Group<FootstepSet>),
    FURN(Group<Furniture>),
    GMST(Group<GameSetting>),
    GDRY(Group<Godray>),
    GLOB(Group<Global>),
    GRAS(Group<Grass>),
    HAZD(Group<Hazard>),
    HDPT(Group<HeadPart>),
    IDLE(Group<IdleAnimation>),
    IDLM(Group<IdleMarker>),
    IMAD(Group<ImageSpaceAdapter>),
    IMGS(Group<ImageSpace>),
    INGR(Group<Ingredient>),
    INNR(Group<InstanceNamingRules>),
    IPCT(Group<Impact>),
    IPDS(Group<ImpactDataSet>),
    KEYM(Group<Key>),
    KYWD(Group<Keyword>),
    KSSM(Group<KeywordSoundMapping>),
    // LAND is not a top-level group
    LAYR(Group<Layer>),
    LCRT(Group<LocationReferenceType>),
    LCTN(Group<Location>),
    LENS(Group<LensFlare>),
    LGTM(Group<LightingTemplate>),
    LIGH(Group<Light>),
    LSCR(Group<LoadingScreen>),
    LTEX(Group<LandscapeTexture>),
    LVLI(Group<LeveledItem>),
    LVLN(Group<LeveledNPC>),
    MATO(Group<MaterialObject>),
    MATT(Group<MaterialType>),
    MESG(Group<Message>),
    MGEF(Group<MagicEffect>),
    MISC(Group<MiscItem>),
    MOVT(Group<MovementType>),
    MSTT(Group<MoveableStatic>),
    MSWP(Group<MaterialSwap>),
    MUSC(Group<MusicType>),
    MUST(Group<MusicTrack>),
    NAVI(Group<NavigationMeshInfoMap>),
    NOCM(Group<NavObstacleManager>),
    NOTE(Group<Note>),
    NPC_(Group<NonPlayerCharacter>),
    OMOD(Group<ObjectModification>),
    OTFT(Group<Outfit>),
    OVIS(Group<ObjectVisibility>),
    PACK(Group<Package>),
    PERK(Group<Perk>),
    PKIN(Group<PackIn>),
    PROJ(Group<Projectile>),
    QUST(Group<Quest>),
    RACE(Group<Race>),
    REGN(Group<Region>),
    RELA(Group<Relationship>),
    REVB(Group<Reverb>),
    RFCT(Group<VisualEffect>),
    RFGP(Group<ReferenceGroup>),
    SCCO(Group<SceneCollection>),
    SCOL(Group<StaticCollection>),
    SCSN(Group<AudioCategorySnapshot>),
    SMBN(Group<StoryManagerBranchNode>),
    SMEN(Group<StoryManagerEventNode>),
    SMQN(Group<StoryManagerQuestNode>),
    SNCT(Group<SoundCategory>),
    SNDR(Group<SoundDescriptor>),
    SOPM(Group<SoundOutputModel>),
    SOUN(Group<SoundMarker>),
    SPEL(Group<Spell>),
    SPGD(Group<ShaderParticleGeometry>),
    STAG(Group<SoundTag>),
    STAT(Group<Static>),
    TACT(Group<TalkingActivator>),
    TERM(Group<Terminal>),
    TREE(Group<Tree>),
    TRNS(Group<Transform>),
    TXST(Group<TextureSet>),
    VTYP(Group<VoiceType>),
    WATR(Group<Water>),
    WEAP(Group<Weapon>),
    WRLD(Group<WorldEntry>),
    WTHR(Group<Weather>),
    ZOOM(Group<Zoom>),
}

impl Parse<&[u8]> for TopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;

        //println!("Parsing TopGroup: {:?}", header.label);
        match header.label {
            GroupLabel::Top(label) => {
                match &label.0 {
                    b"AACT" => {
                        let (_, group) = many0(Action::parse)(data)?;
                        Ok((i, TopGroup::AACT(Group { header, data: group })))
                    }
                    b"ACTI" => {
                        let (_, group) = many0(Activator::parse)(data)?;
                        Ok((i, TopGroup::ACTI(Group { header, data: group })))
                    }
                    b"ADDN" => {
                        let (_, group) = many0(AddonNode::parse)(data)?;
                        Ok((i, TopGroup::ADDN(Group { header, data: group })))
                    }
                    b"AECH" => {
                        let (_, group) = many0(AudioEffectChain::parse)(data)?;
                        Ok((i, TopGroup::AECH(Group { header, data: group })))
                    }
                    b"ALCH" => {
                        let (_, group) = many0(Alchemy::parse)(data)?;
                        Ok((i, TopGroup::ALCH(Group { header, data: group })))
                    }
                    b"AMDL" => {
                        let (_, group) = many0(AimModel::parse)(data)?;
                        Ok((i, TopGroup::AMDL(Group { header, data: group })))
                    }
                    b"AMMO" => {
                        let (_, group) = many0(Ammo::parse)(data)?;
                        Ok((i, TopGroup::AMMO(Group { header, data: group })))
                    }
                    b"ANIO" => {
                        let (_, group) = many0(AnimatedObject::parse)(data)?;
                        Ok((i, TopGroup::ANIO(Group { header, data: group })))
                    }
                    b"AORU" => {
                        let (_, group) = many0(AttractionRule::parse)(data)?;
                        Ok((i, TopGroup::AORU(Group { header, data: group })))
                    }
                    b"ARMA" => {
                        let (_, group) = many0(ArmorAddon::parse)(data)?;
                        Ok((i, TopGroup::ARMA(Group { header, data: group })))
                    }
                    b"ARMO" => {
                        let (_, group) = many0(Armor::parse)(data)?;
                        Ok((i, TopGroup::ARMO(Group { header, data: group })))
                    }
                    b"ARTO" => {
                        let (_, group) = many0(ArtObject::parse)(data)?;
                        Ok((i, TopGroup::ARTO(Group { header, data: group })))
                    }
                    b"ASPC" => {
                        let (_, group) = many0(AcousticSpace::parse)(data)?;
                        Ok((i, TopGroup::ASPC(Group { header, data: group })))
                    }
                    b"ASTP" => {
                        let (_, group) = many0(AssociationType::parse)(data)?;
                        Ok((i, TopGroup::ASTP(Group { header, data: group })))
                    }
                    b"AVIF" => {
                        let (_, group) = many0(ActorValueInformation::parse)(data)?;
                        Ok((i, TopGroup::AVIF(Group { header, data: group })))
                    }
                    b"BOOK" => {
                        let (_, group) = many0(Book::parse)(data)?;
                        Ok((i, TopGroup::BOOK(Group { header, data: group })))
                    }
                    b"BPTD" => {
                        let (_, group) = many0(BodyPartData::parse)(data)?;
                        Ok((i, TopGroup::BPTD(Group { header, data: group })))
                    }
                    b"BNDS" => {
                        let (_, group) = many0(BendableSpline::parse)(data)?;
                        Ok((i, TopGroup::BNDS(Group { header, data: group })))
                    }
                    b"CAMS" => {
                        let (_, group) = many0(CameraShot::parse)(data)?;
                        Ok((i, TopGroup::CAMS(Group { header, data: group })))
                    }
                    b"CELL" => {
                        //let (i, (header, group)) = alloc_group(i)?;
                        Ok((i, TopGroup::CELL(Group { header, data: Vec::new()})))
                    }
                    b"CLAS" => {
                        let (_, group) = many0(Class::parse)(data)?;
                        Ok((i, TopGroup::CLAS(Group { header, data: group })))
                    }
                    b"CLFM" => {
                        let (_, group) = many0(Color::parse)(data)?;
                        Ok((i, TopGroup::CLFM(Group { header, data: group })))
                    }
                    b"CLMT" => {
                        let (_, group) = many0(Climate::parse)(data)?;
                        Ok((i, TopGroup::CLMT(Group { header, data: group })))
                    }
                    b"CMPO" => {
                        let (_, group) = many0(Component::parse)(data)?;
                        Ok((i, TopGroup::CMPO(Group { header, data: group })))
                    }
                    b"COBJ" => {
                        let (_, group) = many0(ConstructibleObject::parse)(data)?;
                        Ok((i, TopGroup::COBJ(Group { header, data: group })))
                    }
                    b"COLL" => {
                        let (_, group) = many0(CollisionLayer::parse)(data)?;
                        Ok((i, TopGroup::COLL(Group { header, data: group })))
                    }
                    b"CONT" => {
                        let (_, group) = many0(Container::parse)(data)?;
                        Ok((i, TopGroup::CONT(Group { header, data: group })))
                    }
                    b"CPTH" => {
                        let (_, group) = many0(CameraPath::parse)(data)?;
                        Ok((i, TopGroup::CPTH(Group { header, data: group })))
                    }
                    b"CSTY" => {
                        let (_, group) = many0(CombatStyle::parse)(data)?;
                        Ok((i, TopGroup::CSTY(Group { header, data: group })))
                    }
                    b"DEBR" => {
                        let (_, group) = many0(Debris::parse)(data)?;
                        Ok((i, TopGroup::DEBR(Group { header, data: group })))
                    }   
                    b"DFOB" => {
                        let (_, group) = many0(DefaultObject::parse)(data)?;
                        Ok((i, TopGroup::DFOB(Group { header, data: group })))
                    }
                    b"DLVW" => {
                        let (_, group) = many0(DialogView::parse)(data)?;
                        Ok((i, TopGroup::DLVW(Group { header, data: group })))
                    }
                    b"DMGT" => {
                        let (_, group) = many0(DamageType::parse)(data)?;
                        Ok((i, TopGroup::DMGT(Group { header, data: group })))
                    }
                    b"DOBJ" => {
                        let (_, group) = many0(DefaultObjects::parse)(data)?;
                        Ok((i, TopGroup::DOBJ(Group { header, data: group })))
                    }
                    b"DOOR" => {
                        let (_, group) = many0(Door::parse)(data)?;
                        Ok((i, TopGroup::DOOR(Group { header, data: group })))
                    }
                    b"ECZN" => {
                        let (_, group) = many0(EncounterZone::parse)(data)?;
                        Ok((i, TopGroup::ECZN(Group { header, data: group })))
                    }
                    b"EFSH" => {
                        let (_, group) = many0(EffectShader::parse)(data)?;
                        Ok((i, TopGroup::EFSH(Group { header, data: group })))
                    }
                    b"ENCH" => {
                        let (_, group) = many0(ObjectEffect::parse)(data)?;
                        Ok((i, TopGroup::ENCH(Group { header, data: group })))
                    }
                    b"EQUP" => {
                        let (_, group) = many0(EquipType::parse)(data)?;
                        Ok((i, TopGroup::EQUP(Group { header, data: group })))
                    }
                    b"EXPL" => {
                        let (_, group) = many0(Explosion::parse)(data)?;
                        Ok((i, TopGroup::EXPL(Group { header, data: group })))
                    }
                    b"FACT" => {
                        let (_, group) = many0(Faction::parse)(data)?;
                        Ok((i, TopGroup::FACT(Group { header, data: group })))
                    }
                    b"FLOR" => {
                        let (_, group) = many0(Flora::parse)(data)?;
                        Ok((i, TopGroup::FLOR(Group { header, data: group })))
                    }
                    b"FLST" => {
                        let (_, group) = many0(FormIdList::parse)(data)?;
                        Ok((i, TopGroup::FLST(Group { header, data: group })))
                    }
                    b"FSTP" => {
                        let (_, group) = many0(Footstep::parse)(data)?;
                        Ok((i, TopGroup::FSTP(Group { header, data: group })))
                    }
                    b"FSTS" => {
                        let (_, group) = many0(FootstepSet::parse)(data)?;
                        Ok((i, TopGroup::FSTS(Group { header, data: group })))
                    }
                    b"FURN" => {
                        let (_, group) = many0(Furniture::parse)(data)?;
                        Ok((i, TopGroup::FURN(Group { header, data: group })))
                    }
                    b"GMST" => {
                        let (_, group) = many0(GameSetting::parse)(data)?;
                        Ok((i, TopGroup::GMST(Group { header, data: group })))
                    }
                    b"GDRY" => {
                        let (_, group) = many0(Godray::parse)(data)?;
                        Ok((i, TopGroup::GDRY(Group { header, data: group })))
                    }
                    b"GLOB" => {
                        let (_, group) = many0(Global::parse)(data)?;
                        Ok((i, TopGroup::GLOB(Group { header, data: group })))
                    }
                    b"GRAS" => {
                        let (_, group) = many0(Grass::parse)(data)?;
                        Ok((i, TopGroup::GRAS(Group { header, data: group })))
                    }
                    b"HAZD" => {
                        let (_, group) = many0(Hazard::parse)(data)?;
                        Ok((i, TopGroup::HAZD(Group { header, data: group })))
                    }
                    b"HDPT" => {
                        let (_, group) = many0(HeadPart::parse)(data)?;
                        Ok((i, TopGroup::HDPT(Group { header, data: group })))
                    }
                    b"IDLE" => {
                        let (_, group) = many0(IdleAnimation::parse)(data)?;
                        Ok((i, TopGroup::IDLE(Group { header, data: group })))
                    }
                    b"IDLM" => {
                        let (_, group) = many0(IdleMarker::parse)(data)?;
                        Ok((i, TopGroup::IDLM(Group { header, data: group })))
                    }
                    b"IMAD" => {
                        let (_, group) = many0(ImageSpaceAdapter::parse)(data)?;
                        Ok((i, TopGroup::IMAD(Group { header, data: group })))
                    }
                    b"IMGS" => {
                        let (_, group) = many0(ImageSpace::parse)(data)?;
                        Ok((i, TopGroup::IMGS(Group { header, data: group })))
                    }
                    b"INGR" => {
                        let (_, group) = many0(Ingredient::parse)(data)?;
                        Ok((i, TopGroup::INGR(Group { header, data: group })))
                    }
                    b"INNR" => {
                        let (_, group) = many0(InstanceNamingRules::parse)(data)?;
                        Ok((i, TopGroup::INNR(Group { header, data: group })))
                    }
                    b"IPCT" => {
                        let (_, group) = many0(Impact::parse)(data)?;
                        Ok((i, TopGroup::IPCT(Group { header, data: group })))
                    }
                    b"IPDS" => {
                        let (_, group) = many0(ImpactDataSet::parse)(data)?;
                        Ok((i, TopGroup::IPDS(Group { header, data: group })))
                    }
                    b"KEYM" => {
                        let (_, group) = many0(Key::parse)(data)?;
                        Ok((i, TopGroup::KEYM(Group { header, data: group })))
                    }
                    b"KYWD" => {
                        let (_, group) = many0(Keyword::parse)(data)?;
                        Ok((i, TopGroup::KYWD(Group { header, data: group })))
                    }
                    b"KSSM" => {
                        let (_, group) = many0(KeywordSoundMapping::parse)(data)?;
                        Ok((i, TopGroup::KSSM(Group { header, data: group })))
                    }
                    b"LAYR" => {
                        let (_, group) = many0(Layer::parse)(data)?;
                        Ok((i, TopGroup::LAYR(Group { header, data: group })))
                    }
                    b"LCRT" => {
                        let (_, group) = many0(LocationReferenceType::parse)(data)?;
                        Ok((i, TopGroup::LCRT(Group { header, data: group })))
                    }
                    b"LCTN" => {
                        let (_, group) = many0(Location::parse)(data)?;
                        Ok((i, TopGroup::LCTN(Group { header, data: group })))
                    }
                    b"LENS" => {
                        let (_, group) = many0(LensFlare::parse)(data)?;
                        Ok((i, TopGroup::LENS(Group { header, data: group })))
                    }
                    b"LGTM" => {
                        let (_, group) = many0(LightingTemplate::parse)(data)?;
                        Ok((i, TopGroup::LGTM(Group { header, data: group })))
                    }
                    b"LIGH" => {
                        let (_, group) = many0(Light::parse)(data)?;
                        Ok((i, TopGroup::LIGH(Group { header, data: group })))
                    }
                    b"LSCR" => {
                        let (_, group) = many0(LoadingScreen::parse)(data)?;
                        Ok((i, TopGroup::LSCR(Group { header, data: group })))
                    }
                    b"LTEX" => {
                        let (_, group) = many0(LandscapeTexture::parse)(data)?;
                        Ok((i, TopGroup::LTEX(Group { header, data: group })))
                    }
                    b"LVLI" => {
                        let (_, group) = many0(LeveledItem::parse)(data)?;
                        Ok((i, TopGroup::LVLI(Group { header, data: group })))
                    }
                    b"LVLN" => {
                        let (_, group) = many0(LeveledNPC::parse)(data)?;
                        Ok((i, TopGroup::LVLN(Group { header, data: group })))
                    }
                    b"MATO" => {
                        let (_, group) = many0(MaterialObject::parse)(data)?;
                        Ok((i, TopGroup::MATO(Group { header, data: group })))
                    }
                    b"MATT" => {
                        let (_, group) = many0(MaterialType::parse)(data)?;
                        Ok((i, TopGroup::MATT(Group { header, data: group })))
                    }
                    b"MESG" => {
                        let (_, group) = many0(Message::parse)(data)?;
                        Ok((i, TopGroup::MESG(Group { header, data: group })))
                    }
                    b"MGEF" => {
                        let (_, group) = many0(MagicEffect::parse)(data)?;
                        Ok((i, TopGroup::MGEF(Group { header, data: group })))
                    }
                    b"MISC" => {
                        let (_, group) = many0(MiscItem::parse)(data)?;
                        Ok((i, TopGroup::MISC(Group { header, data: group })))
                    }
                    b"MOVT" => {
                        let (_, group) = many0(MovementType::parse)(data)?;
                        Ok((i, TopGroup::MOVT(Group { header, data: group })))
                    }
                    b"MSTT" => {
                        let (_, group) = many0(MoveableStatic::parse)(data)?;
                        Ok((i, TopGroup::MSTT(Group { header, data: group } )))
                    }
                    b"MSWP" => {
                        let (_, group) = many0(MaterialSwap::parse)(data)?;
                        Ok((i, TopGroup::MSWP(Group { header, data: group })))
                    }
                    b"MUSC" => {
                        let (_, group) = many0(MusicType::parse)(data)?;
                        Ok((i, TopGroup::MUSC(Group { header, data: group })))
                    }
                    b"MUST" => {
                        let (_, group) = many0(MusicTrack::parse)(data)?;
                        Ok((i, TopGroup::MUST(Group { header, data: group })))
                    }
                    b"NAVI" => {
                        let (_, group) = many0(NavigationMeshInfoMap::parse)(data)?;
                        Ok((i, TopGroup::NAVI(Group { header, data: group })))
                    }
                    b"NOCM" => {
                        let (_, group) = many0(NavObstacleManager::parse)(data)?; 
                        Ok((i, TopGroup::NOCM(Group { header, data: group })))
                    }
                    b"NOTE" => {
                        let (_, group) = many0(Note::parse)(data)?;
                        Ok((i, TopGroup::NOTE(Group { header, data: group })))
                    }
                    b"NPC_" => {
                        let (_, group) = many0(NonPlayerCharacter::parse)(data)?;
                        Ok((i, TopGroup::NPC_(Group { header, data: group })))
                    }
                    // b"NPC_" => {
                    //     let (i, (header, raw)) = alloc_group(i)?;
                    //     Ok((i, TopGroup::NPC_(Group { header, data: Vec::new()})))
                    // }
                    b"OMOD" => {
                        let (_, group) = many0(ObjectModification::parse)(data)?;
                        Ok((i, TopGroup::OMOD(Group { header, data: group })))
                    }
                    b"OTFT" => {
                        let (_, group) = many0(Outfit::parse)(data)?;
                        Ok((i, TopGroup::OTFT(Group { header, data: group })))
                    }
                    b"OVIS" => {
                        let (_, group) = many0(ObjectVisibility::parse)(data)?;
                        Ok((i, TopGroup::OVIS(Group { header, data: group })))
                    }
                    b"PACK" => {
                        let (_, group) = many0(Package::parse)(data)?;
                        Ok((i, TopGroup::PACK(Group { header, data: group })))
                    }
                    b"PERK" => {
                        let (_, group) = many0(Perk::parse)(data)?;
                        Ok((i, TopGroup::PERK(Group { header, data: group })))
                    }
                    b"PKIN" => {
                        let (_, group) = many0(PackIn::parse)(data)?;
                        Ok((i, TopGroup::PKIN(Group { header, data: group })))
                    }
                    b"PROJ" => {
                        let (_, group) = many0(Projectile::parse)(data)?;
                        Ok((i, TopGroup::PROJ(Group { header, data: group })))
                    }
                    b"QUST" => {
                        let (_, (header, raw)) = alloc_group(i)?;
                        Ok((i, TopGroup::QUST(Group { header, data: Vec::new()})))
                    }
                    b"RACE" => {
                        let (_, group) = many0(Race::parse)(data)?;
                        Ok((i, TopGroup::RACE(Group { header, data: group })))
                    }
                    b"REGN" => {
                        let (_, group) = many0(Region::parse)(data)?;
                        Ok((i, TopGroup::REGN(Group { header, data: group })))
                    }
                    b"RELA" => {
                        let (_, group) = many0(Relationship::parse)(data)?;
                        Ok((i, TopGroup::RELA(Group { header, data: group })))
                    }
                    b"REVB" => {
                        let (_, group) = many0(Reverb::parse)(data)?;
                        Ok((i, TopGroup::REVB(Group { header, data: group })))
                    }
                    b"RFCT" => {
                        let (_, group) = many0(VisualEffect::parse)(data)?;
                        Ok((i, TopGroup::RFCT(Group { header, data: group })))
                    }
                    b"RFGP" => {
                        let (_, group) = many0(ReferenceGroup::parse)(data)?;
                        Ok((i, TopGroup::RFGP(Group { header, data: group })))
                    }
                    b"SCCO" => {
                        let (_, group) = many0(SceneCollection::parse)(data)?;
                        Ok((i, TopGroup::SCCO(Group { header, data: group })))
                    }
                    b"SCOL" => {
                        let (_, group) = many0(StaticCollection::parse)(data)?;
                        Ok((i, TopGroup::SCOL(Group { header, data: group })))
                    }
                    b"SCSN" => {
                        let (_, group) = many0(AudioCategorySnapshot::parse)(data)?;
                        Ok((i, TopGroup::SCSN(Group { header, data: group })))
                    }
                    b"SMBN" => {
                        let (_, group) = many0(StoryManagerBranchNode::parse)(data)?;
                        Ok((i, TopGroup::SMBN(Group { header, data: group })))
                    }
                    b"SMEN" => {
                        let (_, group) = many0(StoryManagerEventNode::parse)(data)?;
                        Ok((i, TopGroup::SMEN(Group { header, data: group })))
                    }
                    b"SMQN" => {
                        let (_, group) = many0(StoryManagerQuestNode::parse)(data)?;
                        Ok((i, TopGroup::SMQN(Group { header, data: group })))
                    }
                    b"SNCT" => {
                        let (_, group) = many0(SoundCategory::parse)(data)?;
                        Ok((i, TopGroup::SNCT(Group { header, data: group })))
                    }
                    b"SNDR" => {
                        let (_, group) = many0(SoundDescriptor::parse)(data)?;
                        Ok((i, TopGroup::SNDR(Group { header, data: group })))
                    }
                    b"SOPM" => {
                        let (_, group) = many0(SoundOutputModel::parse)(data)?;
                        Ok((i, TopGroup::SOPM(Group { header, data: group })))
                    }
                    b"SOUN" => {
                        let (_, group) = many0(SoundMarker::parse)(data)?;
                        Ok((i, TopGroup::SOUN(Group { header, data: group })))
                    }
                    b"SPEL" => {
                        let (_, group) = many0(Spell::parse)(data)?;
                        Ok((i, TopGroup::SPEL(Group { header, data: group })))
                    }
                    b"SPGD" => {
                        let (_, group) = many0(ShaderParticleGeometry::parse)(data)?;
                        Ok((i, TopGroup::SPGD(Group { header, data: group })))
                    }
                    b"STAG" => {
                        let (_, group) = many0(SoundTag::parse)(data)?;
                        Ok((i, TopGroup::STAG(Group { header, data: group })))
                    }
                    b"STAT" => {
                        let (_, group) = many0(Static::parse)(data)?;
                        Ok((i, TopGroup::STAT(Group { header, data: group })))
                    }
                    b"TACT" => {
                        let (_, group) = many0(TalkingActivator::parse)(data)?;
                        Ok((i, TopGroup::TACT(Group { header, data: group })))
                    }
                    b"TERM" => {
                        let (_, group) = many0(Terminal::parse)(data)?;
                        Ok((i, TopGroup::TERM(Group { header, data: group })))
                    }
                    b"TREE" => {
                        let (_, group) = many0(Tree::parse)(data)?;
                        Ok((i, TopGroup::TREE(Group { header, data: group })))
                    }
                    b"TRNS" => {
                        let (_, group) = many0(Transform::parse)(data)?;
                        Ok((i, TopGroup::TRNS(Group { header, data: group })))
                    }
                    b"TXST" => {
                        let (_, group) = many0(TextureSet::parse)(data)?;
                        Ok((i, TopGroup::TXST(Group { header, data: group })))
                    }
                    b"VTYP" => {
                        let (_, group) = many0(VoiceType::parse)(data)?;
                        Ok((i, TopGroup::VTYP(Group { header, data: group })))
                    }
                    b"WATR" => {
                        let (_, group) = many0(Water::parse)(data)?;
                        Ok((i, TopGroup::WATR(Group { header, data: group })))
                    }
                    b"WEAP" => {
                        let (_, group) = many0(Weapon::parse)(data)?;
                        Ok((i, TopGroup::WEAP(Group { header, data: group })))
                    }
                    b"WRLD" => {
                        let (_, we) = many0(WorldEntry::parse)(data)?;
                        Ok((i, TopGroup::WRLD(Group { header, data: we })))
                    }
                    b"WTHR" => {
                        let (_, group) = many0(Weather::parse)(data)?;
                        Ok((i, TopGroup::WTHR(Group { header, data: group })))
                    }
                    b"ZOOM" => {
                        let (_, group) = many0(Zoom::parse)(data)?;
                        Ok((i, TopGroup::ZOOM(Group { header, data: group })))
                    }

                    _ => {
                        unimplemented!("Top group {} not implemented", label);
                    }
                }
            }
            _ => {
                return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Tag)));
            }
        }



    }
}