use crate::{
    archive::{ArchiveError, Entry, Vdfs, VdfsKind},
    material::{BasicMaterial, Group, ZenMaterial},
    model::{Vertex, ZenMesh, ZenModel},
    mrm::{Mrm, MrmError},
    msh::{Msh, MshError},
    texture::{TextureError, ZenTexture},
};
use bevy::{
    ecs::{system::EntityCommands, world::EntityMut},
    prelude::{
        Assets, BuildChildren, BuildWorldChildren, Bundle, Color, Commands, ComputedVisibility,
        Entity, Handle, Image, Mesh, PbrBundle, Res, ResMut, StandardMaterial, Transform, World,
        WorldChildBuilder,
    },
    render::view::NoFrustumCulling,
};
use miette::Diagnostic;
use std::{
    default,
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

pub struct ZenLoadContext<'a> {
    pub meshes: &'a mut Assets<ZenMesh>,
    pub materials: &'a mut Assets<ZenMaterial>,
    pub textures: &'a mut Assets<ZenTexture>,
}

impl<'a> ZenLoadContext<'a> {
    pub fn new(
        meshes: &'a mut Assets<ZenMesh>,
        materials: &'a mut Assets<ZenMaterial>,
        textures: &'a mut Assets<ZenTexture>,
    ) -> Self {
        Self {
            meshes,
            materials,
            textures,
        }
    }

    pub fn add_mesh(&mut self, mesh: ZenMesh) -> Handle<ZenMesh> {
        self.meshes.add(mesh)
    }

    pub fn get_mesh(&self, handle: &Handle<ZenMesh>) -> &ZenMesh {
        self.meshes.get(handle).expect("Should be present")
    }

    pub fn remove_mesh(&mut self, handle: &Handle<ZenMesh>) -> ZenMesh {
        self.meshes.remove(handle).expect("Should be present")
    }

    pub fn add_material(&mut self, material: ZenMaterial) -> Handle<ZenMaterial> {
        self.materials.add(material)
    }

    pub fn get_material(&self, handle: &Handle<ZenMaterial>) -> &ZenMaterial {
        self.materials.get(handle).expect("Should be present")
    }

    pub fn remove_material(&mut self, handle: &Handle<ZenMaterial>) -> ZenMaterial {
        self.materials.remove(handle).expect("Should be present")
    }

    pub fn add_texture(&mut self, texture: ZenTexture) -> Handle<ZenTexture> {
        self.textures.add(texture)
    }

    pub fn get_texture(&self, handle: &Handle<ZenTexture>) -> &ZenTexture {
        self.textures.get(handle).expect("Should be present")
    }

