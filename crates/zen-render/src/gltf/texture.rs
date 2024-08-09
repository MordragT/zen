use image::codecs::png::PngEncoder;

use gltf_json as json;
use zen_parser::binary::BinaryRead;

use crate::texture::ZTex;

use super::{error::GltfResult, GltfBuilder};

impl GltfBuilder {
    pub fn insert_ztex<R>(&mut self, ztex: &mut ZTex<R>) -> GltfResult<json::Index<json::Texture>>
    where
        R: BinaryRead,
    {
        let mut image = Vec::new();

        let encoder = PngEncoder::new(&mut image);
        ztex.encode(encoder)?;

        let byte_length = self.push_buffer(image);

        let image_buf = self.root.push(json::Buffer {
            byte_length,
            name: None,
            uri: None,
            extensions: None,
            extras: None,
        });

        let image_view = self.root.push(json::buffer::View {
            buffer: image_buf,
            byte_length,
            byte_offset: None,
            byte_stride: None,
            name: None,
            target: Some(json::validation::Checked::Valid(
                json::buffer::Target::ArrayBuffer,
            )),
            extensions: None,
            extras: None,
        });

        let image = self.root.push(json::Image {
            buffer_view: Some(image_view),
            mime_type: Some(json::image::MimeType("image/png".to_owned())),
            name: None,
            uri: None,
            extensions: None,
            extras: None,
        });

        let texture = self.root.push(json::Texture {
            name: None,
            sampler: None,
            source: image,
            extensions: None,
            extras: None,
        });

        Ok(texture)
    }
}
