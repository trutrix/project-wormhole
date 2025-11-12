use crate::dev::*;

define_record! {
    b"LAND", Landscape, [
        b"VHGT", VertexHeight, VertexHeightData;
        b"VNML", VertexNormals, EmptyParser;
        b"VCLR", VertexColors, EmptyParser;
        b"DATA", DATA, EmptyParser;
        b"BTXT", BaseTexture, EmptyParser;
        b"ATXT", AdditionalTexture, EmptyParser;
        b"VTXT", VTXT, EmptyParser;
        b"MPCD", HiResHeightmap, EmptyParser;
    ]
}


#[derive(NomLE)]
pub struct VertexHeightData {
    pub offset: f32,
    pub gradient: [[i8;33];33],
    _filler: [i8;3]
}

impl std::fmt::Debug for VertexHeightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexHeightData {{ offset: {}, gradient: too verbose }}", self.offset)
    }
}

impl VertexHeightData {

}