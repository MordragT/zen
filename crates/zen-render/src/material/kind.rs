use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ZenMaterialKind {
    Undef,
    Metal,
    Stone,
    Wood,
    Earth,
    Water,
    Snow,
}
