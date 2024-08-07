use std::{collections::HashMap, fmt, sync::Arc};

/// Vdfs Entry
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdfsEntry {
    pub name: Arc<str>,
    pub index: u32,
    pub offset: u32,
    pub size: u32,
    pub kind: u32,
    pub attr: u32,
}

pub(crate) type VdfsEntries = HashMap<Arc<str>, VdfsEntry>;

impl fmt::Display for VdfsEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            index,
            offset,
            size,
            kind,
            attr,
        } = self;

        write!(
            f,
            "{name} {{ index: {index}, offset: {offset}, size: {size}, kind: {kind}, attr: {attr} }}"
        )
    }
}
