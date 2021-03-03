use crate::code::Code;
use zen_parser::prelude::*;
pub struct Machine<R: BinaryRead> {
    stack: Vec<u32>,
    code: Code<R>,
    instruction_pointer: usize,
}

impl<R: BinaryRead> Machine<R> {
    pub fn run(&mut self) {
        while self.instruction_pointer < self.code.len() {}
    }
}
