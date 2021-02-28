use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

lazy_static! {
    // pub static ref GOTHIC_ONE_INSTANCE: GothicOne =
    //     GothicOne::new("/home/tom/Steam/common/Gothic/");
    // pub static ref GOTHIC_TWO_INSTANCE: GothicTwo =
    //     GothicTwo::new("/home/tom/Steam/common/Gothic II/");
    pub static ref INSTANCE: GameInstance =
        GameInstance::GothicTwo(GothicTwo::new("/home/tom/Steam/common/Gothic II/"));
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

pub enum GameInstance {
    GothicOne(GothicOne),
    GothicTwo(GothicTwo),
}

impl GameInstance {
    pub const fn animations(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.animations,
            GameInstance::GothicTwo(game) => &game.animations,
        }
    }
    pub const fn meshes(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.meshes,
            GameInstance::GothicTwo(game) => &game.meshes,
        }
    }
    pub const fn sounds(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.sounds,
            GameInstance::GothicTwo(game) => &game.sounds,
        }
    }
    pub const fn textures(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.textures,
            GameInstance::GothicTwo(game) => &game.textures,
        }
    }
    pub const fn worlds(&self) -> &PathBuf {
        match self {
            GameInstance::GothicOne(game) => &game.worlds,
            GameInstance::GothicTwo(game) => &game.worlds,
        }
    }
}

pub struct GothicOne {
    pub animations: PathBuf,
    pub meshes: PathBuf,
    pub sounds: PathBuf,
    pub textures: PathBuf,
    pub worlds: PathBuf,
}

impl GothicOne {
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
