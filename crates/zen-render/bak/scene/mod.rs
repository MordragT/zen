use std::io::{self, Seek, SeekFrom};

use self::{
    vob::VobError,
    vob_tree::{VobTree, VobTreeError},
};
use crate::archive::Entry;
use serde::Deserialize;
use thiserror::Error;
use zen_parser::{binary, prelude::BinaryDeserializer};

pub mod vob;
pub mod vob_tree;

#[derive(Debug, Error)]
pub enum ZenSceneError {
    #[error("Texture Binary Error: {0}")]
    Binary(#[from] binary::Error),
    #[error("Vob Error: {0}")]
    Vob(#[from] VobError),
    #[error("Vob Tree: {0}")]
    VobTree(#[from] VobTreeError),
    #[error("Expected: oCWorld:zCWorld, got {0}")]
    ExpectedWorld(String),
    #[error("VobTree not found")]
    VobTreeNotFound,
    #[error("Io Error: {0}")]
    IoError(#[from] io::Error),
}

pub struct ZenScene {
    vob_tree: VobTree,
}

impl<'a> TryFrom<Entry<'a>> for ZenScene {
    type Error = ZenSceneError;

    fn try_from(entry: Entry) -> Result<Self, Self::Error> {
        let mut deserializer = BinaryDeserializer::from(entry);
        let chunk = <Chunk>::deserialize(&mut deserializer)?;

        if &chunk.class_name != "oCWorld:zCWorld" {
            return Err(ZenSceneError::ExpectedWorld(chunk.class_name));
        }

        let mut vob_tree = None;

        while let Ok(chunk) = <Chunk>::deserialize(&mut deserializer) {
            if chunk.object_name == "VobTree" {
                vob_tree = Some(VobTree::try_from(&mut deserializer)?);
            } else {
                log::debug!("Skipping chunk: {chunk:?}");
                deserializer.seek(SeekFrom::Current(chunk.length as i64))?;
            }
        }

        if let Some(vob_tree) = vob_tree {
            Ok(Self { vob_tree })
        } else {
            Err(ZenSceneError::VobTreeNotFound)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Chunk {
    pub length: u32,
    pub version: u16,
    pub index: u32,
    pub object_name: String,
    pub class_name: String,
}
