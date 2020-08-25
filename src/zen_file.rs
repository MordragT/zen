/// Possible filetypes this zen-file can have
pub enum Kind {
    Unknown,
    Ascii,
    Binary,
    BinSafe,
}
/// Information about one of the chunks in a zen-file
pub struct ChunkHeader {
    start_position: u32,
    size: u32,
    verison: u16,
    object_id: u32,
    name: String,
    class_name: String,
    create_object: bool,
}

/// File Header for zen-files
pub struct Header {
    version: i32,
    kind: Kind,
    save_game: bool,
    date: String,
    user: String,
    object_count: i32,
}

/// Header for BinSafe file kind
pub struct BinSafeHeader {
    version: u32,
    hash_table_offset: u32,
}
