use nom_derive::{Parse, nom::{error::{Error, ErrorKind}, number::complete::{le_f32, le_u8, le_u32}}};
use project_wormhole_shared::{structs::fourcc::FourCC, structs::u8_bool::parse_u8_bool};

// Link to info about bgsm
// https://github.com/ousnius/Material-Editor/blob/master/MaterialLib/BGSM.cs

#[derive(Debug)]
pub struct BaseMaterialFile {
    pub magic: FourCC,
    pub version: u32,
    pub tile_flags: u32,
    pub u_offset: f32,
    pub v_offset: f32,
    pub u_scale: f32,
    pub v_scale: f32,

    pub alpha: f32,
    pub alpha_blend_mode_0: u8,
    pub alpha_blend_mode_1: u32,
    pub alpha_blend_mode_2: u32,
    pub alpha_test_ref: u8,

    pub alpha_test: bool,
    pub z_buffer_write: bool,
    pub z_buffer_test: bool,
    pub screen_space_reflections: bool,
    pub wetness_control_screen_space_reflections: bool,
    pub decal: bool,
    pub two_sided: bool,
    pub decal_no_fade: bool,
    pub non_occluder: bool,

    pub refraction: bool,
    pub refraction_falloff: bool,
    pub refraction_power: f32,

    pub environment_mapping: Option<bool>,
    pub environment_mapping_mask_scale: Option<f32>,

    pub depth_bias: Option<bool>,

    pub grayscale_to_palette_color: bool,

    pub mask_writes: Option<u8>,
}

impl BaseMaterialFile {
    pub fn is_bgsm(&self) -> bool {
        self.magic.0 == *b"BGSM"
    }

    pub fn is_bgem(&self) -> bool {
        self.magic.0 == *b"BGEM"
    }

    pub fn tile_u(&self) -> bool {
        (self.tile_flags & 2) != 0
    }

    pub fn tile_v(&self) -> bool {
        (self.tile_flags & 1) != 0
    }
}

impl Parse<&[u8]> for BaseMaterialFile {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, Error<&[u8]>> {
            
            let (i, magic) = FourCC::parse(i)?;

            #[cfg(debug_assertions)]
            if magic.0 != *b"BGSM" && magic.0 != *b"BGEM" {
                return Err(nom_derive::nom::Err::Error(Error::new(i, ErrorKind::Tag)));
            }

            let (i, version) = le_u32(i)?;
            let (i, tile_flags) = le_u32(i)?;
            let (i, u_offset) = le_f32(i)?;
            let (i, v_offset) = le_f32(i)?;
            let (i, u_scale) = le_f32(i)?;
            let (i, v_scale) = le_f32(i)?;
            let (i, alpha) = le_f32(i)?;
            let (i, alpha_blend_mode_0) = le_u8(i)?;
            let (i, alpha_blend_mode_1) = le_u32(i)?;
            let (i, alpha_blend_mode_2) = le_u32(i)?;
            let (i, alpha_test_ref) = le_u8(i)?;
            let (i, alpha_test) = parse_u8_bool(i)?;
            let (i, z_buffer_write) = parse_u8_bool(i)?;
            let (i, z_buffer_test) = parse_u8_bool(i)?;
            let (i, screen_space_reflections) = parse_u8_bool(i)?;
            let (i, wetness_control_screen_space_reflections) = parse_u8_bool(i)?;
            let (i, decal) = parse_u8_bool(i)?;
            let (i, two_sided) = parse_u8_bool(i)?;
            let (i, decal_no_fade) = parse_u8_bool(i)?;
            let (i, non_occluder) = parse_u8_bool(i)?;
            let (i, refraction) = parse_u8_bool(i)?;
            let (i, refraction_falloff) = parse_u8_bool(i)?;
            let (i, refraction_power) = le_f32(i)?;

            let (i, (environment_mapping, environment_mapping_mask_scale, depth_bias)) = if version < 10 {
                let (i, environment_mapping) = parse_u8_bool(i)?;
                let (i, environment_mapping_mask_scale) = le_f32(i)?;
                (i, (Some(environment_mapping), Some(environment_mapping_mask_scale), None))
            } else {
                let (i, depth_bias) = parse_u8_bool(i)?;
                (i, (None, None, Some(depth_bias)))
            };

            let (i, grayscale_to_palette_color) = parse_u8_bool(i)?;

            let (i, mask_writes) = if version >= 6 {
                let (i, mask_writes) = le_u8(i)?;
                (i, Some(mask_writes))
            } else {
                (i, None)
            };


            Ok((i, Self {
                magic,
                version,
                tile_flags,
                u_offset,
                v_offset,
                u_scale,
                v_scale,
                alpha,
                alpha_blend_mode_0,
                alpha_blend_mode_1,
                alpha_blend_mode_2,
                alpha_test_ref,
                alpha_test,
                z_buffer_write,
                z_buffer_test,
                screen_space_reflections,
                wetness_control_screen_space_reflections,
                decal,
                two_sided,
                decal_no_fade,
                non_occluder,
                refraction,
                refraction_falloff,
                refraction_power,
                environment_mapping,
                environment_mapping_mask_scale,
                depth_bias,
                grayscale_to_palette_color,
                mask_writes,
            }) )

    }
}




pub struct BGSM {
    pub base: BaseMaterialFile,
    pub diffuse_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub smoothness_specular_texture: Option<String>,
    pub emissive_texture: Option<String>,
}