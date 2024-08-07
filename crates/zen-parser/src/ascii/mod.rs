pub use de::AsciiDeserializer;
pub use error::{AsciiError, AsciiErrorCode, AsciiResult};
pub use read::AsciiRead;
use std::fmt;

mod de;
mod error;
mod read;

/// Position of the Error
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
