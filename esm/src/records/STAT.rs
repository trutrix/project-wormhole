use crate::dev::*;

define_record3! {
    "iden": b"STAT";
    "name": Static;
    "fields": [
        EditorId;
        ModelData;
        PreviewTransform;
        VirtualMachineAdapter;
        ObjectBounds;
        FullName;
        Properties;
        b"NVNM", NavigationMesh, EmptyParser; // TODO: common navmesh struct
        b"MNAM", DistantLOD, EmptyParser; // TODO: common distant LOD struct
        b"DNAM", DirectionalMaterial, StaticDirectionalMaterial;
        b"FTYP", ForcedLocationRefType, FormId;
    ]
}


#[derive(Debug, NomLE)]
pub struct StaticDirectionalMaterial {
    pub max_angle: f32,
    pub material: FormId,
    pub leaf_amplitude: Option<f32>,
    pub leaf_frequency: Option<f32>
}