    pub fn remove_texture(&mut self, handle: &Handle<ZenTexture>) -> ZenTexture {
        self.textures.remove(handle).expect("Should be present")
    }
}

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

    fn load_mrm(&self, mrm: Mrm, context: &mut ZenLoadContext<'_>) -> AssetResult<ZenModel> {
        let Mrm {
            name,
            vertices,
            sub_meshes,
            ..
        } = mrm;
        log::debug!("Loading MRM: {name}");

        let children = sub_meshes
            .into_iter()
            .enumerate()
            .map(|(index, sub_mesh)| {
                let indices = sub_mesh
                    .triangles
                    .into_iter()
                    .map(|v| v.to_array())
                    .flatten()
                    .map(|pos| pos as u32)
                    .collect::<Vec<u32>>();

                let material = Some(self.load_material(&sub_mesh.material, context)?);

                let mut mesh = sub_mesh.wedges.into_iter().fold(
                    ZenMesh {
                        vertices: Vec::new(),
                        indices,
                    },
                    |mut mesh, wedge| {
                        let vertex = Vertex {
                            position: vertices[wedge.vertex_index as usize].to_array(),
                            // flip uvs ?
                            tex_coords: wedge.tex_coord.to_array(), //.map(|x| -x),
                            // flip normals ?
                            normal: wedge.normal.to_array(), //.map(|x| -x),
                        };
                        mesh.vertices.push(vertex);
                        mesh
                    },
                );

                //mesh.scale(0.02);

                //let mesh = mesh.pack();
                let mesh = Some(context.add_mesh(mesh));

                Ok(ZenModel {
                    name: format!("{name}-{index}"),
                    children: vec![],
                    mesh,
                    material,
                    transform: Transform::default(),
                })
            })
            .collect::<AssetResult<Vec<ZenModel>>>()?;

        let model = ZenModel {
            name,
            children,
            mesh: None,
            material: None,
            transform: Transform::default(),
        };

        Ok(model)
    }

    fn load_msh(&self, msh: Msh, context: &mut ZenLoadContext<'_>) -> AssetResult<ZenModel> {
        log::debug!("Loading MSH: {}", &msh.name);
        let Msh {
            name,
            materials,
            vertices,
            features,
            polygons,
        } = msh;

        let children = polygons
            .into_iter()
            .enumerate()
            .map(|(index, polygon)| {
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
                let material = Some(self.load_material(material, context)?);

                let num_elements = (vertices.len() / 3) as u32;
                let mesh = ZenMesh { vertices, indices };

                let mesh = Some(context.add_mesh(mesh));
                let model = ZenModel {
                    name: format!("{name}-{index}"),
                    children: vec![],
                    mesh,
                    material,
                    transform: Transform::default(),
                };

                Ok(model)
            })
            .collect::<AssetResult<Vec<ZenModel>>>()?;

        let model = ZenModel {
            name,
            children,
            mesh: None,
            material: None,
            transform: Transform::default(),
        };
        Ok(model)
    }

    fn load_material(
        &self,
        material: &BasicMaterial,
        context: &mut ZenLoadContext<'_>,
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
            let texture = entry.try_into()?;
            let texture = context.add_texture(texture);

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
            Ok(context.add_material(material))
        } else {
            Err(AssetError::NotFound(material.name().to_owned()))
        }
    }

    pub fn load_model(
        &self,
        name: &str,
        context: &mut ZenLoadContext<'_>,
    ) -> AssetResult<ZenModel> {
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
                self.load_mrm(mrm, context)
            } else if entry.name().ends_with(".MSH") {
                let msh = Msh::new(entry, name)?;
                log::debug!("Entry read into MSH");
                self.load_msh(msh, context)
            } else {
                unreachable!()
            }
        } else {
            Err(AssetError::NotFound(name.to_string()))
        }
    }

    fn spawn_model_with(
        &self,
        entity: &mut EntityCommands,
        model: ZenModel,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        textures: &mut Assets<Image>,
        context: &mut ZenLoadContext<'_>,
    ) {
        let ZenModel {
            children,
            mesh,
            material,
            transform,
            ..
        } = model;

        entity.with_children(|parent| {
            let mesh = meshes.add(context.get_mesh(&mesh.unwrap()).clone().into());
            parent
                .spawn()
                .insert_bundle(PbrBundle {
                    mesh,
                    material: materials.add(StandardMaterial {
                        double_sided: true,
                        cull_mode: None,
                        base_color: Color::ORANGE_RED,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert(NoFrustumCulling);

            // let mut entity = parent.spawn();
            // entity.insert(transform);

            // if let Some(handle) = mesh {
            //     let mesh = context.remove_mesh(&handle).into();
            //     let handle = meshes.add(mesh);
            //     entity.insert(handle);
            // }

            // if let Some(handle) = material {
            // let material = context.remove_material(&handle);
            // let texture = context.remove_texture(&material.texture).into();
            // let texture_handle = textures.add(texture);

            // let material = StandardMaterial {
            //     double_sided: true,
            //     cull_mode: None,
            //     base_color: material.color,
            //     base_color_texture: Some(texture_handle),
            //     metallic: material.metallic,
            //     perceptual_roughness: material.roughness,
            //     reflectance: material.reflectance,
            //     ....Default::default()
            // };
            // let material_handle = materials.add(material);

            //     entity.insert(material_handle);
            // }

            // for child in children {
            //     self.spawn_model_with(&mut entity, child, meshes, context);
            // }
        });
    }

    pub fn spawn_model(
        &self,
        model: ZenModel,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        textures: &mut Assets<Image>,
        context: &mut ZenLoadContext<'_>,
        commands: &mut Commands,
    ) -> AssetResult<Entity> {
        let ZenModel {
            children,
            mesh,
            material,
            transform,
            ..
        } = model;

        let mut entity = commands.spawn();
        entity.insert(transform);

        if let Some(handle) = mesh {
            let mesh = context.get_mesh(&handle).clone().into();
            let handle = meshes.add(mesh);
            entity.insert(handle);
        }

        if let Some(handle) = material {
            entity.insert(handle);
        }

        for child in children {
            self.spawn_model_with(&mut entity, child, meshes, materials, textures, context);
        }

        Ok(entity.id())
    }
}
