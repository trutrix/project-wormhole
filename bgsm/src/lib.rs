mod test;
use nom_derive::{NomLE, Parse, nom::{error::{Error, ErrorKind}, number::complete::{le_f32, le_i32, le_u8, le_u16, le_u32}}};
use nom_derive::nom;
use project_wormhole_shared::{prelude::{ParseVersioned, U32ESMString, empty_string_to_none, parse_u32_esm_string}, structs::{color::{Color3, Color4}, fourcc::FourCC, u8_bool::parse_u8_bool}};

// Link to info about bgsm
// https://github.com/ousnius/Material-Editor/blob/master/MaterialLib/BGSM.cs

#[derive(Debug, Default)]
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



#[derive(Debug, Default)]
pub struct BGSM {
    pub base: BaseMaterialFile,
    pub diffuse_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub smoothness_specular_texture: Option<String>,
    pub emissive_texture: Option<String>,
    pub glow_texture: Option<String>,
    pub wrinkles_texture: Option<String>,
    pub specular_texture: Option<String>,
    pub lighting_texture: Option<String>,
    pub flow_texture: Option<String>,
    pub distance_field_alpha_texture: Option<String>,
    pub envmap_texture: Option<String>,
    pub inner_layer_texture: Option<String>,
    pub displacement_texture: Option<String>,
    pub enable_editor_alpha_ref: bool,
    pub translucency_options: Option<TranslucencyOptions>,
    pub lighting_options: Option<LightingOptions>,
    pub specular_enabled: bool,
    pub specular_color: Color3<f32>,
    pub specular_mult: f32,
    pub smoothness: f32,
    pub fresnel_power: f32,
    pub wetness: Wetness,
    pub pbr: Option<bool>,
    pub custom_porosity: Option<bool>,
    pub porosity_value: Option<f32>,
    pub root_material_path: Option<String>,
    pub aniso_lighting: bool,
    pub emit_enabled: bool,
    pub emittance_color: Option<Color4<u8>>,
    pub emittance_mult: f32
}


impl Parse<&[u8]> for BGSM {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, Error<&[u8]>> {
        let (i, base) = BaseMaterialFile::parse(i)?;

        #[cfg(debug_assertions)]
        if base.magic.0 != *b"BGSM" {
            return Err(nom_derive::nom::Err::Error(Error::new(i, ErrorKind::Tag)));
        }

        let (i, diffuse_texture) = parse_u32_esm_string(i)?;
        let diffuse_texture = empty_string_to_none(diffuse_texture);

        let (i, normal_texture) = parse_u32_esm_string(i)?;
        let normal_texture = empty_string_to_none(normal_texture);

        let (i, smoothness_specular_texture) = parse_u32_esm_string(i)?;
        let smoothness_specular_texture = empty_string_to_none(smoothness_specular_texture);

        let (i, emissive_texture) = parse_u32_esm_string(i)?;
        let emissive_texture = empty_string_to_none(emissive_texture);

        let glow_texture;
        let wrinkles_texture;
        let mut specular_texture = None;
        let mut lighting_texture = None;
        let mut flow_texture = None;
        let mut distance_field_alpha_texture = None;
        let mut envmap_texture = None;
        let mut inner_layer_texture = None;
        let mut displacement_texture = None;

        let mut data = i;

        if base.version > 2 {
            let (i, gt) = parse_u32_esm_string(data)?;
            glow_texture = empty_string_to_none(gt);

            let (i, wt) = parse_u32_esm_string(i)?;
            wrinkles_texture = empty_string_to_none(wt);

            let (i, st) = parse_u32_esm_string(i)?;
            specular_texture = empty_string_to_none(st);

            let (i, lt) = parse_u32_esm_string(i)?;
            lighting_texture = empty_string_to_none(lt);

            let (i, ft) = parse_u32_esm_string(i)?;
            flow_texture = empty_string_to_none(ft);

            data = i;

            if base.version >= 17 {
                let (i, dfas) = parse_u32_esm_string(data)?;
                distance_field_alpha_texture = empty_string_to_none(dfas);
                data = i;
            }
        } else {
            let (i, et) = parse_u32_esm_string(data)?;
            envmap_texture = empty_string_to_none(et);

            let (i, gt) = parse_u32_esm_string(i)?;
            glow_texture = empty_string_to_none(gt);

            let (i, ilt) = parse_u32_esm_string(i)?;
            inner_layer_texture = empty_string_to_none(ilt);

            let (i, wt) = parse_u32_esm_string(i)?;
            wrinkles_texture = empty_string_to_none(wt);

            let (i, dt) = parse_u32_esm_string(i)?;
            displacement_texture = empty_string_to_none(dt);

            data = i;
        }


