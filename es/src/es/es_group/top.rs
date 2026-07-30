use nom_derive::nom;

use crate::{es::{es_group::{ESGroupHeader, ESGroupLabel, ESGroupTraits}, es_object::ESObject, es_record::ESRecordHeader}, records::AACT, traits::ParseAllocated};

// ====================================================================================================

#[derive(Debug)]
pub enum ESTopTyped {
    Unhandled(ESGroupHeader),
    // AACT(ESGroup<ESRecord<AACT::Action>>),
    // ACTI(ActivatorGroup),
    // ADDN(AddonNodeGroup),
    // AECH(AudioEffectChainGroup),
    // ALCH(AlchemyGroup),
    // AMDL(AimModelGroup),
    // AMMO(AmmoGroup),
    // ANIO(AnimatedObjectGroup),
    // AORU(AttractionRuleGroup),
    // ARMA(ArmorAddonGroup),
    // ARMO(ArmorGroup),
    // ARTO(ArtObjectGroup),
    // ASPC(AcousticSpaceGroup),
    // ASTP(AssociationTypeGroup),
    // AVIF(ActorValueInformationGroup),
    // BNDS(BendableSplineGroup),
    // BOOK(BookGroup),
    // BPTD(BodyPartDataGroup),
    // CAMS(CameraShotGroup),
    // CELL(GroupOld<InteriorCellBlock>), // Does not contain top level data records
    // CLAS(ClassGroup),
    // CLFM(ColorGroup),
    // CLMT(ClimateGroup),
    // CMPO(ComponentGroup),
    // COBJ(ConstructibleObjectGroup),
    // COLL(CollisionLayerGroup),
    // CONT(ContainerGroup),
    // CPTH(CameraPathGroup),
    // CSTY(CombatStyleGroup),
    // DEBR(DebrisGroup),
    // DFOB(DefaultObjectGroup),
    // DLVW(DialogViewGroup),
    // DMGT(DamageTypeGroup),
    // DOBJ(DefaultObjectsGroup),
    // DOOR(DoorGroup),
    // ECZN(EncounterZoneGroup),
    // EFSH(EffectShaderGroup),
    // ENCH(ObjectEffectGroup),
    // EQUP(EquipTypeGroup),
    // EXPL(ExplosionGroup),
    // FACT(FactionGroup),
    // FLOR(FloraGroup),
    // FLST(FormIdListGroup),
    // FSTP(FootstepGroup),
    // FSTS(FootstepSetGroup),
    // FURN(FurnitureGroup),
    // GMST(GameSettingGroup),
    // GDRY(GodrayGroup),
    // GLOB(GlobalGroup),
    // GRAS(GrassGroup),
    // HAZD(HazardGroup),
    // HDPT(HeadPartGroup),
    // IDLE(IdleAnimationGroup),
    // IDLM(IdleMarkerGroup),
    // IMAD(ImageSpaceAdapterGroup),
    // IMGS(ImageSpaceGroup),
    // INGR(IngredientGroup),
    // INNR(InstanceNamingRulesGroup),
    // IPCT(ImpactGroup),
    // IPDS(ImpactDataSetGroup),
    // KEYM(KeyGroup),
    // KYWD(KeywordGroup),
    // KSSM(KeywordSoundMappingGroup),
    // LAYR(LayerGroup),
    // LCRT(LocationReferenceTypeGroup),
    // LCTN(LocationGroup),
    // LENS(LensFlareGroup),
    // LGTM(LightingTemplateGroup),
    // LIGH(LightGroup),
    // LSCR(LoadingScreenGroup),
    // LTEX(LandscapeTextureGroup),
    // LVLI(LeveledItemGroup),
    // LVLN(LeveledNPCGroup),
    // MATO(MaterialObjectGroup),
    // MATT(MaterialTypeGroup),
    // MESG(MessageGroup),
    // MGEF(MagicEffectGroup),
    // MISC(MiscItemGroup),
    // MOVT(MovementTypeGroup),
    // MSTT(MoveableStaticGroup),
    // MSWP(MaterialSwapGroup),
    // MUSC(MusicTypeGroup),
    // MUST(MusicTrackGroup),
    // NAVI(NavigationMeshInfoMapGroup),
    // NOCM(NavObstacleManagerGroup),
    // NOTE(NoteGroup),
    // NPC_(NonPlayerCharacterGroup),
    // OMOD(ObjectModificationGroup),
    // OTFT(OutfitGroup),
    // OVIS(ObjectVisibilityGroup),
    // PACK(PackageGroup),
    // PERK(PerkGroup),
    // PKIN(PackInGroup),
    // PROJ(ProjectileGroup),
    // QUST(QuestGroup),
    // RACE(RaceGroup),
    // REGN(RegionGroup),
    // RELA(RelationshipGroup),
    // REVB(ReverbGroup),
    // RFCT(VisualEffectGroup),
    // RFGP(ReferenceGroupGroup),
    // SCCO(SceneCollectionGroup),
    // SCOL(StaticCollectionGroup),
    // SCSN(AudioCategorySnapshotGroup),
    // SMBN(StoryManagerBranchNodeGroup),
    // SMEN(StoryManagerEventNodeGroup),
    // SMQN(StoryManagerQuestNodeGroup),
    // SNCT(SoundCategoryGroup),
    // SNDR(SoundDescriptorGroup),
    // SOPM(SoundOutputModelGroup),
    // SOUN(SoundMarkerGroup),
    // SPEL(SpellGroup),
    // SPGD(ShaderParticleGeometryGroup),
    // STAG(SoundTagGroup),
    // STAT(StaticGroup),
    // TACT(TalkingActivatorGroup),
    // TERM(TerminalGroup),
    // TREE(TreeGroup),
    // TRNS(TransformGroup),
    // TXST(TextureSetGroup),
    // VTYP(VoiceTypeGroup),
    // WATR(WaterGroup),
    // WEAP(WeaponGroup),
    // WRLD(WorldspaceGroup),
    // WTHR(WeatherGroup),
    // ZOOM(ZoomGroup)
}

