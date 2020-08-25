use ddsfile::{AlphaMode, D3D10ResourceDimension, Dds};
use std::cmp;
use std::convert::TryInto;
use std::io::prelude::*;
use std::mem;

pub mod ztex;

/// Convert ZTEX to DDS image format
pub fn convert_ztex_to_dds<'a>(ztex_data: &[u8]) -> Result<Dds, &'a str> {
    let mut index = mem::size_of::<ztex::Header>();
    // TODO überprüfe ob header richtiges Format bekommt
    let header: ztex::Header = match ztex_data.get(0..index) {
        Some(header) => bincode::deserialize(header).unwrap(),
        None => return Err("Could not read ZTEX Header."),
    };
    if header.get_signature() != ztex::FILE_SIGNATURE || header.get_version() != ztex::FILE_VERSION
    {
        return Err("Wrong ZTEX Signature or Version");
    }
    let dds = match header.get_format().try_into() {
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
            // let new_index = index + mem::size_of::<ztex::Palette>();
            // let palette: ztex::Palette = match ztex_data.get(index..new_index) {
            //     Some(palette) => bincode::deserialize(palette).unwrap(),
            //     None => return Err("Could not read P8 palette"),
            // };
            // index = new_index;
            let mut palette = ztex::Palette::new();
            for _ in 0..ztex::PALETTE_ENTRIES {
                let new_index = index + mem::size_of::<ztex::Entry>();
                match ztex_data.get(index..new_index) {
                    Some(entry) => {
                        let entry: ztex::Entry = bincode::deserialize(entry).unwrap();
                        palette.push(entry);
                        index = new_index;
                    }
                    None => return Err("Could not read entry of P8 palette"),
                }
            }
            match palette.len() == ztex::PALETTE_ENTRIES {
                true => Some(palette),
                false => None,
            }
        }
        false => None,
    };
    let mipmap_count = cmp::max(1, header.get_mipmap_level());
    let mut buffer_size = 0;
    for val in 0..mipmap_count {
        buffer_size += get_mip_map_size(
            &header.get_format(),
            header.get_width(),
            header.get_height(),
            val,
        );
    }
    let buffer = {
        let new_index = index + buffer_size;
        match ztex_data.get(index..new_index) {
            Some(buf) => {
                //index = new_index; // value never read anyways
                buf
            }
            None => return Err("Error reading..."),
        }
    };
    dds.write(&mut Vec::from(buffer)).unwrap();
    Ok(dds)
}

fn get_mip_map_size(format: &ztex::Format, width: u32, height: u32, level: u32) -> usize {
    let mut x = cmp::max(1, width) as usize;
    let mut y = cmp::max(1, height) as usize;
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
