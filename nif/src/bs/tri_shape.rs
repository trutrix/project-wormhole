use project_wormhole_ba2::dev::Bounds;

use super::prelude::*;

#[derive(Debug, Clone)]
pub struct BSTriShape {
    pub av: NiAVObject,
    pub bounding_sphere: Bounds,
    pub skin: u32,
    pub shader_property: u32,
    pub alpha_property: u32,
    pub vertex_desc: BSVertexDesc,
    pub num_triangles: u32,
    pub num_vertices: u16,
    pub data_size: u32,
    pub vertex_data: Vec<BSVertexData>,
    pub triangles: Vec<Vec3<u16>>,
}

impl Parse<&[u8]> for BSTriShape {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        let (i, av) = NiAVObject::parse(i)?;
        let (i, bounding_sphere) = Bounds::parse(i)?;
        let (i, skin) = le_u32(i)?;
        let (i, shader_property) = le_u32(i)?;
        let (i, alpha_property) = le_u32(i)?;
        let (i, vertex_desc) = BSVertexDesc::parse(i)?;
        let (i, num_triangles) = le_u32(i)?;
        let (i, num_vertices) = le_u16(i)?;
        let (i, data_size) = le_u32(i)?;
        let mut vertex_data = Vec::new();
        let mut data = i;

        for _index in 0..num_vertices {
            let (i, bsdata) = BSVertexData::parse_with_flags(data, &vertex_desc)?;
            data = i;
            vertex_data.push(bsdata);
        }
        
        let (i, triangles) = count(Vec3::<u16>::parse, num_triangles as usize)(data)?;

        Ok((i, BSTriShape {
            av,
            bounding_sphere,
            skin,
            shader_property,
            alpha_property,
            vertex_desc,
            num_triangles,
            num_vertices,
            data_size,
            vertex_data,
            triangles
        }))
    }
}


impl BSTriShape {

    // Get all indices from the triangles
    pub fn get_indices(&self) -> Vec<u16> {

        // Initialize indices vector
        let mut indices = Vec::new();

        // Iterate through all triangles
        for triangle in &self.triangles {

            // Push the indices to the vector
            indices.push(triangle.x);
            indices.push(triangle.y);
            indices.push(triangle.z);
        }

        // Return the indices
        indices
    }

    pub fn get_indices_as_bytes(&self) -> Vec<u8> {
        let mut indices = Vec::new();
        for index in self.get_indices() {
            indices.extend_from_slice(&index.to_le_bytes());
        }
        indices
    }

    // Get the min and max vertices from the vertex data
    pub fn get_vertices_min_max(&self) -> (Vec3<f32>, Vec3<f32>) {
        
        // Initialize min and max to 0.0
        let mut min = Vec3::zero();
        let mut max = Vec3::zero();

        // Iterate through all vertices
        for vertex in &self.vertex_data {
            
            // If the vertex has a half vertex
            if let Some(v) = &vertex.position {

                // If min and max are still 0.0, set them to the first value
                if min.x == 0.0 && max.x == 0.0 {
                    let val = v.x;
                    min.x = val;
                    max.x = val;
                }

                // If min and max are still 0.0, set them to the first value
                if min.y == 0.0 && max.y == 0.0 {
                    let val = v.y;
                    min.y = val;
                    max.y = val;
                }

                // If min and max are still 0.0, set them to the first value
                if min.z == 0.0 && max.z == 0.0 {
                    let val = v.z;
                    min.z = val;
                    max.z = val;
                }

                // If the value is less than the min, set the min to the value
                if v.x < min.x {
                    min.x = v.x;
                }

                // If the value is less than the min, set the min to the value
                if v.y < min.y {
                    min.y = v.y;
                }

                // If the value is less than the min, set the min to the value
                if v.z < min.z {
                    min.z = v.z;
                }

                // If the value is greater than the max, set the max to the value
                if v.x > max.x {
                    max.x = v.x;
                }

                // If the value is greater than the max, set the max to the value
                if v.y > max.y {
                    max.y = v.y;
                }

                // If the value is greater than the max, set the max to the value
                if v.z > max.z {
                    max.z = v.z;
                }
            }
        }
        
        // Return the min and max
        (min, max)

    }

    // Get all vertices from the vertex data, converting them to f32 if they are f16
    pub fn get_vertices(&self) -> Vec<f32> {

        // Initialize vertices vector
        let mut vertices = Vec::new();

        // Iterate through all vertices
        for vertex in &self.vertex_data {
            
            // If the vertex has a half vertex
            if let Some(v) = &vertex.position {
                vertices.push(v.x);
                vertices.push(v.y);
                vertices.push(v.z);
            }

            // If the vertex has a full vertex
            else if let Some(v) = &vertex.position {
                vertices.push(v.x);
                vertices.push(v.y);
                vertices.push(v.z);
            }

        }
        
        // Return the vertices
        vertices
    }

