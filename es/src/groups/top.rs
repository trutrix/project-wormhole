use crate::dev::*;
use crate::groups::prelude::{InteriorCellBlock, RawInteriorCellBlock};
use crate::records::all::*;

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
    CELL(Group<InteriorCellBlock>), // Does not contain top level data records
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
    QUST(Group<Quest>),
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
    WRLD(Group<Worldspace>),
    WTHR(WeatherGroup),
    ZOOM(ZoomGroup),
}

// ====================================================================================================

impl Parse<&[u8]> for TopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;

        if header.size == 0 {
            return Ok((i, TopGroup::Empty(Group { header, data: Vec::new() })));
        }

        Self::parse_pre_alloc(raw, header)
    }
}

// ====================================================================================================

impl TopGroup {
    pub fn parse_pre_alloc(raw: &[u8], header: GroupHeader) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        if let GroupLabel::Top(iden) = header.label {
            match &iden.0 {
                b"AACT" => { Ok((&[], TopGroup::AACT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ACTI" => { Ok((&[], TopGroup::ACTI(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ADDN" => { Ok((&[], TopGroup::ADDN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"AECH" => { Ok((&[], TopGroup::AECH(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ALCH" => { Ok((&[], TopGroup::ALCH(Group::parse_pre_alloc(raw, header)?.1))) }
                b"AMDL" => { Ok((&[], TopGroup::AMDL(Group::parse_pre_alloc(raw, header)?.1))) }
                b"AMMO" => { Ok((&[], TopGroup::AMMO(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ANIO" => { Ok((&[], TopGroup::ANIO(Group::parse_pre_alloc(raw, header)?.1))) }
                b"AORU" => { Ok((&[], TopGroup::AORU(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ARMA" => { Ok((&[], TopGroup::ARMA(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ARMO" => { Ok((&[], TopGroup::ARMO(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ARTO" => { Ok((&[], TopGroup::ARTO(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ASPC" => { Ok((&[], TopGroup::ASPC(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ASTP" => { Ok((&[], TopGroup::ASTP(Group::parse_pre_alloc(raw, header)?.1))) }
                b"AVIF" => { Ok((&[], TopGroup::AVIF(Group::parse_pre_alloc(raw, header)?.1))) }
                b"BOOK" => { Ok((&[], TopGroup::BOOK(Group::parse_pre_alloc(raw, header)?.1))) }
                b"BPTD" => { Ok((&[], TopGroup::BPTD(Group::parse_pre_alloc(raw, header)?.1))) }
                b"BNDS" => { Ok((&[], TopGroup::BNDS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CAMS" => { Ok((&[], TopGroup::CAMS(Group::parse_pre_alloc(raw, header)?.1))) }
                //b"CELL" => { Ok((&[], TopGroup::CELL(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CLAS" => { Ok((&[], TopGroup::CLAS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CLFM" => { Ok((&[], TopGroup::CLFM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CLMT" => { Ok((&[], TopGroup::CLMT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CMPO" => { Ok((&[], TopGroup::CMPO(Group::parse_pre_alloc(raw, header)?.1))) }
                b"COBJ" => { Ok((&[], TopGroup::COBJ(Group::parse_pre_alloc(raw, header)?.1))) }
                b"COLL" => { Ok((&[], TopGroup::COLL(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CONT" => { Ok((&[], TopGroup::CONT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CPTH" => { Ok((&[], TopGroup::CPTH(Group::parse_pre_alloc(raw, header)?.1))) }
                b"CSTY" => { Ok((&[], TopGroup::CSTY(Group::parse_pre_alloc(raw, header)?.1))) }
                b"DEBR" => { Ok((&[], TopGroup::DEBR(Group::parse_pre_alloc(raw, header)?.1))) }   
                b"DFOB" => { Ok((&[], TopGroup::DFOB(Group::parse_pre_alloc(raw, header)?.1))) }
                b"DLVW" => { Ok((&[], TopGroup::DLVW(Group::parse_pre_alloc(raw, header)?.1))) }
                b"DMGT" => { Ok((&[], TopGroup::DMGT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"DOBJ" => { Ok((&[], TopGroup::DOBJ(Group::parse_pre_alloc(raw, header)?.1))) }
                b"DOOR" => { Ok((&[], TopGroup::DOOR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ECZN" => { Ok((&[], TopGroup::ECZN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"EFSH" => { Ok((&[], TopGroup::EFSH(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ENCH" => { Ok((&[], TopGroup::ENCH(Group::parse_pre_alloc(raw, header)?.1))) }
                b"EQUP" => { Ok((&[], TopGroup::EQUP(Group::parse_pre_alloc(raw, header)?.1))) }
                b"EXPL" => { Ok((&[], TopGroup::EXPL(Group::parse_pre_alloc(raw, header)?.1))) }
                b"FACT" => { Ok((&[], TopGroup::FACT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"FLOR" => { Ok((&[], TopGroup::FLOR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"FLST" => { Ok((&[], TopGroup::FLST(Group::parse_pre_alloc(raw, header)?.1))) }
                b"FSTP" => { Ok((&[], TopGroup::FSTP(Group::parse_pre_alloc(raw, header)?.1))) }
                b"FSTS" => { Ok((&[], TopGroup::FSTS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"FURN" => { Ok((&[], TopGroup::FURN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"GMST" => { Ok((&[], TopGroup::GMST(Group::parse_pre_alloc(raw, header)?.1))) }
                b"GDRY" => { Ok((&[], TopGroup::GDRY(Group::parse_pre_alloc(raw, header)?.1))) }
                b"GLOB" => { Ok((&[], TopGroup::GLOB(Group::parse_pre_alloc(raw, header)?.1))) }
                b"GRAS" => { Ok((&[], TopGroup::GRAS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"HAZD" => { Ok((&[], TopGroup::HAZD(Group::parse_pre_alloc(raw, header)?.1))) }
                b"HDPT" => { Ok((&[], TopGroup::HDPT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"IDLE" => { Ok((&[], TopGroup::IDLE(Group::parse_pre_alloc(raw, header)?.1))) }
                b"IDLM" => { Ok((&[], TopGroup::IDLM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"IMAD" => { Ok((&[], TopGroup::IMAD(Group::parse_pre_alloc(raw, header)?.1))) }
                b"IMGS" => { Ok((&[], TopGroup::IMGS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"INGR" => { Ok((&[], TopGroup::INGR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"INNR" => { Ok((&[], TopGroup::INNR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"IPCT" => { Ok((&[], TopGroup::IPCT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"IPDS" => { Ok((&[], TopGroup::IPDS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"KEYM" => { Ok((&[], TopGroup::KEYM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"KYWD" => { Ok((&[], TopGroup::KYWD(Group::parse_pre_alloc(raw, header)?.1))) }
                b"KSSM" => { Ok((&[], TopGroup::KSSM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LAYR" => { Ok((&[], TopGroup::LAYR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LCRT" => { Ok((&[], TopGroup::LCRT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LCTN" => { Ok((&[], TopGroup::LCTN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LENS" => { Ok((&[], TopGroup::LENS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LGTM" => { Ok((&[], TopGroup::LGTM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LIGH" => { Ok((&[], TopGroup::LIGH(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LSCR" => { Ok((&[], TopGroup::LSCR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LTEX" => { Ok((&[], TopGroup::LTEX(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LVLI" => { Ok((&[], TopGroup::LVLI(Group::parse_pre_alloc(raw, header)?.1))) }
                b"LVLN" => { Ok((&[], TopGroup::LVLN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MATO" => { Ok((&[], TopGroup::MATO(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MATT" => { Ok((&[], TopGroup::MATT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MESG" => { Ok((&[], TopGroup::MESG(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MGEF" => { Ok((&[], TopGroup::MGEF(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MISC" => { Ok((&[], TopGroup::MISC(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MOVT" => { Ok((&[], TopGroup::MOVT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MSTT" => { Ok((&[], TopGroup::MSTT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MSWP" => { Ok((&[], TopGroup::MSWP(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MUSC" => { Ok((&[], TopGroup::MUSC(Group::parse_pre_alloc(raw, header)?.1))) }
                b"MUST" => { Ok((&[], TopGroup::MUST(Group::parse_pre_alloc(raw, header)?.1))) }
                b"NAVI" => { Ok((&[], TopGroup::NAVI(Group::parse_pre_alloc(raw, header)?.1))) }
                b"NOCM" => { Ok((&[], TopGroup::NOCM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"NOTE" => { Ok((&[], TopGroup::NOTE(Group::parse_pre_alloc(raw, header)?.1))) }
                b"NPC_" => { Ok((&[], TopGroup::NPC_(Group::parse_pre_alloc(raw, header)?.1))) }
                b"OMOD" => { Ok((&[], TopGroup::OMOD(Group::parse_pre_alloc(raw, header)?.1))) }
                b"OTFT" => { Ok((&[], TopGroup::OTFT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"OVIS" => { Ok((&[], TopGroup::OVIS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"PACK" => { Ok((&[], TopGroup::PACK(Group::parse_pre_alloc(raw, header)?.1))) }
                b"PERK" => { Ok((&[], TopGroup::PERK(Group::parse_pre_alloc(raw, header)?.1))) }
                b"PKIN" => { Ok((&[], TopGroup::PKIN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"PROJ" => { Ok((&[], TopGroup::PROJ(Group::parse_pre_alloc(raw, header)?.1))) }
                //b"QUST" => { Ok((&[], TopGroup::QUST(Group::parse_pre_alloc(raw, header)?.1))) }
                b"RACE" => { Ok((&[], TopGroup::RACE(Group::parse_pre_alloc(raw, header)?.1))) }
                b"REGN" => { Ok((&[], TopGroup::REGN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"RELA" => { Ok((&[], TopGroup::RELA(Group::parse_pre_alloc(raw, header)?.1))) }
                b"REVB" => { Ok((&[], TopGroup::REVB(Group::parse_pre_alloc(raw, header)?.1))) }
                b"RFCT" => { Ok((&[], TopGroup::RFCT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"RFGP" => { Ok((&[], TopGroup::RFGP(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SCCO" => { Ok((&[], TopGroup::SCCO(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SCOL" => { Ok((&[], TopGroup::SCOL(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SCSN" => { Ok((&[], TopGroup::SCSN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SMBN" => { Ok((&[], TopGroup::SMBN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SMEN" => { Ok((&[], TopGroup::SMEN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SMQN" => { Ok((&[], TopGroup::SMQN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SNCT" => { Ok((&[], TopGroup::SNCT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SNDR" => { Ok((&[], TopGroup::SNDR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SOPM" => { Ok((&[], TopGroup::SOPM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SOUN" => { Ok((&[], TopGroup::SOUN(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SPEL" => { Ok((&[], TopGroup::SPEL(Group::parse_pre_alloc(raw, header)?.1))) }
                b"SPGD" => { Ok((&[], TopGroup::SPGD(Group::parse_pre_alloc(raw, header)?.1))) }
                b"STAG" => { Ok((&[], TopGroup::STAG(Group::parse_pre_alloc(raw, header)?.1))) }
                b"STAT" => { Ok((&[], TopGroup::STAT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"TACT" => { Ok((&[], TopGroup::TACT(Group::parse_pre_alloc(raw, header)?.1))) }
                b"TERM" => { Ok((&[], TopGroup::TERM(Group::parse_pre_alloc(raw, header)?.1))) }
                b"TREE" => { Ok((&[], TopGroup::TREE(Group::parse_pre_alloc(raw, header)?.1))) }
                b"TRNS" => { Ok((&[], TopGroup::TRNS(Group::parse_pre_alloc(raw, header)?.1))) }
                b"TXST" => { Ok((&[], TopGroup::TXST(Group::parse_pre_alloc(raw, header)?.1))) }
                b"VTYP" => { Ok((&[], TopGroup::VTYP(Group::parse_pre_alloc(raw, header)?.1))) }
                b"WATR" => { Ok((&[], TopGroup::WATR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"WEAP" => { Ok((&[], TopGroup::WEAP(Group::parse_pre_alloc(raw, header)?.1))) }
                //b"WRLD" => { Ok((&[], TopGroup::WRLD(Group::parse_with_header(raw, header)?.1))) }
                b"WTHR" => { Ok((&[], TopGroup::WTHR(Group::parse_pre_alloc(raw, header)?.1))) }
                b"ZOOM" => { Ok((&[], TopGroup::ZOOM(Group::parse_pre_alloc(raw, header)?.1))) }
                _ => {

                    #[cfg(debug_assertions)]
                    println!("Top group {} not implemented", header.label);

                    Ok((&[], TopGroup::Unhandled(Group { header, data: Vec::new() })))
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