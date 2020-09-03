use image::dds::DdsDecoder;
use image::png::PNGEncoder;
use image::ImageDecoder;
use std::fs::File;
use std::io::Cursor;
use zen_archive::Vdfs;

fn main() {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic/Data/textures.VDF").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    let yellow_tex = vdf.get_by_name("ORC_BODYSHAMAN_V-C.TEX").unwrap().unwrap();
    let data = Cursor::new(yellow_tex.data);
    let dds = zen_texture::convert_ztex_to_dds(data).unwrap();
    let mut dds_file_buf = vec![];
    dds.write(&mut dds_file_buf).unwrap();
    let dds_file = Cursor::new(dds_file_buf);
    let decoder = DdsDecoder::new(dds_file).unwrap();
    let (width, height) = decoder.dimensions();
    let color_type = decoder.color_type();
    let mut dds_bytes = vec![0_u8; decoder.total_bytes() as usize];
    decoder.read_image(&mut dds_bytes).unwrap();

    let mut output_jpeg = File::create("/home/tom/out.png").unwrap();
    let encoder = PNGEncoder::new(&mut output_jpeg);
    encoder
        .encode(dds_bytes.as_slice(), width, height, color_type)
        .unwrap();
    //vdf.list();
}
