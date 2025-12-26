
use glam::*;
use nom_derive::nom::IResult;
use nom_derive::Parse;

// ================================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSVec2(pub Vec2);

impl Parse<&[u8]> for BSVec2 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = f32::parse_le(i)?;
        let (i, y) = f32::parse_le(i)?;
        Ok((i, BSVec2(Vec2::new(x, y))))
    }
}

pub fn parse_vec2(i: &[u8]) -> IResult<&[u8], Vec2> {
    let (i, x) = f32::parse_le(i)?;
    let (i, y) = f32::parse_le(i)?;
    Ok((i, Vec2::new(x, y)))
}

// ================================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BSVec3(pub Vec3);

impl Parse<&[u8]> for BSVec3 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, x) = f32::parse_le(i)?;
        let (i, y) = f32::parse_le(i)?;
        let (i, z) = f32::parse_le(i)?;
        Ok((i, BSVec3(Vec3::new(x, y, z))))
    }
}

pub fn parse_vec3(i: &[u8]) -> IResult<&[u8], Vec3> {
    let (i, x) = f32::parse_le(i)?;
    let (i, y) = f32::parse_le(i)?;
    let (i, z) = f32::parse_le(i)?;
    Ok((i, Vec3::new(x, y, z)))
}

// ================================================================================


#[derive(Clone, Copy, PartialEq, Debug)]
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

pub fn parse_vec4(i: &[u8]) -> IResult<&[u8], Vec4> {
    let (i, x) = f32::parse_le(i)?;
    let (i, y) = f32::parse_le(i)?;
    let (i, z) = f32::parse_le(i)?;
    let (i, w) = f32::parse_le(i)?;
    Ok((i, Vec4::new(x, y, z, w)))
}

// ================================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
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

pub fn parse_quat(i: &[u8]) -> IResult<&[u8], Quat> {
    let (i, x) = f32::parse_le(i)?;
    let (i, y) = f32::parse_le(i)?;
    let (i, z) = f32::parse_le(i)?;
    let (i, w) = f32::parse_le(i)?;
    Ok((i, Quat::from_xyzw(x, y, z, w)))
}

// ================================================================================


#[derive(Clone, Copy, PartialEq, Debug)]
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

pub fn parse_mat3(i: &[u8]) -> IResult<&[u8], Mat3> {
    let (i, c1) = parse_vec3(i)?;
    let (i, c2) = parse_vec3(i)?;
    let (i, c3) = parse_vec3(i)?;
    Ok((i, Mat3::from_cols(c1, c2, c3)))
}

// ================================================================================


#[derive(Clone, Copy, PartialEq, Debug)]
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

pub fn parse_mat4(i: &[u8]) -> IResult<&[u8], Mat4> {
    let (i, c1) = parse_vec4(i)?;
    let (i, c2) = parse_vec4(i)?;
    let (i, c3) = parse_vec4(i)?;
    let (i, c4) = parse_vec4(i)?;
    Ok((i, Mat4::from_cols(c1, c2, c3, c4)))
}

impl BSMatrix4 {
    pub fn to_bytes_mc(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        for i in 0..4 {
            for j in 0..4 {
                let value = self.0.col(i)[j];
                let byte_index = (j * 4 + i) * 4;
                bytes[byte_index..byte_index + 4].copy_from_slice(&value.to_le_bytes());
            }
        }
        bytes
    }

    pub fn to_bytes_mr(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        for i in 0..4 {
            for j in 0..4 {
                let value = self.0.row(i)[j];
                let byte_index = (i * 4 + j) * 4;
                bytes[byte_index..byte_index + 4].copy_from_slice(&value.to_le_bytes());
            }
        }
        bytes
    }
}

// ================================================================================



pub fn parse_u16_vec3(i: &[u8]) -> IResult<&[u8], U16Vec3> {
    let (i, x) = u16::parse_le(i)?;
    let (i, y) = u16::parse_le(i)?;
    let (i, z) = u16::parse_le(i)?;
    Ok((i, U16Vec3::new(x, y, z)))
}
