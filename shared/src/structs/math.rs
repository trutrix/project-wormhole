
use glam::*;
use nom_derive::nom::IResult;
use nom_derive::Parse;
use nom_derive::nom::multi::count;
use nom_derive::nom::number::complete::{le_f32, le_u16};

// ================================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSVec2(pub Vec2);

impl Parse<&[u8]> for BSVec2 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = le_f32(i)?;
        let (i, y) = le_f32(i)?;
        Ok((i, BSVec2(Vec2::new(x, y))))
    }
}

pub fn parse_vec2(i: &[u8]) -> IResult<&[u8], Vec2> {
    let (i, x) = le_f32(i)?;
    let (i, y) = le_f32(i)?;
    Ok((i, Vec2::new(x, y)))
}

// ================================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSVec3(pub Vec3);

impl Parse<&[u8]> for BSVec3 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = le_f32(i)?;
        let (i, y) = le_f32(i)?;
        let (i, z) = le_f32(i)?;
        Ok((i, BSVec3(Vec3::new(x, y, z))))
    }
}

pub fn parse_vec3(i: &[u8]) -> IResult<&[u8], Vec3> {
    let (i, x) = le_f32(i)?;
    let (i, y) = le_f32(i)?;
    let (i, z) = le_f32(i)?;
    Ok((i, Vec3::new(x, y, z)))
}

// ================================================================================


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSVec4(pub Vec4);

impl Parse<&[u8]> for BSVec4 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = le_f32(i)?;
        let (i, y) = le_f32(i)?;
        let (i, z) = le_f32(i)?;
        let (i, w) = le_f32(i)?;
        Ok((i, BSVec4(Vec4::new(x, y, z, w))))
    }
}

pub fn parse_vec4(i: &[u8]) -> IResult<&[u8], Vec4> {
    let (i, x) = le_f32(i)?;
    let (i, y) = le_f32(i)?;
    let (i, z) = le_f32(i)?;
    let (i, w) = le_f32(i)?;
    Ok((i, Vec4::new(x, y, z, w)))
}

// ================================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSQuat(pub Quat);

impl Parse<&[u8]> for BSQuat {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = le_f32(i)?;
        let (i, y) = le_f32(i)?;
        let (i, z) = le_f32(i)?;
        let (i, w) = le_f32(i)?;
        Ok((i, BSQuat(Quat::from_xyzw(x, y, z, w))))
    }
}

pub fn parse_quat(i: &[u8]) -> IResult<&[u8], Quat> {
    let (i, x) = le_f32(i)?;
    let (i, y) = le_f32(i)?;
    let (i, z) = le_f32(i)?;
    let (i, w) = le_f32(i)?;
    Ok((i, Quat::from_xyzw(x, y, z, w)))
}

// ================================================================================

/// These are stored in row-major order in files
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSMatrix3(pub Mat3);

impl Parse<&[u8]> for BSMatrix3 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, result) = parse_mat3(i)?;
        Ok((i, BSMatrix3(result)))
    }
}

/// These are stored in row-major order in files
pub fn parse_mat3(i: &[u8]) -> IResult<&[u8], Mat3> {
    let (i, floats) = count(le_f32, 9)(i)?;
    Ok((i, Mat3::from_cols_slice(&floats).transpose()))
}

// ================================================================================


/// These are stored in row-major order in files
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSMatrix4(pub Mat4);
impl Parse<&[u8]> for BSMatrix4 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, result) = parse_mat4(i)?;
        Ok((i, BSMatrix4(result)))
    }
}

/// These are stored in row-major order in files
pub fn parse_mat4(i: &[u8]) -> IResult<&[u8], Mat4> {
    let (i, floats) = count(le_f32, 16)(i)?;
    Ok((i, Mat4::from_cols_slice(&floats).transpose()))
}

// ================================================================================



pub fn parse_u16_vec3(i: &[u8]) -> IResult<&[u8], U16Vec3> {
    let (i, x) = le_u16(i)?;
    let (i, y) = le_u16(i)?;
    let (i, z) = le_u16(i)?;
    Ok((i, U16Vec3::new(x, y, z)))
}
