use gltf_json::{self as json, validation::USize64};

mod material;
mod mesh;
mod primitive;
mod texture;

pub mod error;

#[derive(Debug, Clone)]
pub struct GltfBuilder {
    buffers: Vec<Vec<u8>>,
    root: json::Root,
}

impl GltfBuilder {
    pub fn push_buffer<T>(&mut self, buf: Vec<T>) -> USize64 {
        let bytes = to_padded_byte_vector(buf);
        let byte_length = bytes.len();
        self.buffers.push(bytes);

        USize64::from(byte_length)
    }
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * std::mem::size_of::<T>();
    let byte_capacity = vec.capacity() * std::mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}
