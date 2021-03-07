use crate::code::Code;
use std::fmt;
use zen_parser::binary::BinaryRead;
#[derive(Debug)]
pub struct Stack<T: Default>(Vec<T>);

impl<T: Default> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> T {
        self.0.pop().unwrap_or_default()
    }
}

impl<T: Default + fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "[{}]",
            self.0.iter().fold(String::new(), |mut string, val| {
                string.push_str(&format!("{}, ", val.to_string()));
                string
            })
        ))
    }
}

#[derive(Debug)]
pub enum Value {
    Address(usize),
    Data(i32),
}

impl Value {
    pub fn get(&self, code: &Code) -> i32 {
        match self {
            Self::Address(offset) => *code.get(*offset).unwrap(),
            Self::Data(d) => *d,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Data(0)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(a) => f.write_str(&format!("address({})", a)),
            Self::Data(d) => f.write_str(&format!("data({})", d)),
        }
    }
}

// #[derive(Clone, Copy)]
// pub enum Constant {
//     Integer(i32),
//     Unsigned(u32),
//     Float(f32),
// }

// impl Constant {
//     pub fn add(&self, other: Constant) -> Constant {
//         match self {
//             Self::Integer(s) => match other {
//                 Self::Integer(o) => Self::Integer(s + o),
//                 Self::Unsigned(o) => Self::Integer(s + o as i32),
//                 Self::Float(o) => Self::Float(*s as f32 + o),
//             },
//             Self::Unsigned(s) => match other {
//                 Self::Integer(o) => Self::Integer(*s as i32 + o),
//                 Self::Unsigned(o) => Self::Unsigned(s + o),
//                 Self::Float(o) => Self::Float(*s as f32 + o),
//             },
//             Self::Float(s) => match other {
//                 Self::Integer(o) => Self::Float(s + o as f32),
//                 Self::Unsigned(o) => Self::Float(s + o as f32),
//                 Self::Float(o) => Self::Float(s + o as f32),
//             },
//         }
//     }
// }

// #[derive(Copy, Clone)]
// pub enum Value {
//     Address(u32),
//     Constant(Constant),
// }

// type VTable = HashMap<u32, Constant>;

// impl Value {
//     pub fn get(&self, table: &VTable) -> Constant {
//         match self {
//             Self::Address(a) => *table.get(a).unwrap(),
//             Self::Constant(c) => *c,
//         }
//     }
// }

// impl Default for Value {
//     fn default() -> Self {
//         Self::Constant(Constant::Integer(0))
//     }
// }
