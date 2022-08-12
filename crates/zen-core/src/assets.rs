use crate::{
    archive::{ArchiveError, Entry, Vdfs, VdfsKind},
    material::{BasicMaterial, Group, ZenMaterial},
    model::{Vertex, ZenMesh, ZenModel},
    mrm::{Mrm, MrmError},
    msh::{Msh, MshError},
    texture::{TextureError, ZenTexture},
};
use bevy::prelude::{Assets, Handle, Res, ResMut};
use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Error, Debug)]
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

pub struct ZenAssetLoaderBuilder {
    mesh_archives: Vec<File>,
    animation_archives: Vec<File>,
    texture_archives: Vec<File>,
    sound_archives: Vec<File>,
    scene_archives: Vec<File>,
}

impl ZenAssetLoaderBuilder {
    pub fn new() -> Self {
        Self {
            mesh_archives: Vec::with_capacity(5),
            animation_archives: Vec::with_capacity(5),
            texture_archives: Vec::with_capacity(5),
            sound_archives: Vec::with_capacity(10),
            scene_archives: Vec::with_capacity(5),
        }
    }

    pub fn archive<P: AsRef<Path>>(mut self, kind: VdfsKind, path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        match kind {
            VdfsKind::Mesh => self.mesh_archives.push(file),
            VdfsKind::Animation => self.animation_archives.push(file),
            VdfsKind::Texture => self.texture_archives.push(file),
            VdfsKind::Sound => self.sound_archives.push(file),
            VdfsKind::World => self.scene_archives.push(file),
        }
        Ok(self)
    }

    pub fn build(self) -> AssetResult<ZenAssetLoader> {
        let mut meshes = Vec::new();
        for file in self.mesh_archives {
            let vdfs = Vdfs::new(file)?;
            meshes.extend(vdfs.entries()?);
        }

        let mut animations = Vec::new();
        for file in self.animation_archives {
            let vdfs = Vdfs::new(file)?;
            animations.extend(vdfs.entries()?);
        }

        let mut textures = Vec::new();
        for file in self.texture_archives {
            let vdfs = Vdfs::new(file)?;
            textures.extend(vdfs.entries()?);
        }

        let mut sounds = Vec::new();
        for file in self.sound_archives {
            let vdfs = Vdfs::new(file)?;
            sounds.extend(vdfs.entries()?);
        }

        let mut scenes = Vec::new();
        for file in self.scene_archives {
            let vdfs = Vdfs::new(file)?;
            scenes.extend(vdfs.entries()?);
        }

        Ok(ZenAssetLoader {
            meshes,
            animations,
            textures,
            sounds,
            scenes,
        })
    }
}

pub struct ZenAssetLoader {
    meshes: Vec<Entry<File>>,
    animations: Vec<Entry<File>>,
    textures: Vec<Entry<File>>,
    sounds: Vec<Entry<File>>,
    scenes: Vec<Entry<File>>,
}

impl ZenAssetLoader {
    fn load_mrm(
        &mut self,
        mrm: Mrm,
        model_assets: &mut Assets<ZenModel>,
        mesh_assets: &mut Assets<ZenMesh>,
        material_assets: &mut Assets<ZenMaterial>,
        texture_assets: &mut Assets<ZenTexture>,
    ) -> AssetResult<Handle<ZenModel>> {
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

        let model = ZenModel {
            name: mrm.name,
            meshes,
        };

        Ok(model_assets.add(model))
    }

    fn load_msh(
        &mut self,
        msh: Msh,
        model_assets: &mut Assets<ZenModel>,
        mesh_assets: &mut Assets<ZenMesh>,
        material_assets: &mut Assets<ZenMaterial>,
        texture_assets: &mut Assets<ZenTexture>,
    ) -> AssetResult<Handle<ZenModel>> {
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

        let model = ZenModel { name, meshes };

        Ok(model_assets.add(model))
    }

    fn load_material(
        &mut self,
        material: &BasicMaterial,
        material_assets: &mut Assets<ZenMaterial>,
        texture_assets: &mut Assets<ZenTexture>,
    ) -> AssetResult<Handle<ZenMaterial>> {
        let entry = self
            .textures
            .iter_mut()
            .find(|entry| entry.name() == material.name())
            .ok_or(AssetError::NotFound(material.name().clone()))?;

        let texture = ZenTexture::from_ztex(entry, material.name())?;
        let texture = texture_assets.add(texture);

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
    }

    pub fn load_model(
        &mut self,
        name: &str,
        model_assets: ResMut<Assets<ZenModel>>,
        mesh_assets: ResMut<Assets<ZenMesh>>,
        material_assets: ResMut<Assets<ZenMaterial>>,
        texture_assets: ResMut<Assets<ZenTexture>>,
    ) -> AssetResult<Handle<ZenModel>> {
        let entry = self
            .meshes
            .iter_mut()
            .find(|entry| entry.name() == name)
            .ok_or(AssetError::NotFound(name.to_string()))?;

        if entry.name().ends_with(".MRM") {
            let mrm = Mrm::new(entry, name)?;
            self.load_mrm(
                mrm,
                model_assets.into_inner(),
                mesh_assets.into_inner(),
                material_assets.into_inner(),
                texture_assets.into_inner(),
            )
        } else if entry.name().ends_with(".MSH") {
            let msh = Msh::new(entry, name)?;
            self.load_msh(
                msh,
                model_assets.into_inner(),
                mesh_assets.into_inner(),
                material_assets.into_inner(),
                texture_assets.into_inner(),
            )
        } else {
            unreachable!()
        }
    }

    pub fn load_texture(&self, name: &str, textures: Assets<ZenTexture>) -> Handle<ZenTexture> {
        todo!()
    }
}
