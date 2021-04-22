use crate::error::*;
use serde::Deserialize;
use zen_parser::prelude::*;

#[derive(Debug, Default)]
pub struct ZenMeshBuilder {
    name: String,
}

#[derive(Debug, Default)]
pub struct ZenMesh {}

fn read_chunk<R: BinaryRead + AsciiRead>(reader: R, builder: ZenMeshBuilder) -> Result<ZenMesh> {
    let mut deserializer = BinaryDeserializer::from(reader);
    // let chunk_header = <ChunkHeader>::deserialize(&mut deserializer)?;

    // match chunk_header.name.as_str() {
    //     "MeshAndBsp" => todo!(),
    //     "VobTree" => todo!(),
    //     "WayNet" => todo!(),
    //     _ => todo!(),
    // }
    todo!()
}

impl ZenMesh {
    pub fn new<R: BinaryRead + AsciiRead>(mut reader: R, name: &str) -> Result<ZenMesh> {
        let _header = Reader::from(&mut reader).read_header()?;

        let mut buf = [0_u8; 182];
        reader.read(&mut buf);
        buf.into_iter().for_each(|u| print!("{}", *u as char));

        //read chunk header ocworld
        //struct oCWorld {}

        // todo!()

        //read chunk header
        // MeshAndBsp | VobTree | WayNet | _ => skip chunk

        // let mut deserializer = BinaryDeserializer::from(&mut reader);
        // let _unknown = <Unknown>::deserialize(&mut deserializer)?;

        read_chunk::<R>(
            reader,
            ZenMeshBuilder {
                name: name.to_owned(),
                ..Default::default()
            },
        )
    }
}
