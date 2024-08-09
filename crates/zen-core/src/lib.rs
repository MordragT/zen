pub const GOTHIC1_PATH: &'static str = "/run/media/Media/Programs/Steam/steamapps/common/Gothic";
pub const GOTHIC2_PATH: &'static str = "/run/media/Media/Programs/Steam/steamapps/common/Gothic II";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameKind {
    Gothic1,
    Gothic2,
    Unknown,
}
