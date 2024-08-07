use std::str::FromStr;

use serde::Deserialize;
use thiserror::Error;
use zen_parser::{
    binary,
    prelude::{BinaryDeserializer, BinaryRead},
};

use crate::{
    archive::Entry,
    vob::{VobError, VobKind},
};

use super::Chunk;

#[derive(Debug, Error)]
pub enum VobTreeError {
    #[error("Texture Binary Error: {0}")]
    Binary(#[from] binary::Error),
    #[error("Vob Error: {0}")]
    Vob(#[from] VobError),
}

pub struct VobTree {}

impl<'a, R: BinaryRead> TryFrom<&'a mut BinaryDeserializer<R>> for VobTree {
    type Error = VobTreeError;

    fn try_from(deserializer: &mut BinaryDeserializer<R>) -> Result<Self, Self::Error> {
        let chunk = <Chunk>::deserialize(&mut deserializer)?;
        let kind = VobKind::from_str(&chunk.class_name)?;

        //     match kind {
        // Vob,
        // VobLevelCompo,
        // Item,
        // MoverController,
        // VobScreenFX,
        // VobStair,
        // PFXController,
        // VobAnimate,
        // VobLensFlare,
        // VobLight,
        // VobSpot,
        // VobStartpoint,
        // MessageFilter,
        // CodeMaster,
        // TriggerWorldStart,
        // CSCamera,
        // CamTrjKeyFrame,
        // TouchDamage,
        // TriggerUntouch,
        // Earthquake,
        // Mob,
        // MobInter,
        // MobBed,
        // MobFire,
        // MobLadder,
        // MobSwitch,
        // MobWheel,
        // MobContainer,
        // MobDoor,
        // Trigger,
        // TriggerList,
        // TriggerScript,
        // TriggerChangeLevel,
        // TriggerCutScene,
        // Mover,
        // VobSound,
        // VobSoundDaytime,
        // ZoneMusic,
        // ZoneMusicDefault,
        // ZoneZFog,
        // ZoneZFogDefault,
        // ZoneVobFarPlane,
        // ZoneVobFarPlaneDefault,
        // Unknown,
        //     }

        todo!()
    }
}
