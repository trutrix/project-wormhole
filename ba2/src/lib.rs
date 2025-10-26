pub mod shared;
pub mod archive;
mod tests;

pub mod prelude {
    pub use nom::*;
    pub use nom::bytes::complete::take;
    pub use nom::multi::count;
    pub use nom::multi::many0;
    pub use nom::combinator::complete;
    pub use nom_derive::*;
    pub use std::io::{Read, Seek, SeekFrom};
    pub use std::fs::File;
    pub use log::*;
    pub use proc::cc4;
    pub use shared::fourcc::FourCC;
    pub use shared::common::standardize_path;
    pub use std::collections::BTreeMap;

    pub use super::shared::*;
    pub use super::archive::*;


    pub fn format_from_u8(format: u8) -> image_dds::ddsfile::DxgiFormat {
        image_dds::ddsfile::DxgiFormat::try_from(format).unwrap()
    }
    
    
    pub fn srgb_transfer(srgb: u8) -> u8 {
        let t = srgb as f32;
        let t = t.powf(1.0 / 2.2);
        t as u8
    }
    
    pub fn fix_s_image(image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
        for pixel in image.pixels_mut() {
            pixel[3] = pixel[1];
            let t = srgb_transfer(pixel[0]);
            pixel[0] = t;
            pixel[1] = t;
            pixel[2] = t;
        }
    }
    

}