pub use error::Error;
use error::Result;
use zen_core::{Asset, AssetLoader};
use zen_material::Material;
use zen_model::{Mesh, Model, Vertex};

pub struct ObjLoader;

mod error;

impl AssetLoader for ObjLoader {
    type Error = Error;
    fn load(data: &[u8], name: &str) -> Result<Asset> {
        let cursor = Cursor::new(data);
        let obj = Obj::new(cursor, name)?;
        let model = Model::try_from(obj)?;
        Ok(Asset::Model(model))
    }

    fn extensions() -> &'static [&'static str] {
        &["mrm"]
    }
}

impl TryFrom<Obj> for Model {
    type Error = Error;

    fn try_from(obj: Obj) -> Result<Model> {
        obj.meshes.into_iter().map(|m| {
            let mesh = m.mesh;
            for 
        })
    }
}

pub struct Obj {
    meshes: Vec<tobj::Model>,
    materials: Vec<tobj::Materials>,
}

impl Obj {
    pub fn load<P: AsRef<Path>>(file: P) -> Result<Self> {
        let result = tobj::load_obj(
            file,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )?;
        let meshes = result.0;
        let materials = result.1?;
        Ok(Self { meshes, materials })
    }
}
