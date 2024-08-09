use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};

use super::{error::ZTexError, ZTex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZTexLoader {}

impl AssetLoader for ZTexLoader {
    type Asset = Image;
    type Settings = ();
    type Error = ZTexError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let ztex = ZTex::from_bytes(bytes)?;
        let image = Image::try_from(ztex)?;

        Ok(image)
    }

    fn extensions(&self) -> &[&str] {
        &["TEX", "tex"]
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Reflect, Asset)]
// pub struct PngBuffer {
//     buf: Vec<u8>,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct ZTexPngBufferLoader {}

// impl AssetLoader for ZTexPngBufferLoader {
//     type Asset = PngBuffer;
//     type Settings = ();
//     type Error = ZTexError;

//     async fn load<'a>(
//         &'a self,
//         reader: &'a mut Reader<'_>,
//         _settings: &'a Self::Settings,
//         _load_context: &'a mut LoadContext<'_>,
//     ) -> Result<Self::Asset, Self::Error> {
//         let mut bytes = Vec::new();
//         reader.read_to_end(&mut bytes).await?;

//         let mut ztex = ZTex::from_bytes(bytes)?;
//         let mut buf = Vec::new();
//         let encoder = PngEncoder::new(&mut buf);

//         ztex.encode(encoder)?;

//         Ok(PngBuffer { buf })
//     }

//     fn extensions(&self) -> &[&str] {
//         &["TEX", "tex"]
//     }
// }
