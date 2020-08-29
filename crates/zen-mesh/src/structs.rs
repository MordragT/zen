use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Date {
    year: u32,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}, {}:{}:{}",
            self.day, self.month, self.year, self.hour, self.minute, self.second
        )
    }
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

#[derive(Deserialize, Debug)]
pub struct Chunk {
    pub id: u16,
    pub length: u32,
}
