pub use de::AsciiDeserializer;
pub use error::Error;
pub use read::AsciiRead;
use std::fmt;

pub mod de;
pub mod error;
pub mod read;

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
