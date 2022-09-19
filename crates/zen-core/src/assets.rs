use crate::{
    archive::{ArchiveError, Entry, Vdfs, VdfsKind},
    material::{BasicMaterial, Group, ZenMaterial},
    model::{Vertex, ZenMesh, ZenModel, ZenModelBundle},
    mrm::{Mrm, MrmError},
    msh::{Msh, MshError},
    texture::{TextureError, ZenTexture},
};
use bevy::prelude::{Assets, Handle, Image, Res, ResMut};
use miette::Diagnostic;
use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Diagnostic, Error, Debug)]
pub enum AssetError {
    #[error("Asset Dir Error: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("Asset IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Asset Archive Error: {0}")]
    Archive(#[from] ArchiveError),
    #[error("Empty Archive: {0:?}")]
    EmptyArchive(PathBuf),
    #[error("Asset not found: {0}")]
    NotFound(String),
    #[error("Asset MRM Error: {0}")]
    Mrm(#[from] MrmError),
    #[error("Asset MSH Error: {0}")]
    Msh(#[from] MshError),
    #[error("Asset Texture Error: {0}")]
    Texture(#[from] TextureError),
}

type AssetResult<T> = Result<T, AssetError>;

// TODO keep Handles in LoadContext

pub struct ZenAssetLoader {
    meshes: Vec<Vdfs<File>>,
    animations: Vec<Vdfs<File>>,
    textures: Vec<Vdfs<File>>,
    sounds: Vec<Vdfs<File>>,
    scenes: Vec<Vdfs<File>>,
}

impl ZenAssetLoader {
    pub fn new() -> Self {
        Self {
            meshes: Vec::with_capacity(5),
            animations: Vec::with_capacity(5),
            textures: Vec::with_capacity(5),
            sounds: Vec::with_capacity(10),
            scenes: Vec::with_capacity(5),
        }
    }

    pub fn archive<P: AsRef<Path>>(mut self, kind: VdfsKind, path: P) -> AssetResult<Self> {
        let archive = Vdfs::new(File::open(path)?)?;
        match kind {
            VdfsKind::Mesh => self.meshes.push(archive),
            VdfsKind::Animation => self.animations.push(archive),
            VdfsKind::Texture => self.textures.push(archive),
            VdfsKind::Sound => self.sounds.push(archive),
            VdfsKind::World => self.scenes.push(archive),
        }
        Ok(self)
    }

    fn load_mrm(
        &self,
        mrm: Mrm,
        mesh_assets: &mut Assets<ZenMesh>,
        material_assets: &mut Assets<ZenMaterial>,
        texture_assets: &mut Assets<Image>,
    ) -> AssetResult<ZenModel> {
        log::debug!("Loading MRM: {}", &mrm.name);
        let (object_sub_meshes, object_vertices) = (mrm.sub_meshes, mrm.vertices);

        let meshes = object_sub_meshes
            .into_iter()
            .map(|sub_mesh| {
                let indices = sub_mesh
                    .triangles
                    .into_iter()
                    .map(|v| v.to_array())
                    .flatten()
                    .map(|pos| pos as u32)
                    .collect::<Vec<u32>>();

                let material =
                    self.load_material(&sub_mesh.material, material_assets, texture_assets)?;

                let mut mesh = sub_mesh.wedges.into_iter().fold(
                    ZenMesh {
                        vertices: Vec::new(),
                        num_elements: indices.len() as u32,
                        indices,
                        material,
                    },
                    |mut mesh, wedge| {
                        let vertex = Vertex {
                            position: object_vertices[wedge.vertex_index as usize].to_array(),
                            tex_coords: wedge.tex_coord.to_array(),
                            normal: wedge.normal.to_array(),
                        };
                        mesh.vertices.push(vertex);
                        mesh
                    },
                );

                mesh.scale(0.02);

                //let mesh = mesh.pack();

                Ok(mesh_assets.add(mesh))
            })
            .collect::<AssetResult<Vec<Handle<ZenMesh>>>>()?;

        let model = ZenModel { meshes };

        Ok(model)
    }

    fn load_msh(
        &self,
        msh: Msh,
        mesh_assets: &mut Assets<ZenMesh>,
        material_assets: &mut Assets<ZenMaterial>,
        texture_assets: &mut Assets<Image>,
    ) -> AssetResult<ZenModel> {
        log::debug!("Loading MSH: {}", &msh.name);
        let Msh {
            name,
            materials,
            vertices,
            features,
            polygons,
        } = msh;

        let meshes = polygons
            .into_iter()
            .map(|polygon| {
                let vertices = polygon
                    .indices
                    .iter()
                    .map(|index| Vertex {
                        position: vertices[index.vertex as usize].to_array(),
                        tex_coords: features[index.feature as usize].tex_coord.to_array(),
                        normal: features[index.feature as usize].vert_normal.to_array(),
                    })
                    .collect::<Vec<Vertex>>();
                let indices = (0..vertices.len() / 3)
                    .into_iter()
                    .map(|i| i as u32)
                    .collect::<Vec<u32>>();

                let material = &materials[polygon.material_index as usize];
                let material = self.load_material(material, material_assets, texture_assets)?;

                let num_elements = (vertices.len() / 3) as u32;
                let mesh = ZenMesh {
                    vertices,
                    indices,
                    material,
                    num_elements,
                };
                Ok(mesh_assets.add(mesh))
            })
            .collect::<AssetResult<Vec<Handle<ZenMesh>>>>()?;

        let model = ZenModel { meshes };

        Ok(model)
    }

    fn load_material(
        &self,
        material: &BasicMaterial,
        material_assets: &mut Assets<ZenMaterial>,
        texture_assets: &mut Assets<Image>,
    ) -> AssetResult<Handle<ZenMaterial>> {
        log::debug!("Loading material: {}", &material.name());

        let c_tex_name = material.compiled_texture();
        log::debug!("Finding texture: {c_tex_name}");

        let mut entry = None;
        for archive in &self.textures {
            if let Some(e) = archive.entries()?.find(|e| e.name() == &c_tex_name) {
                entry = Some(e);
            }
        }
        if let Some(entry) = entry {
            let texture = ZenTexture::from_ztex(entry, material.name())?;
            let texture = texture_assets.add(texture.into());

            let color = crate::material::to_color(material.color());

            let (metallic, roughness, reflectance) = match material.group() {
                // TODO check if values are fine
                &Group::Undef => (0.0, 0.5, 0.5),
                &Group::Metal => (1.0, 0.25, 0.85),
                &Group::Stone => (0.5, 0.5, 0.6),
                &Group::Wood => (0.0, 0.7, 0.5),
                &Group::Earth => (0.0, 0.9, 0.5),
                &Group::Water => (0.0, 0.5, 0.75),
                &Group::Snow => (0.0, 0.8, 0.5),
            };

            let material = ZenMaterial {
                texture,
                color,
                metallic,
                reflectance,
                roughness,
            };
            Ok(material_assets.add(material))
        } else {
            Err(AssetError::NotFound(material.name().to_owned()))
        }
    }

    pub fn load_model(
        &self,
        name: &str,
        model_assets: &mut Assets<ZenModel>,
        mesh_assets: &mut Assets<ZenMesh>,
        material_assets: &mut Assets<ZenMaterial>,
        texture_assets: &mut Assets<Image>,
    ) -> AssetResult<Handle<ZenModel>> {
        let mut entry = None;
        for archive in &self.meshes {
            log::debug!("Searching {archive}\nfor model {name}");
            if let Some(e) = archive.entries()?.find(|e| e.name() == name) {
                entry = Some(e);
            }
        }
        if let Some(entry) = entry {
            log::debug!("Entry found: {entry}");
            if entry.name().ends_with(".MRM") {
                let mrm = Mrm::new(entry, name)?;
                log::debug!("Entry read into MRM");
                let model = self.load_mrm(mrm, mesh_assets, material_assets, texture_assets)?;
                Ok(model_assets.add(model))
            } else if entry.name().ends_with(".MSH") {
                let msh = Msh::new(entry, name)?;
                log::debug!("Entry read into MSH");
                let model = self.load_msh(msh, mesh_assets, material_assets, texture_assets)?;
                Ok(model_assets.add(model))
            } else {
                unreachable!()
            }
        } else {
            Err(AssetError::NotFound(name.to_string()))
        }
    }

    pub fn load_texture(&self, name: &str, textures: Assets<Image>) -> Handle<Image> {
        todo!()
    }
}
