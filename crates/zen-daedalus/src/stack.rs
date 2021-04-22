use crate::code::Code;
use std::fmt;

/// This is the stack which is used by the [machine](crate::machine)
#[derive(Debug)]
pub struct Stack<T: Default>(Vec<T>);

impl<T: Default> Stack<T> {
    /// Creates a new stack
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }
    /// Checks if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Pushes a value on the stack
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }
    /// Pops a value from the stack and pops the default value if the stack is empty
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

/// The Values that are used on the stack for the [machine](crate::machine)
#[derive(Debug)]
pub enum Value {
    Address(usize),
    Data(i32),
}

impl Value {
    /// Gets the inner data or uses the code to retrieve the data
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
