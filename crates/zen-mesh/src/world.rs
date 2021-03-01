// use crate::error::*;
// use serde::Deserialize;
// use zen_parser::prelude::*;
// use zen_types::mesh::Chunk;

// #[derive(Debug, Default)]
// pub struct WorldMeshBuilder {
//     name: String,
// }

// #[derive(Debug, Default)]
// pub struct WorldMesh {
//     id: u16,
//     length: u32,
// }

// fn read_chunk<R: BinaryRead + AsciiRead>(reader: R, builder: WorldMeshBuilder) -> Result<WorldMesh> {
//     let mut deserializer = BinaryDeserializer::from(reader);
//     let chunk_header = <ChunkHeader>::deserialize(&mut deserializer)?;

//     match chunk_header.
// }

// impl WorldMesh {
//     pub fn new<R: BinaryRead + AsciiRead>(reader: R, name: &str) -> Result<WorldMesh> {
//         // #[derive(Debug, Deserialize)]
//         // struct Unknown {
//         //     a: u32,
//         //     b: u32,
//         //     c: u32,
//         //     d: u32,
//         // }

//         let _header = AsciiDeserializer::from(&mut reader).read_header()?;

//         //read chunk header ocworld

//         //read chunk header
//             // MeshAndBsp | VobTree | WayNet | _ => skip chunk

//         let mut deserializer = BinaryDeserializer::from(&mut reader);
//         let _unknown = <Unknown>::deserialize(&mut deserializer)?;

//         //read_chunk::<R>(reader, WorldMeshBuilder {name: name.to_owned(), ..Default::default()})
//     }
// }
