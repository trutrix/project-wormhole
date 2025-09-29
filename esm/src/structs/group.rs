use std::fmt::Debug;

use crate::records::all::*;
use crate::dev::*;
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
        let (_, records) = many0(complete(RawRecord::parse))(data)?;
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
        let (_, records) = many0(complete(T::parse))(data)?;
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
        let (_, cells) = many0(complete(RawCellRecord::parse))(data)?;
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
        let (_, worlds) = many0(complete(RawWorldRecord::parse))(data)?;
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
        let (_, quests) = many0(complete(RawQuestRecord::parse))(data)?;
        Ok((i, Self { header, quests }))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub enum TopGroup {
    GameSettings(Group<GameSetting>),
    Keywords(Group<Keyword>),
    LCRT(LCRT),
    AACT(Group<Action>),
    TRNS(TRNS),
    CMPO(CMPO),
    TXST(TXST),
    GLOB(GLOB),
    DMGT(DMGT),
    CLAS(CLAS),
    FACT(FACT),
    HDPT(HDPT),
    RACE(RACE),
    SOUN(SOUN),
    ASPC(ASPC),
    MGEF(MGEF),
    LTEX(LTEX),
    ENCH(ENCH),
    SPEL(SPEL),
    ACTI(Group<Activator>),
    TACT(TACT),
    ARMO(ARMO),
    BOOK(BOOK),
    CONT(CONT),
    DOOR(DOOR),
    INGR(INGR),
    LIGH(LIGH),
    MISC(MISC),
    STAT(STAT),
    SCOL(SCOL),
    MSTT(MSTT),
    GRAS(GRAS),
    TREE(TREE),
    FLOR(FLOR),
    FURN(FURN),
    WEAP(WEAP),
    AMMO(Group<Ammo>),
    NPC_(NPC_),
    LVLN(LVLN),
    KEYM(KEYM),
    ALCH(Group<Alchemy>),
    IDLM(IDLM),
    NOTE(NOTE),
    PROJ(PROJ),
    HAZD(HAZD),
    BNDS(BNDS),
    TERM(TERM),
    LVLI(LVLI),
    WTHR(WTHR),
    CLMT(CLMT),
    SPGD(SPGD),
    RFCT(RFCT),
    REGN(REGN),
    NAVI(NAVI),
    CELL(Group<Cell>),
    WRLD(Group<Worldspace>),
    QUST(QUST),
    IDLE(IDLE),
    PACK(PACK),
    CSTY(CSTY),
    LSCR(LSCR),
    ANIO(Group<AnimatedObject>),
    WATR(WATR),
    EFSH(EFSH),
    EXPL(EXPL),
    DEBR(DEBR),
    IMGS(IMGS),
    IMAD(IMAD),
    FLST(FLST),
    PERK(PERK),
    BPTD(BPTD),
    ADDN(Group<Addon>),
    AVIF(AVIF),
    CAMS(CAMS),
    CPTH(CPTH),
    VTYP(VTYP),
    MATT(MATT),
    IPCT(IPCT),
    IPDS(IPDS),
    ARMA(ARMA),
    ECZN(ECZN),
    LCTN(LCTN),
    MESG(MESG),
    DOBJ(DOBJ),
    DFOB(DFOB),
    LGTM(LGTM),
    MUSC(MUSC),
    FSTP(FSTP),
    FSTS(FSTS),
    SMBN(SMBN),
    SMQN(SMQN),
    SMEN(SMEN),
    MUST(MUST),
    DLVW(DLVW),
    EQUP(EQUP),
    RELA(RELA),
    ASTP(ASTP),
    OTFT(OTFT),
    ARTO(ARTO),
    MATO(MATO),
    MOVT(MOVT),
    SNDR(SNDR),
    SNCT(SNCT),
    SOPM(SOPM),
    COLL(COLL),
    CLFM(CLFM),
    REVB(REVB),
    PKIN(PKIN),
    RFGP(RFGP),
    AMDL(AMDL),
    LAYR(LAYR),
    COBJ(COBJ),
    OMOD(OMOD),
    MSWP(MSWP),
    ZOOM(ZOOM),
    INNR(INNR),
    KSSM(KSSM),
    AECH(AECH),
    SCCO(SCCO),
    AORU(AORU),
    SCSN(SCSN),
    STAG(STAG),
    NOCM(NOCM),
    LENS(LENS),
    GDRY(GDRY),
    OVIS(OVIS),
    Unhandled    
}

impl Parse<&[u8]> for TopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;

        match header.label {
            GroupLabel::Top(label) => {
                match &label.0 {
                    b"GMST" => {
                        let (_, records) = many0(complete(GameSetting::parse))(data)?;
                        Ok((i, TopGroup::GameSettings(Group { header, data: records })))
                    }
                    b"KYWD" => {
                        let (_, records) = many0(complete(Keyword::parse))(data)?;
                        Ok((i, TopGroup::Keywords(Group { header, data: records })))
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