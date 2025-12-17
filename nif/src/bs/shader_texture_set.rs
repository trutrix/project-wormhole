use project_wormhole_ba2::dev::{ensure_texture_parent, standardize_path};

use super::prelude::*;

#[derive(Debug, Clone)]
pub struct BSShaderTextureSet {
    pub diffuse: Option<String>,
    pub normal: Option<String>,
    pub glow: Option<String>,
    pub specular: Option<String>,
}

impl Parse<&[u8]> for BSShaderTextureSet {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, string_count) = le_u32(i)?;
        let (i, textures) = count(SizedString32::parse, string_count as usize)(i)?;

        warn!("{} textures", textures.len());

        let mut fixed = Vec::new();

        for texture in textures {
            let mut fixed_path = standardize_path(&texture.0);
            ensure_texture_parent(&mut fixed_path);

            fixed.push(fixed_path);
        }

        let mut tset = BSShaderTextureSet { diffuse: None, normal: None, glow: None, specular: None};

        for texture in fixed {

            if texture == "textures/" {
                continue;
            }

            if texture.ends_with("d.dds") {
                tset.diffuse = Some(texture);
            } else if texture.ends_with("n.dds") {
                tset.normal = Some(texture);
            } else if texture.ends_with("g.dds") {
                tset.glow = Some(texture);
            } else if texture.ends_with("s.dds") {
                tset.specular = Some(texture);
            } else {
                warn!("Unknown texture type: {}", texture);
            }
        }

        Ok((i, tset))
    }
}