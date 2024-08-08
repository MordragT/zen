use core::{fmt, str};
use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use zen_parser::binary::{BinaryDecoder, BinaryRead};

use crate::{
    entry::{VdfsEntries, VdfsEntry},
    error::{VdfsError, VdfsResult},
};

const SIGNATURE_LENGTH: usize = 16;

/// Vdfs Header attributes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub(crate) struct VdfsHeader {
    pub signature: [u8; SIGNATURE_LENGTH],
    pub count: u32,
    pub num_files: u32,
    pub timestamp: u32,
    pub data_size: u32,
    pub offset: u32,
    pub version: u32,
}

impl fmt::Display for VdfsHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VDFS Archive:\n\tSize: {}\n\tTime: {}\n\tOffset: {}\n\tNumber Files: {}",
            self.data_size, self.timestamp, self.offset, self.num_files
        )?;

        Ok(())
    }
}

impl VdfsHeader {
    const SUPPORTED_VERSION: u32 = 0x50;

    const SIGNATURE_G1: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\r\n\r\n";
    const SIGNATURE_G2: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\n\r\n\r";

    const ENTRY_NAME_LENGTH: usize = 64;
    const ENTRY_DIR: u32 = 0x80000000;

    pub(crate) fn validate(&self) -> VdfsResult<()> {
        if self.signature != Self::SIGNATURE_G1 && self.signature != Self::SIGNATURE_G2 {
            return Err(VdfsError::UnknownSignature);
        }

        if self.version != Self::SUPPORTED_VERSION {
            return Err(VdfsError::UnsupportedVersion);
        }

        Ok(())
    }

    pub(crate) fn read_entries<R>(&self, decoder: &mut BinaryDecoder<R>) -> VdfsResult<VdfsEntries>
    where
        R: BinaryRead,
    {
        let mut entries = HashMap::new();

        decoder.set_position(self.offset as u64)?;

        for index in 0..self.count {
            let mut name_buf = [0_u8; Self::ENTRY_NAME_LENGTH];
            decoder.read_bytes(&mut name_buf)?;

            let end = name_buf
                .iter()
                .position(|c| {
                    !(*c >= b'A' && *c <= b'Z'
                        || *c == b'_'
                        || *c == b'.'
                        || *c == b'-'
                        || *c >= b'0' && *c <= b'9')
                })
                .unwrap_or(Self::ENTRY_NAME_LENGTH - 1);
            let name: Arc<str> = str::from_utf8(&name_buf[..end])
                .expect("name should be valid ascii")
                .into();

            let offset = decoder.decode::<u32>()?;
            let size = decoder.decode::<u32>()?;
            let kind = decoder.decode::<u32>()?;
            let attr = decoder.decode::<u32>()?;

            if kind & Self::ENTRY_DIR != 0 {
                // return Err(VdfsError::UnknownEntryKind(kind));
                continue;
            }

            entries.insert(
                name.clone(),
                VdfsEntry {
                    name,
                    index,
                    offset,
                    size,
                    kind,
                    attr,
                },
            );
        }

        Ok(entries)
    }
}
