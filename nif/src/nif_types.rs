use super::dev::*;


#[derive(Debug, NomLE, PartialEq, Clone)]
pub struct NifRotation(pub Matrix3<f32>);

impl Default for NifRotation {
    fn default() -> Self {
        Self(Matrix3([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        ]))
    }
}


#[derive(Debug, NomLE, PartialEq, Clone)]
pub struct NifScale(pub f32);

impl Default for NifScale {
    fn default() -> Self {
        Self(1.0)
    }
}

impl From<NifScale> for [f32; 3] {
    fn from(scale: NifScale) -> [f32; 3] {
        [scale.0, scale.0, scale.0]
    }
}


#[derive(Debug, NomLE, PartialEq, Clone)]
pub struct NifTranslation(pub Vec3<f32>);

impl Default for NifTranslation {
    fn default() -> Self {
        Self(Vec3::zero())
    }
}