use ddsfile::{AlphaMode, D3D10ResourceDimension, Dds};
use std::cmp;
use std::convert::TryInto;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub mod ztex;

/// Convert ZTEX to DDS image format
pub fn convert_ztex_to_dds<'a>(ztex_data: &[u8]) -> Result<Dds, &'a str> {
    let mut reader = Cursor::new(ztex_data);
    let header =
        bincode::deserialize_from::<&mut Cursor<&[u8]>, ztex::Header>(&mut reader).unwrap(); //ztex::Header::from_reader(&mut reader);
    if header.get_signature() != ztex::FILE_SIGNATURE || header.get_version() != ztex::FILE_VERSION
    {
        return Err("Wrong ZTEX Signature or Version");
    }
    let mut dds = match header.get_format().try_into() {
        Ok(format) => Dds::new_d3d(
            header.get_height(),
            header.get_width(),
            None,
            format,
            Some(header.get_mipmap_level()),
            None,
        ),
        Err(_) => match header.get_format().try_into() {
            Ok(format) => Dds::new_dxgi(
                header.get_height(),
                header.get_width(),
                None,
                format,
                Some(header.get_mipmap_level()),
                None,
                None,
                false,
                D3D10ResourceDimension::Unknown,
                AlphaMode::Unknown,
            ),
            Err(_) => return Err("Couldnt convert ZTEX format to DDS format."),
        },
    }
    .unwrap();

    let _palette = match header.get_format() == ztex::Format::P8 {
        true => {
            let mut palette = ztex::Palette::new();
            for _ in 0..ztex::PALETTE_ENTRIES {
                let entry =
                    bincode::deserialize_from::<&mut Cursor<&[u8]>, ztex::Entry>(&mut reader)
                        .unwrap();
                palette.push(entry);
            }
            match palette.len() == ztex::PALETTE_ENTRIES {
                true => Some(palette),
                false => None,
            }
        }
        false => None,
    };
    let mipmap_count = cmp::max(1, header.get_mipmap_level());
    let mut size_of_all_mip_maps = 0;
    for layer in 0..mipmap_count {
        size_of_all_mip_maps += get_mip_map_size(
            &header.get_format(),
            header.get_width(),
            header.get_height(),
            layer,
        );
    }
    let size_of_biggest_mip_map = get_mip_map_size(
        &header.get_format(),
        header.get_width(),
        header.get_height(),
        0,
    );
    let pos_of_biggest_mip_map = size_of_all_mip_maps - size_of_biggest_mip_map;
    reader
        .seek(SeekFrom::Current(pos_of_biggest_mip_map as i64))
        .unwrap();
    let mut biggest_mip_map_buf = vec![0_u8; size_of_biggest_mip_map as usize];
    reader.read_exact(&mut biggest_mip_map_buf).unwrap();
    dds.set_data(0, biggest_mip_map_buf).unwrap();

    Ok(dds)
}

/// level 0 = highest, ztex is built other way round, 0 = lowest
fn get_mip_map_size(format: &ztex::Format, width: u32, height: u32, level: u32) -> u32 {
    let mut x = cmp::max(1, width);
    let mut y = cmp::max(1, height);
    for _ in 0..level {
        if x > 1 {
            x >>= 1;
        }
        if y > 1 {
            y >>= 1;
        }
    }
    match format {
        ztex::Format::B8G8R8A8
        | ztex::Format::R8G8B8A8
        | ztex::Format::A8B8G8R8
        | ztex::Format::A8R8G8B8 => x * y * 4,
        ztex::Format::B8G8R8 | ztex::Format::R8G8B8 => x * y * 3,
        ztex::Format::A4R4G4B4 | ztex::Format::A1R5G5B5 | ztex::Format::R5G6B5 => x * y * 2,
        ztex::Format::P8 => x * y,
        ztex::Format::DXT1 => cmp::max(1, x / 4) * cmp::max(1, y / 4) * 8,
        ztex::Format::DXT2 | ztex::Format::DXT3 | ztex::Format::DXT4 | ztex::Format::DXT5 => {
            cmp::max(1, x / 4) * cmp::max(1, y / 4) * 16
        }
    }
}
