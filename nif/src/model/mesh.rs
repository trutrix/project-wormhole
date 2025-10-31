use crate::dev::*;

// Type aliases, used to make the code more readable

/// A position in 3D space, 3 floats
type Position = Vec3<f32>;
/// A normal vector, 3 floats, optional
type Normal = Vec3<f32>;
/// A UV coordinate, 2 floats, optional, used for texture mapping
type UV = Vec2<f32>;
/// A triangle, 3 unsigned integers, used to define a face between 3 vertices
type Triangle = Vec3<u16>;
/// A set of weights, 4 floats, optional, used for skinning
type Weights = Vec4<f32>;
/// A set of joints, 4 unsigned integers, optional (required for weights to function), used for skinning
type Joints = Vec4<u8>;


pub struct Mesh {
    pub name: String,
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    pub material_index: Option<u8>
}

impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mesh {{ vertices: {:?}, triangles: {:?}, material_index: {:?} }}", self.vertices.len(), self.triangles.len(), self.material_index)
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            name: "Unnamed".to_string(),
            vertices: Vec::new(),
            triangles: Vec::new(),
            material_index: None
        }
    }
}

impl Mesh {
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    pub fn has_normals(&self) -> bool {
        self.vertices.iter().any(|v| v.has_normal())
    }

    pub fn has_uvs(&self) -> bool {
        self.vertices.iter().any(|v| v.has_uv())
    }

    pub fn is_valid(&self) -> Result<(), String> {
        for vertex in &self.vertices {
            vertex.is_valid()?;
        }

        Ok(())
    }

    pub fn vertex_positions(&self) -> Vec<Position> {
        self.vertices.iter().map(|v| v.position.clone()).collect()
    }

    pub fn vertex_positions_as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for vertex in &self.vertices {
            data.extend_from_slice(&vertex.position.x.to_le_bytes());
            data.extend_from_slice(&vertex.position.y.to_le_bytes());
            data.extend_from_slice(&vertex.position.z.to_le_bytes());
        }
        data
    }

    pub fn vertex_normals(&self) -> Vec<Normal> {
        if self.has_normals() {
            return self.vertices.iter().map(|v| v.normal.clone().unwrap()).collect();
        } else {
            Vec::new()
        }
    }

    pub fn vertex_normals_as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for vertex in &self.vertices {
            if let Some(normal) = vertex.normal {
                let normal = normal.normalize();
                data.extend_from_slice(&normal.x.to_le_bytes());
                data.extend_from_slice(&normal.y.to_le_bytes());
                data.extend_from_slice(&normal.z.to_le_bytes());
            }
        }   
        data
    }

    pub fn triangles_as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for triangle in &self.triangles {
            data.extend_from_slice(&triangle.x.to_le_bytes());
            data.extend_from_slice(&triangle.y.to_le_bytes());
            data.extend_from_slice(&triangle.z.to_le_bytes());
        }
        data
    }

    pub fn vertex_uvs(&self) -> Vec<UV> {
        if self.has_uvs() {
            return self.vertices.iter().map(|v| v.uv.unwrap()).collect();
        } else {
            Vec::new()
        }
    }

    pub fn vertex_uvs_as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for vertex in &self.vertices {
            if let Some(uv) = vertex.uv {
                data.extend_from_slice(&uv.x.to_le_bytes());
                data.extend_from_slice(&uv.y.to_le_bytes());
            }
        }
        data
    }

    
    pub fn get_positions_min_max(&self) -> (Vec3<f32>, Vec3<f32>) {
        let positions = self.vertex_positions();

        if positions.len() == 0 {
            return (Vec3::zero(), Vec3::zero());
        }

        let mut min = Vec3 { x: positions[0].x, y: positions[0].y, z: positions[0].z };
        let mut max = min;

        for position in positions {
            min.x = min.x.min(position.x);
            min.y = min.y.min(position.y);
            min.z = min.z.min(position.z);
            
            max.x = max.x.max(position.x);
            max.y = max.y.max(position.y);
            max.z = max.z.max(position.z);
        }

        (min, max)
    }

    pub fn vertex_weights(&self) -> Vec<Weights> {
        self.vertices.iter().filter_map(|v| v.weights).collect()
    }

    pub fn vertex_weights_as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for vertex in &self.vertices {
            if let Some(weights) = vertex.weights {
                //let weights = weights.normalize();
                data.extend_from_slice(&weights.x.to_le_bytes());
                data.extend_from_slice(&weights.y.to_le_bytes());
                data.extend_from_slice(&weights.z.to_le_bytes());
                data.extend_from_slice(&weights.w.to_le_bytes());
            }
        }
        data
    }

    pub fn vertex_joints(&self) -> Vec<Joints> {
        self.vertices.iter().filter_map(|v| v.joints).collect()
    }

    pub fn vertex_joints_as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for vertex in &self.vertices {
            if let Some(joints) = vertex.joints {
                data.extend_from_slice(&joints.x.to_le_bytes());
                data.extend_from_slice(&joints.y.to_le_bytes());
                data.extend_from_slice(&joints.z.to_le_bytes());
                data.extend_from_slice(&joints.w.to_le_bytes());
            }
        }
        data
    }

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vertex {
    position: Position,
    normal: Option<Normal>,
    uv: Option<UV>,
    weights: Option<Weights>,
    joints: Option<Joints>
}

impl Vertex {
    pub fn has_normal(&self) -> bool { self.normal.is_some() }
    pub fn has_uv(&self) -> bool { self.uv.is_some() }
    pub fn has_weights(&self) -> bool { self.weights.is_some() }
    pub fn has_joints(&self) -> bool { self.joints.is_some() }

    pub fn new(position: Position, normal: Option<Normal>, uv: Option<UV>, weights: Option<Weights>, joints: Option<Joints>) -> Self {
        Vertex {
            position,
            normal,
            uv,
            weights,
            joints
        }
    }

    pub const fn position(&self) -> Position { self.position }
    pub const fn normal(&self) -> Option<Normal> { self.normal }
    pub const fn uv(&self) -> Option<UV> { self.uv }
    pub const fn weights(&self) -> Option<Weights> { self.weights }
    pub const fn joints(&self) -> Option<Joints> { self.joints }

    pub fn set_position(&mut self, position: Position) { self.position = position; }
    pub fn set_normal(&mut self, normal: Normal) { self.normal = Some(normal); }
    pub fn set_uv(&mut self, uv: UV) { self.uv = Some(uv); }
    pub fn set_weights(&mut self, weights: Weights) { self.weights = Some(weights); }
    pub fn set_joints(&mut self, joints: Joints) { self.joints = Some(joints); }

    pub fn is_valid(&self) -> Result<(), String> {

        if self.position.x.is_nan() || self.position.y.is_nan() || self.position.z.is_nan() {
            return Err("Vertex position contains NaN".to_string());
        }

        if let Some(normal) = self.normal {
            if normal.x.is_nan() || normal.y.is_nan() || normal.z.is_nan() {
                return Err("Vertex normal contains NaN".to_string());
            }
        }

        if let Some(uv) = self.uv {
            if uv.x.is_nan() || uv.y.is_nan() {
                return Err("Vertex uv contains NaN".to_string());
            }
        }

        if let Some(weights) = self.weights {
            if weights.x.is_nan() || weights.y.is_nan() || weights.z.is_nan() || weights.w.is_nan() {
                return Err("Vertex weights contain NaN".to_string());
            }

            if self.joints.is_none() {
                return Err("Vertex has weights but no joints".to_string());
            }
        }

        Ok(())
    }

}