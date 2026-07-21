use crate::dev::*;
use crate::groups::prelude::{InteriorCellBlock, RawInteriorCellBlock};
use crate::records::all::*;

// ====================================================================================================

#[derive(Debug)]
pub enum TopGroup {
    Unhandled(GroupOld<RawRecord<'static>>),
    Empty(GroupOld<RawRecord<'static>>),

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
    CELL(GroupOld<InteriorCellBlock>), // Does not contain top level data records
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
    WRLD(WorldspaceGroup),
    WTHR(WeatherGroup),
    ZOOM(ZoomGroup),
}

// ====================================================================================================

impl Parse<&[u8]> for TopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;

        if header.size == 0 {
            return Ok((i, TopGroup::Empty(GroupOld { header, data: Vec::new() })));
        }

        Self::parse_pre_alloc(raw, header)
    }
}

// ====================================================================================================

impl TopGroup {
    pub fn parse_pre_alloc(raw: &[u8], header: GroupHeader) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        if let GroupLabel::Top(iden) = header.label {
            match &iden.0 {
                b"AACT" => { Ok((&[], TopGroup::AACT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ACTI" => { Ok((&[], TopGroup::ACTI(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ADDN" => { Ok((&[], TopGroup::ADDN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"AECH" => { Ok((&[], TopGroup::AECH(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ALCH" => { Ok((&[], TopGroup::ALCH(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"AMDL" => { Ok((&[], TopGroup::AMDL(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"AMMO" => { Ok((&[], TopGroup::AMMO(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ANIO" => { Ok((&[], TopGroup::ANIO(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"AORU" => { Ok((&[], TopGroup::AORU(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ARMA" => { Ok((&[], TopGroup::ARMA(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ARMO" => { Ok((&[], TopGroup::ARMO(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ARTO" => { Ok((&[], TopGroup::ARTO(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ASPC" => { Ok((&[], TopGroup::ASPC(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ASTP" => { Ok((&[], TopGroup::ASTP(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"AVIF" => { Ok((&[], TopGroup::AVIF(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"BOOK" => { Ok((&[], TopGroup::BOOK(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"BPTD" => { Ok((&[], TopGroup::BPTD(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"BNDS" => { Ok((&[], TopGroup::BNDS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"CAMS" => { Ok((&[], TopGroup::CAMS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                //b"CELL" => { Ok((&[], TopGroup::CELL(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CLAS" => { Ok((&[], TopGroup::CLAS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"CLFM" => { Ok((&[], TopGroup::CLFM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"CLMT" => { Ok((&[], TopGroup::CLMT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"CMPO" => { Ok((&[], TopGroup::CMPO(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"COBJ" => { Ok((&[], TopGroup::COBJ(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"COLL" => { Ok((&[], TopGroup::COLL(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"CONT" => { Ok((&[], TopGroup::CONT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"CPTH" => { Ok((&[], TopGroup::CPTH(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"CSTY" => { Ok((&[], TopGroup::CSTY(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"DEBR" => { Ok((&[], TopGroup::DEBR(GroupOld::parse_pre_alloc(raw, header)?.1))) }   
                b"DFOB" => { Ok((&[], TopGroup::DFOB(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"DLVW" => { Ok((&[], TopGroup::DLVW(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"DMGT" => { Ok((&[], TopGroup::DMGT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"DOBJ" => { Ok((&[], TopGroup::DOBJ(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"DOOR" => { Ok((&[], TopGroup::DOOR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ECZN" => { Ok((&[], TopGroup::ECZN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"EFSH" => { Ok((&[], TopGroup::EFSH(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ENCH" => { Ok((&[], TopGroup::ENCH(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"EQUP" => { Ok((&[], TopGroup::EQUP(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"EXPL" => { Ok((&[], TopGroup::EXPL(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"FACT" => { Ok((&[], TopGroup::FACT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"FLOR" => { Ok((&[], TopGroup::FLOR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"FLST" => { Ok((&[], TopGroup::FLST(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"FSTP" => { Ok((&[], TopGroup::FSTP(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"FSTS" => { Ok((&[], TopGroup::FSTS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"FURN" => { Ok((&[], TopGroup::FURN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"GMST" => { Ok((&[], TopGroup::GMST(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"GDRY" => { Ok((&[], TopGroup::GDRY(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"GLOB" => { Ok((&[], TopGroup::GLOB(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"GRAS" => { Ok((&[], TopGroup::GRAS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"HAZD" => { Ok((&[], TopGroup::HAZD(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"HDPT" => { Ok((&[], TopGroup::HDPT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"IDLE" => { Ok((&[], TopGroup::IDLE(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"IDLM" => { Ok((&[], TopGroup::IDLM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"IMAD" => { Ok((&[], TopGroup::IMAD(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"IMGS" => { Ok((&[], TopGroup::IMGS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"INGR" => { Ok((&[], TopGroup::INGR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"INNR" => { Ok((&[], TopGroup::INNR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"IPCT" => { Ok((&[], TopGroup::IPCT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"IPDS" => { Ok((&[], TopGroup::IPDS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"KEYM" => { Ok((&[], TopGroup::KEYM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"KYWD" => { Ok((&[], TopGroup::KYWD(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"KSSM" => { Ok((&[], TopGroup::KSSM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LAYR" => { Ok((&[], TopGroup::LAYR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LCRT" => { Ok((&[], TopGroup::LCRT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LCTN" => { Ok((&[], TopGroup::LCTN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LENS" => { Ok((&[], TopGroup::LENS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LGTM" => { Ok((&[], TopGroup::LGTM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LIGH" => { Ok((&[], TopGroup::LIGH(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LSCR" => { Ok((&[], TopGroup::LSCR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LTEX" => { Ok((&[], TopGroup::LTEX(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LVLI" => { Ok((&[], TopGroup::LVLI(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"LVLN" => { Ok((&[], TopGroup::LVLN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MATO" => { Ok((&[], TopGroup::MATO(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MATT" => { Ok((&[], TopGroup::MATT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MESG" => { Ok((&[], TopGroup::MESG(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MGEF" => { Ok((&[], TopGroup::MGEF(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MISC" => { Ok((&[], TopGroup::MISC(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MOVT" => { Ok((&[], TopGroup::MOVT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MSTT" => { Ok((&[], TopGroup::MSTT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MSWP" => { Ok((&[], TopGroup::MSWP(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MUSC" => { Ok((&[], TopGroup::MUSC(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"MUST" => { Ok((&[], TopGroup::MUST(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"NAVI" => { Ok((&[], TopGroup::NAVI(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"NOCM" => { Ok((&[], TopGroup::NOCM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"NOTE" => { Ok((&[], TopGroup::NOTE(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"NPC_" => { Ok((&[], TopGroup::NPC_(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"OMOD" => { Ok((&[], TopGroup::OMOD(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"OTFT" => { Ok((&[], TopGroup::OTFT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"OVIS" => { Ok((&[], TopGroup::OVIS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"PACK" => { Ok((&[], TopGroup::PACK(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"PERK" => { Ok((&[], TopGroup::PERK(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"PKIN" => { Ok((&[], TopGroup::PKIN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"PROJ" => { Ok((&[], TopGroup::PROJ(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                //b"QUST" => { Ok((&[], TopGroup::QUST(Group::parse_pre_alloc(raw, header)?.1))) }
                b"RACE" => { Ok((&[], TopGroup::RACE(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"REGN" => { Ok((&[], TopGroup::REGN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"RELA" => { Ok((&[], TopGroup::RELA(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"REVB" => { Ok((&[], TopGroup::REVB(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"RFCT" => { Ok((&[], TopGroup::RFCT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"RFGP" => { Ok((&[], TopGroup::RFGP(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SCCO" => { Ok((&[], TopGroup::SCCO(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SCOL" => { Ok((&[], TopGroup::SCOL(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SCSN" => { Ok((&[], TopGroup::SCSN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SMBN" => { Ok((&[], TopGroup::SMBN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SMEN" => { Ok((&[], TopGroup::SMEN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SMQN" => { Ok((&[], TopGroup::SMQN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SNCT" => { Ok((&[], TopGroup::SNCT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SNDR" => { Ok((&[], TopGroup::SNDR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SOPM" => { Ok((&[], TopGroup::SOPM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SOUN" => { Ok((&[], TopGroup::SOUN(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SPEL" => { Ok((&[], TopGroup::SPEL(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"SPGD" => { Ok((&[], TopGroup::SPGD(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"STAG" => { Ok((&[], TopGroup::STAG(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"STAT" => { Ok((&[], TopGroup::STAT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"TACT" => { Ok((&[], TopGroup::TACT(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"TERM" => { Ok((&[], TopGroup::TERM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"TREE" => { Ok((&[], TopGroup::TREE(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"TRNS" => { Ok((&[], TopGroup::TRNS(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"TXST" => { Ok((&[], TopGroup::TXST(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"VTYP" => { Ok((&[], TopGroup::VTYP(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"WATR" => { Ok((&[], TopGroup::WATR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"WEAP" => { Ok((&[], TopGroup::WEAP(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                //b"WRLD" => { Ok((&[], TopGroup::WRLD(Group::parse_with_header(raw, header)?.1))) }
                b"WTHR" => { Ok((&[], TopGroup::WTHR(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                b"ZOOM" => { Ok((&[], TopGroup::ZOOM(GroupOld::parse_pre_alloc(raw, header)?.1))) }
                _ => {

                    #[cfg(debug_assertions)]
                    println!("Top group {} not implemented", header.label);

                    Ok((&[], TopGroup::Unhandled(GroupOld { header, data: Vec::new() })))
                }
            }
        } else {
            Err(nom::Err::Error(nom::error::Error::new(raw, nom::error::ErrorKind::Tag)))
        }
    }
}

// ====================================================================================================


#[derive(Debug)]
pub enum RawTopGroup<'esm> {
    Common(Vec<RawRecord<'esm>>),
    Quest(Vec<RawQuestItem<'esm>>),
    World(Vec<RawWorldRecord<'esm>>),
    Cell(Vec<RawInteriorCellBlock<'esm>>)
}

// ====================================================================================================

impl<'esm> Parse<&'esm[u8]> for RawTopGroup<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (gh, raw)) = alloc_group(i)?;

        if &gh.iden.0 != b"GRUP" {
            panic!("Encountered non-group while parsing RawTopGroup: {:?}", gh.iden);
        }

        

        if let GroupLabel::Top(group_iden) = gh.label {

            // #[cfg(debug_assertions)]
            // println!("Parsing TopGroup: {:?}", gh.label);

            match &group_iden.0 {
                b"CELL" => {
                    //let start = std::time::Instant::now();
                    let (_, cell_group) = many0(RawInteriorCellBlock::parse)(raw)?;
                    //println!("Cells parse time: {:?}", start.elapsed());
                    Ok((i, Self::Cell(cell_group)))
                }

                b"WRLD" => {
                    //let start = std::time::Instant::now();
                    let (_, world_group) = many0(RawWorldRecord::parse)(raw)?;
                    //println!("Worlds parse time: {:?}", start.elapsed());
                    Ok((i, Self::World(world_group)))
                }

                b"QUST" => {
                    //let start = std::time::Instant::now();
                    //println!(" Parsing QUST group...");
                    let (_, quest_group) = many0(RawQuestItem::parse)(raw)?;
                    //println!("Quests parse time: {:?}", start.elapsed());
                    Ok((i, Self::Quest(quest_group)))
                }

                _ => {
                    //let start = std::time::Instant::now();
                    let (_, common_group) = many0(RawRecord::parse)(raw)?;
                    //println!("Commons parse time: {:?}", start.elapsed());
                    Ok((i, Self::Common(common_group)))
                }
            }


        } else {
            panic!("Encountered non-top group while parsing RawTopGroup: {:?}", gh.label);
        }


    }
}