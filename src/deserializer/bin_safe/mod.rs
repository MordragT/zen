use super::Header;
/// Header for BinSafe file kind
pub struct BinSafeHeader {
    version: u32,
    hash_table_offset: u32,
}

pub struct Bytes<'a> {
    bytes: &'a [u8],
    line: usize,
    column: usize,
}

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            line: 1,
            column: 1,
        }
    }
}
pub struct BinSafeDeserializer<'a> {
    header: Header,
    bin_safe_header: BinSafeHeader,
    bytes: Bytes<'a>,
}
