use std::convert::TryFrom;

use crate::{
    code::Code,
    stack::{Stack, Value},
};
use operator::Operator;
use serde::Deserialize;
use zen_parser::prelude::*;

pub mod operator;

pub struct Machine<R: BinaryRead> {
    stack: Stack<Value>,
    code: Code<R>,
    instruction_pointer: usize,
}

impl<R: BinaryRead> Machine<R> {
    pub fn new(code: Code<R>) -> Machine<R> {
        Self {
            stack: Stack::new(),
            code,
            instruction_pointer: 0,
        }
    }
    pub fn run(&mut self) {
        while self.instruction_pointer < self.code.len() {
            let operator = self.next_operator();
            match dbg!(operator) {
                Operator::Add => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a + b));
                } // a + b
                Operator::Subract => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a - b));
                } // a - b
                Operator::Multiply => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a * b));
                } // a * b
                Operator::Divide => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a / b));
                } // a / b
                Operator::Mod => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a % b));
                } // a % b
                Operator::BinOr => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a | b));
                } // a | b
                Operator::BinAnd => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a & b));
                } // a & b
                Operator::Less => {
                    let val = if self.stack.pop().get(&self.code.symbol_table)
                        < self.stack.pop().get(&self.code.symbol_table)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a < b
                Operator::Greater => {
                    let val = if self.stack.pop().get(&self.code.symbol_table)
                        > self.stack.pop().get(&self.code.symbol_table)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a > b
                Operator::Assign => todo!(), // a = b
                Operator::LogOr => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    let val = if a != 0 || b != 0 { 1 } else { 0 };
                    self.stack.push(Value::Data(val))
                } // a || b
                Operator::LogAnd => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    let val = if a != 0 && b != 0 { 1 } else { 0 };
                    self.stack.push(Value::Data(val))
                } // a && b
                Operator::ShiftLeft => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a << b));
                } // a << b
                Operator::ShiftRight => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    let b = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(a >> b));
                } // a >> b
                Operator::LessOrEqual => {
                    let val = if self.stack.pop().get(&self.code.symbol_table)
                        <= self.stack.pop().get(&self.code.symbol_table)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a <= b
                Operator::Equal => {
                    let val = if self.stack.pop().get(&self.code.symbol_table)
                        == self.stack.pop().get(&self.code.symbol_table)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a == b
                Operator::NotEqual => {
                    let val = if self.stack.pop().get(&self.code.symbol_table)
                        != self.stack.pop().get(&self.code.symbol_table)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a != b
                Operator::GreaterOrEqual => {
                    let val = if self.stack.pop().get(&self.code.symbol_table)
                        >= self.stack.pop().get(&self.code.symbol_table)
                    {
                        1
                    } else {
                        0
                    };
                    self.stack.push(Value::Data(val))
                } // a >= b
                Operator::AssignAdd => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(idx, arr_idx) => {
                        let other = self.stack.pop().get(&self.code.symbol_table);
                        let val = self.code.symbol_table.get(&idx, &arr_idx).unwrap();
                        self.code
                            .symbol_table
                            .insert(&idx, &arr_idx, val + other as i32);
                    }
                }, // a += b (a = a + b)
                Operator::AssignSubtract => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(idx, arr_idx) => {
                        let other = self.stack.pop().get(&self.code.symbol_table);
                        let val = self.code.symbol_table.get(&idx, &arr_idx).unwrap();
                        self.code
                            .symbol_table
                            .insert(&idx, &arr_idx, val - other as i32);
                    }
                }, // a -= b (a = a - b)
                Operator::AssignMultiply => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(idx, arr_idx) => {
                        let other = self.stack.pop().get(&self.code.symbol_table);
                        let val = self.code.symbol_table.get(&idx, &arr_idx).unwrap();
                        self.code
                            .symbol_table
                            .insert(&idx, &arr_idx, val * other as i32);
                    }
                }, // a *= b (a = a * b)
                Operator::AssignDivide => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(idx, arr_idx) => {
                        let other = self.stack.pop().get(&self.code.symbol_table);
                        let val = self.code.symbol_table.get(&idx, &arr_idx).unwrap();
                        self.code
                            .symbol_table
                            .insert(&idx, &arr_idx, val / other as i32);
                    }
                }, // a /= b (a = a / b)
                Operator::Plus => todo!(),   // +a
                Operator::Minus => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(-a));
                } // -a
                Operator::Not => {
                    let a = self.stack.pop().get(&self.code.symbol_table);
                    self.stack.push(Value::Data(!a));
                } // !a
                Operator::Negate => todo!(), // ~a
                //	LeftBracket     => 40,    // '('
                //	RightBracket    => 41,    // ')'
                //	Semicolon       => 42,    // ';'
                //	Comma           => 43,    // ','
                //	CurlyBracket    => 44,    // '{', '}'
                //	None            => 45,
                //	Float           => 51,
                //	Var             => 52,
                //	Operator        => 53,
                Operator::Ret => todo!(),
                Operator::Call => todo!(),
                Operator::CallExternal => todo!(),
                //	PopInt          => 63,
                Operator::PushInt => {
                    let val = self.next::<i32>();
                    self.stack.push(Value::Data(val as i64));
                }
                Operator::PushVar => {
                    let addr = self.next::<u32>();
                    // TODO 0 should be current instance
                    self.stack.push(Value::Address(0, addr))
                }
                //	PushString      => 66,
                Operator::PushInstance => todo!(),
                //	PushIndex       => 68,
                //	PopVar          => 69,
                Operator::AssignString => todo!(),
                Operator::AssignStringRef => todo!(),
                Operator::AssignFunc => todo!(),
                Operator::AssignFloat => match self.stack.pop() {
                    Value::Data(_) => panic!(),
                    Value::Address(idx, arr_idx) => {
                        let other = self.stack.pop().get(&self.code.symbol_table);
                        let val = self.code.symbol_table.get(&idx, &arr_idx).unwrap();
                        self.code.symbol_table.insert(&idx, &arr_idx, other as i32);
                    }
                },
                Operator::AssignInstance => todo!(),
                Operator::Jump => todo!(),
                Operator::JumpIf => todo!(),
                Operator::SetInstance => todo!(),
                //	Skip            => 90,
                //	Label           => 91,
                //	Func            => 92,
                //	FuncEnd         => 93,
                //	Class           => 94,
                //	ClassEnd        => 95,
                //	Instance        => 96,
                //	InstanceEnd     => 97,
                //	String          => 98,
                //	Array           => 180,  // Var + 128
                Operator::PushArrayVar => todo!(), // PushVar +
            }
            dbg!(&self.stack);
        }
    }
    fn next_operator(&mut self) -> Operator {
        let operator = u8::deserialize(&mut self.code.deserializer).unwrap();
        self.instruction_pointer += 1;
        Operator::try_from(operator).unwrap()
    }
    pub fn next<'de, T: Deserialize<'de>>(&'de mut self) -> T {
        T::deserialize(&mut self.code.deserializer).unwrap()
    }
}
