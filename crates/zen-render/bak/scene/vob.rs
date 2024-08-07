use std::str::FromStr;
use serde::Deserialize;
use bevy::prelude::{Mat3, Vec2, Vec3, Transform};
use serde_repr::Deserialize_repr;
use thiserror::Error;
use zen_parser::{prelude::{BinaryDeserializer, BinaryRead}, binary};

use crate::{get_version, GameVersion, impl_try_from_repr, EnumConversionError};

use super::Chunk;

#[derive(Error, Debug)]
pub enum VobError {
    #[error("Texture Binary Error: {0}")]
    Binary(#[from] binary::Error),
    #[error("Enum Conversion Error: {0}")]
    EnumConversion(#[from] EnumConversionError)
}

pub struct VobBundle {
    transform: Transform,

}

pub struct VobData {
    bounding_box: (Vec3, Vec3),
    position: Vec3,
    rotation: Mat3,
    show_visual: bool,
    camera_alignment: CameraLockMode,
    cd_static: bool,
    cd_dynamic: bool,
    vob_static: bool,
    dynamic_shadows: ShadowMode,
    physics_enabled: bool,

    preset_name: Option<String>,
    vob_name: Option<String>,
    visual_name: Option<String>,

    associated_visual_type: Option<VisualType>,
    visual_decal: Option<Decal>,
    vob_ext: Option<VobDataExt>,
}

pub struct VobDataExt {
    anim_mode: AnimationMode,
    bias: i32,
    ambient: bool,
    anim_strength: f32,
    far_clip_scale: f32,
}

impl<'a, R: BinaryRead> TryFrom<&mut BinaryDeserializer<R>> for VobData {
    type Error = VobError;

    fn try_from(deserializer: &mut BinaryDeserializer<R>) -> Result<Self, Self::Error> {
        let packed = i32::deserialize(deserializer)? != 0;
        let mut has_visual_object = true;
        let mut has_ai_object = true;

		let mut vob = if packed {
            let bounding_box = <(Vec3, Vec3)>::deserialize(deserializer)?;
            let position = Vec3::deserialize(deserializer)?;
            let rotation = Mat3::deserialize(deserializer)?; // 			obj.rotation = glm::transpose(bin.get_mat3x3());

            let bit0 = u8::deserialize(deserializer)?;
			let bit1 = if get_version() == GameVersion::Gothic1 {
				u32::deserialize(deserializer)? as u16
			} else {
				u16::deserialize(deserializer)?
			};

			let show_visual = (bit0 & 0b00000001 >> 0) != 0;
			let camera_alignment = (bit0 & 0b00000110 >> 1).try_into()?;
			let cd_static = (bit0 & 0b00001000 >> 3) != 0;
			let cd_dynamic = (bit0 & 0b00010000 >> 4) != 0;
			let vob_static = (bit0 & 0b00100000 >> 5) != 0;
			let dynamic_shadows = (bit0 & 0b11000000 >> 6).try_into()?;

			let has_preset_name = (bit1 & 0b000000000000001 >> 0) != 0;
			let has_vob_name = (bit1 & 0b000000000000010 >> 1) != 0;
			let has_visual_name = (bit1 & 0b000000000000100 >> 2) != 0;
			has_visual_object = (bit1 & 0b000000000001000 >> 3) != 0;
			has_ai_object = (bit1 & 0b000000000010000 >> 4) != 0;
			// let has_event_man_object = (bit1 & 0b000000000100000 >> 5) != 0;
			let physics_enabled = (bit1 & 0b000000001000000 >> 6) != 0;

			let vob_ext = if get_version() == GameVersion::Gothic2 {
				let anim_mode = ((bit1 & 0b000000110000000 >> 7) as u8).try_into()?;
				let bias = (bit1 & 0b011111000000000 >> 9) as i32;
				let ambient = (bit1 & 0b100000000000000 >> 14) != 0;

			    let anim_strength = f32::deserialize(deserializer)?;
				let far_clip_scale = f32::deserialize(deserializer)?;

                Some(VobDataExt { anim_mode, bias, ambient, anim_strength, far_clip_scale })
			} else {
                None
            };

			let preset_name = if has_preset_name { Some(String::deserialize(deserializer)?) } else { None };
            let vob_name = if has_vob_name { Some(String::deserialize(deserializer)?) } else { None };
            let visual_name = if has_visual_name { Some(String::deserialize(deserializer)?) } else { None };

            VobData {
                bounding_box,
                position,
                rotation,
                show_visual,
                camera_alignment,
                cd_static,
                cd_dynamic,
                vob_static,
                dynamic_shadows,
                physics_enabled,
                vob_ext,
                preset_name,
                vob_name,
                visual_name,
                associated_visual_type: None,
                visual_decal: None,
            }
		} else {
			let preset_name = Some(String::deserialize(deserializer)?);
			let bounding_box = <(Vec3, Vec3)>::deserialize(deserializer)?;

			let rotation = Mat3::deserialize(deserializer)?;
			let position = Vec3::deserialize(deserializer)?;

			let vob_name = Some(String::deserialize(deserializer)?);
			let visual_name = Some(String::deserialize(deserializer)?);
			let show_visual = bool::deserialize(deserializer)?;
			let camera_alignment = CameraLockMode::deserialize(deserializer)?;

            let mut vob_ext = if get_version() == GameVersion::Gothic2 {
				let anim_mode = AnimationMode::deserialize(deserializer)?;
				let anim_strength = f32::deserialize(deserializer)?;
				let far_clip_scale = f32::deserialize(deserializer)?;
                Some(VobDataExt {
                    anim_mode,
                    anim_strength,
                    far_clip_scale,
                    bias: 0,
                    ambient: false,
                })
            } else { None };

            let cd_static = bool::deserialize(deserializer)?;
            let cd_dynamic = bool::deserialize(deserializer)?;
            let vob_static = bool::deserialize(deserializer)?;
            let dynamic_shadows = ShadowMode::deserialize(deserializer)?;

            if let Some(vob_ext) = vob_ext {
                vob_ext.bias = i32::deserialize(deserializer)?;
                vob_ext.ambient = bool::deserialize(deserializer)?;
            }

            VobData {
                preset_name,
                bounding_box,
                rotation,
                position,
                vob_name,
                visual_name,
                show_visual,
                camera_alignment,
                vob_ext,
                cd_static,
                cd_dynamic,
                vob_static,
                dynamic_shadows,
                physics_enabled: false,
                associated_visual_type: None,
                visual_decal: None,
            }
		};

		if has_visual_object {
            let chunk = Chunk::deserialize(deserializer)?;
			let associated_visual_type = VisualType::from_str(&chunk.class_name)?;

			let decal = if associated_visual_type == VisualType::Decal {
				obj.visual_decal = decal::parse(in, version);
			} else { None };

			if (!in.read_object_end()) {
				PX_LOGW("vob_tree: visual \"{}\" not fully parsed", visual.class_name);
				in.skip_object(true);
			}
		}

		if has_ai_object {
			in.skip_object(false);
		}
    }
}

#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CameraLockMode {
    /// The camera is not affected by this Vob.
    None,
    /// The camera's yaw is locked to the Vob's yaw.
    Yaw,
    /// The camera is fully locked to the VOb.
    Full,
}
impl_try_from_repr!(u8, CameraLockMode);

