
use glam::*;
use nom_derive::nom::IResult;
use nom_derive::Parse;

// ================================================================================

#[derive(Clone, Copy, PartialEq)]
pub struct BSVec2(pub Vec2);

impl Parse<&[u8]> for BSVec2 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = f32::parse_le(i)?;
        let (i, y) = f32::parse_le(i)?;
        Ok((i, BSVec2(Vec2::new(x, y))))
    }
}

// ================================================================================

#[derive(Clone, Copy, PartialEq)]
pub struct BSVec3(pub Vec3);

impl Parse<&[u8]> for BSVec3 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = f32::parse_le(i)?;
        let (i, y) = f32::parse_le(i)?;
        let (i, z) = f32::parse_le(i)?;
        Ok((i, BSVec3(Vec3::new(x, y, z))))
    }
}

// ================================================================================


#[derive(Clone, Copy, PartialEq)]
pub struct BSVec4(pub Vec4);

impl Parse<&[u8]> for BSVec4 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = f32::parse_le(i)?;
        let (i, y) = f32::parse_le(i)?;
        let (i, z) = f32::parse_le(i)?;
        let (i, w) = f32::parse_le(i)?;
        Ok((i, BSVec4(Vec4::new(x, y, z, w))))
    }
}

// ================================================================================

#[derive(Clone, Copy, PartialEq)]
pub struct BSQuat(pub Quat);

impl Parse<&[u8]> for BSQuat {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = f32::parse_le(i)?;
        let (i, y) = f32::parse_le(i)?;
        let (i, z) = f32::parse_le(i)?;
        let (i, w) = f32::parse_le(i)?;
        Ok((i, BSQuat(Quat::from_xyzw(x, y, z, w))))
    }
}

// ================================================================================


#[derive(Clone, Copy, PartialEq)]
pub struct BSMatrix3(pub Mat3);

impl Parse<&[u8]> for BSMatrix3 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        // TODO: not sure these are stored in column-major order, verify (looking at you nif files)
        let (i, c1) = BSVec3::parse_le(i)?;
        let (i, c2) = BSVec3::parse_le(i)?;
        let (i, c3) = BSVec3::parse_le(i)?;

        Ok((i, BSMatrix3(Mat3::from_cols(c1.0, c2.0, c3.0))))
    }
}

// ================================================================================


#[derive(Clone, Copy, PartialEq)]
pub struct BSMatrix4(pub Mat4);
impl Parse<&[u8]> for BSMatrix4 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        // TODO: not sure these are stored in column-major order, verify (looking at you nif files)
        let (i, c1) = BSVec4::parse_le(i)?;
        let (i, c2) = BSVec4::parse_le(i)?;
        let (i, c3) = BSVec4::parse_le(i)?;
        let (i, c4) = BSVec4::parse_le(i)?;

        Ok((i, BSMatrix4(Mat4::from_cols(c1.0, c2.0, c3.0, c4.0))))
    
    }
}

// ================================================================================