        let (i, enable_editor_alpha_ref) = parse_u8_bool(data)?;


        let mut translucency_options = None;
        let mut lighting_options = None;

        if base.version >= 8 {
            let (i, to) = TranslucencyOptions::parse(i)?;
            translucency_options = Some(to);
            data = i;
        } else {
            let (i, lo) = LightingOptions::parse(i)?;
            println!("Parsed lighting options: {:?}", lo);
            lighting_options = Some(lo);
            
            data = i;
        }

        let (i, specular_enabled) = parse_u8_bool(data)?;
        let (i, specular_color) = Color3::<f32>::parse(i)?;
        let (i, specular_mult) = le_f32(i)?;
        let (i, smoothness) = le_f32(i)?;
        let (i, fresnel_power) = le_f32(i)?;
        let (i, wetness) = Wetness::parse_versioned(i, base.version)?;
        println!("Parsed wetness: {:?}", wetness);



        let (i, pbr) = if base.version > 2 {
            let (i, pbr) = parse_u8_bool(i)?;
            (i, Some(pbr))
        } else {
            println!("Skipping PBR parsing for version {}", base.version);
            (i, None)
        };

        let mut custom_porosity = None;
        let mut porosity_value = None;

        if base.version >= 9 {
            let (i, cp) = parse_u8_bool(i)?;
            custom_porosity = Some(cp);

            let (i, pv) = le_f32(i)?;
            porosity_value = Some(pv);

            data = i;
        } else {
            println!("Skipping porosity parsing for version {}", base.version);
        }

        let (_, next_value) = le_u32(data)?;
        println!("Next value after porosity (if applicable): {}", next_value);

        let (i, rmp) = parse_u32_esm_string(data)?;
        let root_material_path = empty_string_to_none(rmp);

        // s

        //     AnisoLighting = input.ReadBoolean();
        //     EmitEnabled = input.ReadBoolean();

        //     if (EmitEnabled)
        //     {
        //         EmittanceColor = Color.Read(input).ToUInt32();
        //     }

        //     EmittanceMult = input.ReadSingle();
        //     ModelSpaceNormals = input.ReadBoolean();
        //     ExternalEmittance = input.ReadBoolean();

        //     if (Version >= 12)
        //     {
        //         LumEmittance = input.ReadSingle();
        //     }

        //     if (Version >= 13)
        //     {
        //         UseAdaptativeEmissive = input.ReadBoolean();
        //         AdaptativeEmissive_ExposureOffset = input.ReadSingle();
        //         AdaptativeEmissive_FinalExposureMin = input.ReadSingle();
        //         AdaptativeEmissive_FinalExposureMax = input.ReadSingle();
        //     }

        //     if (Version < 8)
        //     {
        //         BackLighting = input.ReadBoolean();
        //     }

        //     ReceiveShadows = input.ReadBoolean();
        //     HideSecret = input.ReadBoolean();
        //     CastShadows = input.ReadBoolean();
        //     DissolveFade = input.ReadBoolean();
        //     AssumeShadowmask = input.ReadBoolean();

        //     Glowmap = input.ReadBoolean();

        //     if (Version < 7)
        //     {
        //         EnvironmentMappingWindow = input.ReadBoolean();
        //         EnvironmentMappingEye = input.ReadBoolean();
        //     }

        //     Hair = input.ReadBoolean();
        //     HairTintColor = Color.Read(input).ToUInt32();

        //     Tree = input.ReadBoolean();
        //     Facegen = input.ReadBoolean();
        //     SkinTint = input.ReadBoolean();
        //     Tessellate = input.ReadBoolean();

        //     if (Version < 3)
        //     {
        //         DisplacementTextureBias = input.ReadSingle();
        //         DisplacementTextureScale = input.ReadSingle();
        //         TessellationPnScale = input.ReadSingle();
        //         TessellationBaseFactor = input.ReadSingle();
        //         TessellationFadeDistance = input.ReadSingle();
        //     }

        //     GrayscaleToPaletteScale = input.ReadSingle();

        //     if (Version >= 1)
        //     {
        //         SkewSpecularAlpha = input.ReadBoolean();
        //     }