#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ShadowMode {
    /// The Vob does not cast any shadow.
    None,
    /// The Vob casts a basic dark circle at its base.
    Blob,
}
impl_try_from_repr!(u8, ShadowMode);


#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AnimationMode {
    None,
    Wind,
    Wind2,
}
impl_try_from_repr!(u8, AnimationMode);


#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum VisualType {
    Decal,
    Msh,
    Mrm,
    ParticleSystem,
    AiCamera,
    Mdl,
    Mmb,
    Unknown,
}
impl_try_from_repr!(u8, VisualType);

impl FromStr for VisualType {
    type Err = EnumConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let visual_type = match s {
             "zCDecal"  => Self::Decal,
             "zCMesh"  => Self::Msh,
             "zCProgMeshProto"  => Self::Mrm,
             "zCParticleFX"  => Self::ParticleSystem,
             "zCModel"  => Self::Mdl,
             "zCAICamera"  => Self::AiCamera,
             "zCMorphMesh"  => Self::Mmb,
             _ => Self::Unknown,
        };
        Ok(visual_type)
    }
}


#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AlphaFunction {
    Default,
    None,
    Blend,
    Add,
    Sub,
    Mul,
    Mul2,
}
impl_try_from_repr!(u8, AlphaFunction);


pub struct Decal {
    name: String,
    dimension: Vec2,
    offset: Vec2,
    two_sided: bool,
    alpha_func: AlphaFunction,
    texture_anim_fps: f32,
    alpha_weight: u8,
    ignore_daylight: bool,
}

/// Virtual object kind
pub enum VobKind {
    /// The base type for all virtual objects.
    Vob,
    /// A basic Vob used for grouping other Vobs.      
    VobLevelCompo,
    /// A Vob representing an item
    Item,
    MoverController,
    VobScreenFX,
    VobStair,
    PFXController,
    VobAnimate,
    VobLensFlare,
    VobLight,
    VobSpot,
    VobStartpoint,
    MessageFilter,
    CodeMaster,
    TriggerWorldStart,
    CSCamera,
    CamTrjKeyFrame,
    TouchDamage,
    TriggerUntouch,
    Earthquake,
    /// The base VOb type used for dynamic world objects.
    Mob,
    /// The base VOb type used for interactive world objects.      
    MobInter,
    /// A bed the player can sleep in.    
    MobBed,
    /// A campfire the player can cook things on.  
    MobFire,
    /// A ladder the player can climb.    
    MobLadder,
    /// A switch or button the player can use.
    MobSwitch,
    /// A grindstone the player can sharpen their weapon with.    
    MobWheel,
    /// A container the player can open.  
    MobContainer,
    /// A door the player can open.
    MobDoor,
    /// The base VOb type used for all kinds of triggers.
    Trigger,
    /// A collection of multiple triggers.         
    TriggerList,
    /// A trigger for calling a script function.  
    TriggerScript,
    /// A trigger for changing the game world.  
    TriggerChangeLevel,
    /// A cutscene trigger.
    TriggerCutScene,
    Mover,
    /// A Vob which emits a certain sound.
    VobSound,
    /// A Vob which emits a sound only during a specified time.     
    VobSoundDaytime,
    /// A VOb which plays music from the soundtrack.
    ZoneMusic,
    ZoneMusicDefault,
    /// A Vob which indicates a foggy area.
    ZoneZFog,
    ZoneZFogDefault,

