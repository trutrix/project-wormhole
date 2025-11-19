use crate::dev::*;
use bitflags::bitflags;

// TODO: verify correct parsing

#[derive(Debug, NomLE)]
pub struct DestructibleHeader {
    pub health: i32,
    pub dest_count: u8,
    pub flags: DestructibleHeaderFlags,
    _padding: [u8;2],
}


#[derive(Debug, NomLE)]
pub struct DestructibleStageData {
    pub health: u8,
    pub index: u8,
    pub model_damage_stage: u8,
    pub flags: DestructibleStageFlags,
    pub self_damage_per_second: i32,
    pub explosion: FormRef,
    pub debris: FormRef,
    pub debris_count: i32
}


#[derive(Debug, NomLE)]
pub struct DestructibleHeaderFlags(u8);

bitflags! {
    #[derive(Debug)]
    pub struct DestructibleStageFlags: u8 {
        const CAP_DAMAGE       = 0x01;
        const DISABLE          = 0x02;
        const DESTROY          = 0x04;
        const IGNORE_EXTERNAL  = 0x08;
        const BECOMES_DYNAMIC  = 0x10;
    }
}

impl Parse<&[u8]> for DestructibleStageFlags {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self> {
        let (i, raw) = nom::number::complete::le_u8(i)?;
        Ok((i, DestructibleStageFlags::from_bits_retain(raw)))
    }
}