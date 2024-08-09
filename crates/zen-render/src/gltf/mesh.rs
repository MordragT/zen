use gltf_json as json;

use super::{primitive::GltfPrimitive, GltfBuilder};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GltfMesh {
    pub primitives: Vec<GltfPrimitive>,
}

impl GltfBuilder {
    pub fn insert_mesh(&mut self, mesh: GltfMesh) -> json::Index<json::Mesh> {
        // let byte_length = self.push_buffer(mesh.primitives);
        todo!()
    }
}
