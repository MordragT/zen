use crate::error::*;
use std::{fs::File, io::Read, path::Path};
use zen_model::Model;

pub enum Asset {
    Model(Model),
}

impl From<Model> for Asset {
    fn from(model: Model) -> Asset {
        Asset::Model(model)
    }
}

/// Helps to reduce boilerplate for loading different assets from files
pub trait AssetLoader {
    type Error: std::error::Error + 'static;

    /// Load an asset from bytes
    fn load(data: &[u8], name: &str) -> std::result::Result<Asset, Self::Error>;

    /// Returns the extensions used by the asset e.g. .jpg, .jpeg
    fn extensions() -> &'static [&'static str];

    /// Load an asset from a file path
    fn load_from_file(file: impl AsRef<Path>) -> Result<Asset> {
        let path = file.as_ref();
        if let Some(Some(file_name)) = path.file_name().map(|s| s.to_str()) {
            if let Some((name, ext)) = file_name.rsplit_once(".") {
                if Self::extensions().contains(&ext) {
                    let mut file = File::open(path)?;
                    let mut buf = Vec::new();
                    file.read_to_end(&mut buf)?;
                    let asset = Self::load(&buf, name)
                        .map_err(|err| Error::AssetLoaderError(Box::new(err)))?;
                    Ok(asset)
                } else {
                    Err(Error::WrongFileExtension(
                        extensions_to_string(Self::extensions()),
                        ext.to_owned(),
                    ))
                }
            } else {
                Err(Error::ExpectedFileExtension)
            }
        } else {
            Err(Error::ExpectedFile(path.into()))
        }
    }
}

fn extensions_to_string(extensions: &[&str]) -> String {
    extensions.iter().fold(String::new(), |mut init, ext| {
        init.push_str(ext);
        init.push(' ');
        init
    })
}