// ====================================================================================================

impl ParseAllocated<ESGroupHeader, &[u8]> for ESTopTyped {
    fn parse_allocated(header: ESGroupHeader, raw: &[u8]) -> Result<Self, nom_derive::nom::error::Error<&[u8]>> {
        match &header.label_value {
            
            _ => {
                Ok(ESTopTyped::Unhandled(header))
            }
        }
    }
}

// ====================================================================================================

impl ESObject for ESTopTyped {
    fn object_size(&self) -> &u32 {
        match self {
            ESTopTyped::Unhandled(esgroup_header) => &esgroup_header.size,
        }
    }
    
    fn object_count(&self) -> &usize {
        &1usize
    }

    fn try_get_form_id(&self) -> Option<&crate::dev::FormId> {
        None
    }

    fn is_group(&self) -> bool {
        true
    }
}

// ====================================================================================================

impl ESGroupTraits for ESTopTyped {
    fn get_header(&self) -> &ESGroupHeader {
        match self {
            ESTopTyped::Unhandled(g) => g,
        }
    }
}

// ====================================================================================================

pub struct ESTopG<T> {
    pub header: ESGroupHeader,
    pub data: T
}

// ====================================================================================================

impl<'es, T> ParseAllocated<ESGroupHeader, &'es[u8]> for ESTopG<T> where T: nom_derive::Parse<&'es[u8]> {
    fn parse_allocated(header: ESGroupHeader, raw: &'es[u8]) -> Result<Self, nom_derive::nom::error::Error<&'es[u8]>> {
        if let Ok((_, data)) = T::parse(raw) {
            Ok(Self { header, data })
        } else {
            Err(nom::error::Error::new(raw, nom::error::ErrorKind::Fail))
        }
        
    }
}