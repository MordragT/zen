use std::convert::TryFrom;

use crate::{
    code::Code,
    stack::{Stack, Value},
};
use operator::Operator;
use serde::Deserialize;
use zen_parser::prelude::*;

pub mod operator;

pub struct Machine {
    stack: Stack<Value>,
    code: Code,
    instruction_pointer: usize,
}

impl Machine {
    pub fn new(code: Code) -> Machine {
        Self {
            stack: Stack::new(),
            code,
            instruction_pointer: 0,
        }
    }
    pub fn run(&mut self) {
        while self.instruction_pointer < self.code.len() {
            let operator = self.next_operator();
            print!("{}:\t{}  \t", self.instruction_pointer, operator);
            match operator {
                Operator::Add => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a + b));
                } // a + b
                Operator::Subract => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a - b));
                } // a - b
                Operator::Multiply => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a * b));
                } // a * b
                Operator::Divide => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a / b));
                } // a / b
                Operator::Mod => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a % b));
                } // a % b
                Operator::BinOr => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a | b));
                } // a | b
                Operator::BinAnd => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a & b));
                } // a & b
                Operator::Less => {
                    let val = if self.stack.pop().get(&self.code) < self.stack.pop().get(&self.code)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a < b
                Operator::Greater => {
                    let val = if self.stack.pop().get(&self.code) > self.stack.pop().get(&self.code)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a > b
                Operator::Assign => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(offset) => {
                        let other = self.stack.pop().get(&self.code);
                        let val = self.code.get_mut(offset).unwrap();
                        *val = other;
                    }
                },
                Operator::LogOr => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    let val = if a != 0 || b != 0 { 1 } else { 0 };
                    self.stack.push(Value::Data(val))
                } // a || b
                Operator::LogAnd => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    let val = if a != 0 && b != 0 { 1 } else { 0 };
                    self.stack.push(Value::Data(val))
                } // a && b
                Operator::ShiftLeft => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a << b));
                } // a << b
                Operator::ShiftRight => {
                    let a = self.stack.pop().get(&self.code);
                    let b = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(a >> b));
                } // a >> b
                Operator::LessOrEqual => {
                    let val =
                        if self.stack.pop().get(&self.code) <= self.stack.pop().get(&self.code) {
                            1
                        } else {
                            0
                        };
                    self.stack.push(Value::Data(val))
                } // a <= b
                Operator::Equal => {
                    let val =
                        if self.stack.pop().get(&self.code) == self.stack.pop().get(&self.code) {
                            1
                        } else {
                            0
                        };
                    self.stack.push(Value::Data(val))
                } // a == b
                Operator::NotEqual => {
                    let val =
                        if self.stack.pop().get(&self.code) != self.stack.pop().get(&self.code) {
                            1
                        } else {
                            0
                        };
                    self.stack.push(Value::Data(val))
                } // a != b
                Operator::GreaterOrEqual => {
                    let val =
                        if self.stack.pop().get(&self.code) >= self.stack.pop().get(&self.code) {
                            1
                        } else {
                            0
                        };
                    self.stack.push(Value::Data(val))
                } // a >= b
                Operator::AssignAdd => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(offset) => {
                        let other = self.stack.pop().get(&self.code);
                        let val = self.code.get_mut(offset).unwrap();
                        *val += other;
                    }
                }, // a += b (a = a + b)
                Operator::AssignSubtract => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(offset) => {
                        let other = self.stack.pop().get(&self.code);
                        let val = self.code.get_mut(offset).unwrap();
                        *val -= other;
                    }
                }, // a -= b (a = a - b)
                Operator::AssignMultiply => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(offset) => {
                        let other = self.stack.pop().get(&self.code);
                        let val = self.code.get_mut(offset).unwrap();
                        *val *= other;
                    }
                }, // a *= b (a = a * b)
                Operator::AssignDivide => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(offset) => {
                        let other = self.stack.pop().get(&self.code);
                        let val = self.code.get_mut(offset).unwrap();
                        *val /= other;
                    }
                }, // a /= b (a = a / b)
                Operator::Plus => todo!(), // +a
                Operator::Minus => {
                    let a = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(-a));
                } // -a
                Operator::Not => {
                    let a = self.stack.pop().get(&self.code);
                    self.stack.push(Value::Data(!a));
                } // !a
                Operator::Negate => todo!(), // ~a
                Operator::Ret => todo!(),
                Operator::Call => todo!(),
                Operator::CallExternal => todo!(),
                Operator::PushInt => {
                    let val = *self.code.next::<i32>().unwrap();
                    self.stack.push(Value::Data(val));
                }
                Operator::PushVar => {
                    let addr = *self.code.next::<u32>().unwrap();
                    // TODO 0 should be current instance
                    self.stack.push(Value::Address(addr as usize))
                }
                Operator::PushInstance => todo!(),
                Operator::AssignString => todo!(),
                Operator::AssignStringRef => todo!(),
                Operator::AssignFunc => todo!(),
                Operator::AssignFloat => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(offset) => {
                        let other = self.stack.pop().get(&self.code);
                        let val = self.code.get_mut(offset).unwrap();
                        *val = other;
                    }
                },
                Operator::AssignInstance => todo!(),
                Operator::Jump => todo!(),
                Operator::JumpIf => todo!(),
                Operator::SetInstance => todo!(),
                Operator::PushArrayVar => todo!(), // PushVar +
            }
            println!("Stack: {}", self.stack);
        }
    }
    fn next_operator(&mut self) -> Operator {
        self.instruction_pointer += 1;
        let num = self.code.next::<u8>().unwrap();
        Operator::try_from(*num).unwrap()
    }
}
