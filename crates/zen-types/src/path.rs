use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

lazy_static! {
    // pub static ref GOTHIC_ONE_INSTANCE: GothicOne =
    //     GothicOne::new("/home/tom/Steam/common/Gothic/");
    // pub static ref GOTHIC_TWO_INSTANCE: GothicTwo =
    //     GothicTwo::new("/home/tom/Steam/common/Gothic II/");
    /// Holds the information of the instance of the gothic Game
    pub static ref INSTANCE: GameInstance =
        GameInstance::GothicTwo(GothicTwo::new("/home/tom/Steam/common/Gothic II/"));
    /// Holds the information of the instance where to store the exported files
    pub static ref FILES_INSTANCE: Files = Files::new("/home/tom/Git/zen-loader/files/");
}

pub struct Files {
    pub base_path: PathBuf,
    pub animations: PathBuf,
    pub meshes: PathBuf,
    pub sounds: PathBuf,
    pub textures: PathBuf,
    pub worlds: PathBuf,
}

impl Files {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            base_path: path.as_ref().to_owned(),
            animations: path.as_ref().join(Path::new("animations/")),
            meshes: path.as_ref().join(Path::new("meshes/")),
            sounds: path.as_ref().join(Path::new("sounds/")),
            textures: path.as_ref().join(Path::new("textures/")),
            worlds: path.as_ref().join(Path::new("worlds/")),
        }
    }
}

/// Gothic 1 or 2 game instance
pub enum GameInstance {
    GothicOne(GothicOne),
    GothicTwo(GothicTwo),
}

impl GameInstance {
    /// Return path to animations
    pub const fn animations(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.animations,
            GameInstance::GothicTwo(game) => &game.animations,
        }
    }
    /// Return path to meshes
    pub const fn meshes(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.meshes,
            GameInstance::GothicTwo(game) => &game.meshes,
        }
    }
    /// Return path to sounds
    pub const fn sounds(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.sounds,
            GameInstance::GothicTwo(game) => &game.sounds,
        }
    }
    /// Return path to textures
    pub const fn textures(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.textures,
            GameInstance::GothicTwo(game) => &game.textures,
        }
    }
    /// Return path to worlds
    pub const fn worlds(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.worlds,
            GameInstance::GothicTwo(game) => &game.worlds,
        }
    }
}

/// Gothic 1 path informations
pub struct GothicOne {
    pub animations: PathBuf,
    pub meshes: PathBuf,
    pub sounds: PathBuf,
    pub textures: PathBuf,
    pub worlds: PathBuf,
}

impl GothicOne {
    /// Create new gothic 1 path informations based on the gothic 1 path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            animations: path.as_ref().join(Path::new("Data/anims.VDF")),
            meshes: path.as_ref().join(Path::new("Data/meshes.VDF")),
            sounds: path.as_ref().join(Path::new("Data/sound.VDF")),
            textures: path.as_ref().join(Path::new("Data/textures.VDF")),
            worlds: path.as_ref().join(Path::new("Data/worlds.VDF")),
        }
    }
}

/// Gothic 2 path informations
pub struct GothicTwo {
    pub animations: PathBuf,
    pub animations_addon: PathBuf,
    pub meshes: PathBuf,
    pub meshes_addon: PathBuf,
    pub sounds: PathBuf,
    pub sounds_addon: PathBuf,
    pub textures: PathBuf,
    pub textures_addon: PathBuf,
    pub worlds: PathBuf,
    pub worlds_addon: PathBuf,
}

impl GothicTwo {
    /// Create new gothic 2 path informations based on the gothic 2 path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            animations: path.as_ref().join(Path::new("Data/Anims.vdf")),
            animations_addon: path.as_ref().join(Path::new("Data/Anims_Addon.vdf")),
            meshes: path.as_ref().join(Path::new("Data/Meshes.vdf")),
            meshes_addon: path.as_ref().join(Path::new("Data/Meshes_Addon.vdf")),
            sounds: path.as_ref().join(Path::new("Data/Sounds.vdf")),
            sounds_addon: path.as_ref().join(Path::new("Data/Sounds_Addon.vdf")),
            textures: path.as_ref().join(Path::new("Data/Textures.vdf")),
            textures_addon: path.as_ref().join(Path::new("Data/Textures_Addon.vdf")),
            worlds: path.as_ref().join(Path::new("Data/Worlds.vdf")),
            worlds_addon: path.as_ref().join(Path::new("Data/Worlds_Addon.vdf")),
        }
    }
}