        //     if (Version >= 3)
        //     {
        //         Terrain = input.ReadBoolean();

        //         if (Terrain)
        //         {
        //             if (Version == 3)
        //             {
        //                 UnkInt1 = input.ReadUInt32();
        //             }

        //             TerrainThresholdFalloff = input.ReadSingle();
        //             TerrainTilingDistance = input.ReadSingle();
        //             TerrainRotationAngle = input.ReadSingle();
        //         }
        //     }


        Ok((i, Self {
            base,
            diffuse_texture,
            normal_texture,
            smoothness_specular_texture,
            emissive_texture,
            glow_texture,
            wrinkles_texture,
            specular_texture,
            lighting_texture,
            flow_texture,
            distance_field_alpha_texture,
            envmap_texture,
            inner_layer_texture,
            displacement_texture,
            enable_editor_alpha_ref,
            translucency_options,
            lighting_options,
            specular_enabled,
            specular_color,
            specular_mult,
            smoothness,
            fresnel_power,
            wetness,
            pbr,
            custom_porosity,
            porosity_value,
            root_material_path,
            aniso_lighting: false,
            emit_enabled: false,
            emittance_color: None,
            emittance_mult: 0.0,
        }))
    }
}


// ================================================================================


#[derive(Debug)]
pub struct TranslucencyOptions {
    pub translucency: bool,
    pub translucency_thick_object: bool,
    pub translucency_mix_albedo_with_subsurface_color: bool,
    pub translucency_subsurface_color: Color4<u8>,
    pub translucency_transmissive_scale: f32,
    pub translucency_turbulence: f32,
}

impl Parse<&[u8]> for TranslucencyOptions {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, Error<&[u8]>> {
        let (i, translucency) = parse_u8_bool(i)?;
        let (i, translucency_thick_object) = parse_u8_bool(i)?;
        let (i, translucency_mix_albedo_with_subsurface_color) = parse_u8_bool(i)?;
        let (i, translucency_subsurface_color) = Color4::<u8>::parse(i)?;
        let (i, translucency_transmissive_scale) = le_f32(i)?;
        let (i, translucency_turbulence) = le_f32(i)?;
        Ok((i, Self {
            translucency,
            translucency_thick_object,
            translucency_mix_albedo_with_subsurface_color,
            translucency_subsurface_color,
            translucency_transmissive_scale,
            translucency_turbulence,
        }))
    }
}


// ================================================================================

#[derive(Debug)]
pub struct LightingOptions {
    pub rim_lighting: bool,
    pub rim_power: f32,
    pub back_light_power: f32,
    pub subsurface_lighting: bool,
    pub subsurface_lighting_rolloff: f32,
}

impl Parse<&[u8]> for LightingOptions {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, Error<&[u8]>> {
        let (i, rim_lighting) = parse_u8_bool(i)?;
        let (i, rim_power) = le_f32(i)?;
        let (i, back_light_power) = le_f32(i)?;
        let (i, subsurface_lighting) = parse_u8_bool(i)?;
        let (i, subsurface_lighting_rolloff) = le_f32(i)?;

        Ok((i, Self {
            rim_lighting,
            rim_power,
            back_light_power,
            subsurface_lighting,
            subsurface_lighting_rolloff,
        }))
    }
}


// ================================================================================

#[derive(Debug, Default)]
pub struct Wetness {
    pub spec_scale: f32,
    pub spec_power_scale: f32,
    pub spec_minvar: f32,
    pub env_map_scale: Option<f32>,
    pub fresnel_power: f32,
    pub metalness: f32,
}


impl ParseVersioned<u32> for Wetness {
    fn parse_versioned(i: &[u8], version: u32) -> nom::IResult<&[u8], Self> {
        let (i, spec_scale) = le_f32(i)?;
        let (i, spec_power_scale) = le_f32(i)?;
        let (i, spec_minvar) = le_f32(i)?;

        let (i, env_map_scale) = if version < 10 {
            let (i, scale) = le_f32(i)?;
            (i, Some(scale))
        } else {
            (i, None)
        };

        let (i, fresnel_power) = le_f32(i)?;
        let (i, metalness) = le_f32(i)?;

        Ok((i, Self {
            spec_scale,
            spec_power_scale,
            spec_minvar,
            env_map_scale,
            fresnel_power,
            metalness,
        }))
    }
}