    ZoneVobFarPlane,
    ZoneVobFarPlaneDefault,
    Unknown,
}

impl FromStr for VobKind {
    type Err = VobError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "zCVob" => VobKind::Vob,
            "zCVobLevelCompo:zCVob" => VobKind::VobLevelCompo,
            "oCItem:zCVob" => VobKind::Item,
            "oCMOB:zCVob" => VobKind::Mob,
            "oCMobInter:oCMOB:zCVob" => VobKind::MobInter,
            "oCMobBed:oCMobInter:oCMOB:zCVob" => VobKind::MobBed,
            "oCMobFire:oCMobInter:oCMOB:zCVob" => VobKind::MobFire,
            "oCMobLadder:oCMobInter:oCMOB:zCVob" => VobKind::MobLadder,
            "oCMobSwitch:oCMobInter:oCMOB:zCVob" => VobKind::MobSwitch,
            "oCMobWheel:oCMobInter:oCMOB:zCVob" => VobKind::MobWheel,
            "oCMobContainer:oCMobInter:oCMOB:zCVob" => VobKind::MobContainer,
            "oCMobDoor:oCMobInter:oCMOB:zCVob" => VobKind::MobDoor,
            "zCPFXControler:zCVob" => VobKind::PFXController,
            "zCVobAnimate:zCVob" => VobKind::VobAnimate,
            "zCVobLensFlare:zCVob" => VobKind::VobLensFlare,
            "zCVobLight:zCVob" => VobKind::VobLight,
            "zCVobSpot:zCVob" => VobKind::VobSpot,
            "zCVobStartpoint:zCVob" => VobKind::VobStartpoint,
            "zCVobSound:zCVob" => VobKind::VobSound,
            "zCVobSoundDaytime:zCVobSound:zCVob" => VobKind::VobSoundDaytime,
            "oCZoneMusic:zCVob" => VobKind::ZoneMusic,
            "oCZoneMusicDefault:oCZoneMusic:zCVob" => VobKind::ZoneMusicDefault,
            "zCZoneZFog:zCVob" => VobKind::ZoneZFog,
            "zCZoneZFogDefault:zCZoneZFog:zCVob" => VobKind::ZoneZFogDefault,
            "zCZoneVobFarPlane:zCVob" => VobKind::ZoneVobFarPlane,
            "zCZoneVobFarPlaneDefault:zCZoneVobFarPlane:zCVob" => VobKind::ZoneVobFarPlaneDefault,
            "zCMessageFilter:zCVob" => VobKind::MessageFilter,
            "zCCodeMaster:zCVob" => VobKind::CodeMaster,
            "zCTrigger:zCVob" => VobKind::Trigger,
            "zCTriggerList:zCTrigger:zCVob" => VobKind::TriggerList,
            "oCTriggerScript:zCTrigger:zCVob" => VobKind::TriggerScript,
            "zCMover:zCTrigger:zCVob" => VobKind::Mover,
            "oCTriggerChangeLevel:zCTrigger:zCVob" => VobKind::TriggerChangeLevel,
            "zCTriggerWorldStart:zCVob" => VobKind::TriggerWorldStart,
            "zCTriggerUntouch:zCVob" => VobKind::TriggerUntouch,
            "zCCSCamera:zCVob" => VobKind::CSCamera,
            "zCCamTrj_KeyFrame:zCVob" => VobKind::CamTrjKeyFrame,
            "oCTouchDamage:zCTouchDamage:zCVob" => VobKind::TouchDamage,
            "zCEarthquake:zCVob" => VobKind::Earthquake,
            "zCMoverControler:zCVob" => VobKind::MoverController,
            "zCVobScreenFX:zCVob" => VobKind::VobScreenFX,
            "zCVobStair:zCVob" => VobKind::VobStair,
            "oCCSTrigger:zCTrigger:zCVob" => VobKind::TriggerCutScene,
            //"\xa7"=> VobKind::Unknown, // some sort of padding object, probably. seems to be always empty
            _ => VobKind::Unknown,
        };
        Ok(kind)
    }
}
