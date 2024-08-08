/// Possible filetypes this vdfs-file can have
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArchiveKind {
    Unknown,
    Ascii,
    Binary,
    BinSafe,
}

/// File Header for zen-files
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchiveHeader {
    pub version: i32,
    pub kind: ArchiveKind,
    pub save_game: bool,
    pub date: Option<String>,
    pub user: Option<String>,
    pub object_count: i32,
}