    pub fn get_vertices_as_bytes(&self) -> Vec<u8> {
        let mut vertices = Vec::new();
        for vertex in self.get_vertices() {
            vertices.extend_from_slice(&vertex.to_le_bytes());
        }
        vertices
    }


    // Get all normals from the vertex data, if they exist
    pub fn get_normals(&self) -> Vec<f32> {

        // Initialize normals vector
        let mut normals = Vec::new();

        // Iterate through all vertices
        for vertex in &self.vertex_data {
            
            // If the vertex has a normal (it should)
            if let Some(v) = &vertex.normal {

                normals.push(v.x);
                normals.push(v.y);
                normals.push(v.z);

            }

        }

        // Return the normals vector
        normals
    }

    pub fn get_normals_as_bytes(&self) -> Vec<u8> {
        let mut normals = Vec::new();
        for normal in self.get_normals() {
            normals.extend_from_slice(&normal.to_le_bytes());
        }
        normals
    }

    // Get all UVs from the vertex data, if they exist
    pub fn get_uvs(&self) -> Vec<f32> {

        // Initialize uvs vector
        let mut uvs = Vec::new();

        // Iterate through all vertices
        for vertex in &self.vertex_data {
            
            // If the vertex has a uv, push it to the vector
            if let Some(v) = &vertex.uv {
                uvs.push(v.x);
                uvs.push(v.y);
            }

        }

        // Return the uvs vector
        uvs
    }

    pub fn get_uvs_as_bytes(&self) -> Vec<u8> {
        let mut uvs = Vec::new();
        for uv in self.get_uvs() {
            uvs.extend_from_slice(&uv.to_le_bytes());
        }
        uvs
    }

    pub fn get_bone_weights(&self) -> Vec<f32> {
        let mut weights = Vec::new();
        for vertex in &self.vertex_data {
            if let Some(bone_weights) = &vertex.bone_weights {
                let lin: [f32; 4] = bone_weights.normalize_weights().into();
                weights.extend_from_slice(lin.as_slice());
            }
        }

        weights
    }

    pub fn get_bone_weights_as_bytes(&self) -> Vec<u8> {
        let mut weights = Vec::new();
        for weight in self.get_bone_weights() {
            weights.extend_from_slice(&weight.to_le_bytes());
        }
        weights
    }

    pub fn get_bone_indices(&self) -> Vec<&Vec4<u8>> {
        let mut indices = Vec::new();
        for vertex in &self.vertex_data {
            if let Some(bone_indices) = &vertex.bone_indices {
                indices.push(bone_indices);
            }
        }

        indices
    }

    pub fn get_bone_indices_as_bytes(&self) -> Vec<u8> {
        let mut indices = Vec::new();
        for index in self.get_bone_indices() {
            let lin: [u8; 4] = index.clone().into();
            indices.extend_from_slice(lin.as_slice());
        }
        indices
    }


    pub fn get_vertex_positions(&self) -> Vec<Vec3<f32>> {
        let mut positions = Vec::new();
        for vertex in &self.vertex_data {
            if let Some(v) = vertex.position {
                positions.push(v);
            }
        }

        positions
    }

    pub fn get_vertex_normals(&self) -> Vec<Vec3<f32>> {
        let mut normals = Vec::new();
        for vertex in &self.vertex_data {
            if let Some(n) = vertex.normal {
                normals.push(n);
            }
        }

        normals
    }

    pub fn get_triangle_indices(&self) -> Vec<Vec3<u16>> {
        let halves = self.triangles.clone();
        halves.iter().map(|x| Vec3 { x: x.x, y: x.y, z: x.z}).collect::<Vec<Vec3<u16>>>()
    }

    pub fn get_vertex_uvs(&self) -> Vec<Vec2<f32>> {
        let mut uvs = Vec::new();
        for vertex in &self.vertex_data {
            if let Some(v) = vertex.uv {
                uvs.push(v);
            }
        }

        uvs
    }

    pub fn get_vertex_weights(&self) -> Vec<Vec4<f32>> {
        let mut weights = Vec::new();
        for vertex in &self.vertex_data {
            if let Some(w) = vertex.bone_weights {
                weights.push(w.normalize_weights());
            }
        }

        weights
    }

    pub fn get_vertex_joints(&self) -> Vec<Vec4<u8>> {
        let mut joints = Vec::new();
        for vertex in &self.vertex_data {
            if let Some(j) = vertex.bone_indices {
                joints.push(j);
            }
        }

        joints
    }

}




