use std::{collections::HashMap, fs::File, io::Read};

use crate::{dev::*, esm::{ESMError, full::ESMFull, raw::ESMRaw}, groups::prelude::TopGroup, prelude::{FormIdTrait, MapContents}, records::{SingleRecord, all::FileHeader}};


pub struct ESMMapped {
    pub header: FileHeader,
    pub indices: HashMap<FormId, SingleRecord>,
}

// ================================================================================

impl ESMMapped {
    pub fn load_file(file_path: &str) -> Result<Self, ESMError> where Self: Sized {
        let mut file = File::open(file_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        ESMMapped::parse(&buf)
    }

    pub fn parse(i: &[u8]) -> Result<Self, ESMError> where Self: Sized {
        let (_, esm) = ESMFull::parse_mt(i).map_err(|_| ESMError::InvalidGroup)?;
        Ok(ESMMapped::from(esm))
    }
}

// ================================================================================

impl From<ESMFull> for ESMMapped {
    fn from(value: ESMFull) -> Self {
        
        let header = value.header;
        let mut indices = HashMap::new();

        fn iter_insert_records<T: FormIdTrait + Into<SingleRecord>>(indices: &mut HashMap<FormId, SingleRecord>, records: Vec<T>) {
            for record in records {
                let form_id = *record.get_form_id();
                let sr: SingleRecord = record.into();
                indices.insert(form_id, sr);
            }
        }


        for group in value.groups {
            match group {
                TopGroup::Unhandled(group_vec) => { panic!("Unhandled group: {:?}", group_vec.header.label ) },
                TopGroup::AACT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ACTI(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ADDN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AECH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ALCH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AMDL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AMMO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ANIO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AORU(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ARMA(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ARMO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ARTO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ASPC(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ASTP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::AVIF(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::BNDS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::BOOK(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::BPTD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CAMS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                //TopGroup::CELL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CLAS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CLFM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CLMT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CMPO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::COBJ(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::COLL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CONT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CPTH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::CSTY(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DEBR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DFOB(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DLVW(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DMGT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DOBJ(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::DOOR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ECZN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::EFSH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ENCH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::EQUP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::EXPL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FACT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FLOR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FLST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FSTP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FSTS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::FURN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GMST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GDRY(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GLOB(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::GRAS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::HAZD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::HDPT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IDLE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IDLM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IMAD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IMGS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::INGR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::INNR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IPCT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::IPDS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::KEYM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::KYWD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::KSSM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LAYR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LCRT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LCTN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LENS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LGTM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LIGH(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LSCR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LTEX(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LVLI(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::LVLN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MATO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MATT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MESG(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MGEF(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MISC(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MOVT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MSTT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MSWP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MUSC(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::MUST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NAVI(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NOCM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NOTE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::NPC_(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::OMOD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::OTFT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::OVIS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PACK(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PERK(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PKIN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::PROJ(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                // TopGroup::QUST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RACE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::REGN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RELA(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::REVB(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RFCT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::RFGP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SCCO(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SCOL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SCSN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SMBN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SMEN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SMQN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SNCT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SNDR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SOPM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SOUN(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SPEL(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::SPGD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::STAG(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::STAT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TACT(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TERM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TREE(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TRNS(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::TXST(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::VTYP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::WATR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::WEAP(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                // TopGroup::WRLD(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::WTHR(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                TopGroup::ZOOM(group_vec) => iter_insert_records(&mut indices, group_vec.data),
                _=> {}
            }
        }

        Self { header, indices }
    }
}

// ====================================================================================================

pub struct MappedESM<T> {
    pub header: FileHeader,
    pub map: HashMap<FormId, T>
}

// ====================================================================================================

impl<'esm> From<ESMRaw<'esm>> for MappedESM<RawRecord<'esm>> {
    fn from(value: ESMRaw<'esm>) -> Self {
        let mut map = HashMap::new();
        for item in value.objects {
            item.insert_into_one_map(&mut map);
        }
        Self { header: value.header, map }
    }
}

// ====================================================================================================

impl<'esm> MappedESM<RawRecord<'esm>> {
    pub fn diff(&'esm self, esm: &'esm Self) -> (Vec<&'esm FormId>, Vec<&'esm FormId>, Vec<&'esm FormId>) {
        let mut updated = Vec::with_capacity(1000000);
        let mut unchanged = Vec::with_capacity(2000000);
        let mut addition = Vec::with_capacity(1000000);
        
        
        for (id, item) in &esm.map {
            if let Some(original) = self.map.get(&id) {
                if item == original {
                    unchanged.push(id);
                } else {
                    updated.push(id);
                }
            } else {
                addition.push(id);
            }
        }

        (updated, unchanged, addition)
    }
}