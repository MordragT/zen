use serde::Deserialize;

pub mod de;
// use super::Header;
/// Header for BinSafe files
#[derive(Debug, Deserialize)]
pub struct BinSafeHeader {
    pub version: u32,
    pub object_count: u32,
    pub hash_table_offset: u32,
    pub _a: u16,
    pub _b: u8,
}

// pub struct Bytes<'a> {
//     bytes: &'a [u8],
//     line: usize,
//     column: usize,
// }

// impl<'a> Bytes<'a> {
//     pub fn new(bytes: &'a [u8]) -> Self {
//         Self {
//             bytes,
//             line: 1,
//             column: 1,
//         }
//     }
// }
// pub struct BinSafeDeserializer<'a> {
//     header: Header,
//     bin_safe_header: BinSafeHeader,
//     bytes: Bytes<'a>,
// }
