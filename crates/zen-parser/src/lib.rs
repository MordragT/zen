pub mod ascii;
//pub mod binsafe;
pub mod binary;
pub mod prelude;

/// Possible filetypes this zen-file can have
#[derive(Debug)]
pub enum Kind {
    Unknown,
    Ascii,
    Binary,
    BinSafe,
}
/// File Header for zen-files
#[derive(Debug)]
pub struct Header {
    version: i32,
    kind: Kind,
    save_game: bool,
    date: Option<String>,
    user: Option<String>,
    object_count: i32,
}

impl Header {
    pub fn new(
        version: i32,
        kind: Kind,
        save_game: bool,
        date: Option<String>,
        user: Option<String>,
        object_count: i32,
    ) -> Self {
        Self {
            version,
            kind,
            save_game,
            date,
            user,
            object_count,
        }
    }
}
