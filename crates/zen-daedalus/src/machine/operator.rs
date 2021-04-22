use std::convert::TryFrom;

/// The different Operators that can occur in the [machine](crate::machine)
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add = 0,             // a + b
    Subract = 1,         // a - b
    Multiply = 2,        // a * b
    Divide = 3,          // a / b
    Mod = 4,             // a % b
    BinOr = 5,           // a | b
    BinAnd = 6,          // a & b
    Less = 7,            // a < b
    Greater = 8,         // a > b
    Assign = 9,          // a = b
    LogOr = 11,          // a || b
    LogAnd = 12,         // a && b
    ShiftLeft = 13,      // a << b
    ShiftRight = 14,     // a >> b
    LessOrEqual = 15,    // a <= b
    Equal = 16,          // a == b
    NotEqual = 17,       // a != b
    GreaterOrEqual = 18, // a >= b
    AssignAdd = 19,      // a += b (a = a + b)
    AssignSubtract = 20, // a -= b (a = a - b)
    AssignMultiply = 21, // a *= b (a = a * b)
    AssignDivide = 22,   // a /= b (a = a / b)
    Plus = 30,           // +a
    Minus = 31,          // -a
    Not = 32,            // !a
    Negate = 33,         // ~a
    //	LeftBracket     = 40,    // '('
    //	RightBracket    = 41,    // ')'
    //	Semicolon       = 42,    // ';'
    //	Comma           = 43,    // ','
    //	CurlyBracket    = 44,    // '{', '}'
    //	None            = 45,
    //	Float           = 51,
    //	Var             = 52,
    //	Operator        = 53,
    Ret = 60,
    Call = 61,
    CallExternal = 62,
    //	PopInt          = 63,
    PushInt = 64,
    PushVar = 65,
    //	PushString      = 66,
    PushInstance = 67,
    //	PushIndex       = 68,
    //	PopVar          = 69,
    AssignString = 70,
    AssignStringRef = 71,
    AssignFunc = 72,
    AssignFloat = 73,
    AssignInstance = 74,
    Jump = 75,
    JumpIf = 76,
    SetInstance = 80,
    //	Skip            = 90,
    //	Label           = 91,
    //	Func            = 92,
    //	FuncEnd         = 93,
    //	Class           = 94,
    //	ClassEnd        = 95,
    //	Instance        = 96,
    //	InstanceEnd     = 97,
    //	String          = 98,
    //	Array           = 180,  // Var + 128
    PushArrayVar = 245, // PushVar + Array
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => f.write_str("Add"),
            Self::Subract => f.write_str("Subtract"),
            Self::Multiply => f.write_str("Multiply"),
            Self::Divide => f.write_str("Divide"),
            Self::Mod => f.write_str("Mod"),
            Self::BinOr => f.write_str("BinOr"),
            Self::BinAnd => f.write_str("BinAnd"),
            Self::Less => f.write_str("Less"),
            Self::Greater => f.write_str("Greater"),
            Self::Assign => f.write_str("Assign"),
            Self::LogOr => f.write_str("LogOr"),
            Self::LogAnd => f.write_str("LogAnd"),
            Self::ShiftLeft => f.write_str("ShiftLeft"),
            Self::ShiftRight => f.write_str("ShiftRight"),
            Self::LessOrEqual => f.write_str("LessOrEqual"),
            Self::Equal => f.write_str("Equal"),
            Self::NotEqual => f.write_str("NotEqual"),
            Self::GreaterOrEqual => f.write_str("GreaterOrEqual"),
            Self::AssignAdd => f.write_str("AssignAdd"),
            Self::AssignSubtract => f.write_str("AssignSubtract"),
            Self::AssignMultiply => f.write_str("AssignMultiply"),
            Self::AssignDivide => f.write_str("AssignDivide"),
            Self::Plus => f.write_str("Plus"),
            Self::Minus => f.write_str("Minus"),
            Self::Not => f.write_str("Not"),
            Self::Negate => f.write_str("Negate"),
            Self::Ret => f.write_str("Ret"),
            Self::Call => f.write_str("Call"),
            Self::CallExternal => f.write_str("CallExternal"),
            Self::PushInt => f.write_str("PushInt"),
            Self::PushVar => f.write_str("PushVar"),
            Self::PushInstance => f.write_str("PushInstance"),
            Self::AssignString => f.write_str("AssignString"),
            Self::AssignStringRef => f.write_str("AssignStringRef"),
            Self::AssignFunc => f.write_str("AssignFunc"),
            Self::AssignFloat => f.write_str("AssignFloat"),
            Self::AssignInstance => f.write_str("AssignInstance"),
            Self::Jump => f.write_str("Jump"),
            Self::JumpIf => f.write_str("JumpIf"),
            Self::SetInstance => f.write_str("SetInstance"),
            Self::PushArrayVar => f.write_str("PushArrayVec"),
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Operator::Add as u8 => Ok(Operator::Add), // a + b
            x if x == Operator::Subract as u8 => Ok(Operator::Subract), // a - b
            x if x == Operator::Multiply as u8 => Ok(Operator::Multiply), // a * b
            x if x == Operator::Divide as u8 => Ok(Operator::Divide), // a / b
            x if x == Operator::Mod as u8 => Ok(Operator::Mod), // a % b
            x if x == Operator::BinOr as u8 => Ok(Operator::BinOr), // a | b
            x if x == Operator::BinAnd as u8 => Ok(Operator::BinAnd), // a & b
            x if x == Operator::Less as u8 => Ok(Operator::Less), // a < b
            x if x == Operator::Greater as u8 => Ok(Operator::Greater), // a > b
            x if x == Operator::Assign as u8 => Ok(Operator::Assign), // a = b
            x if x == Operator::LogOr as u8 => Ok(Operator::LogOr), // a || b
            x if x == Operator::LogAnd as u8 => Ok(Operator::LogAnd), // a && b
            x if x == Operator::ShiftLeft as u8 => Ok(Operator::ShiftLeft), // a << b
            x if x == Operator::ShiftRight as u8 => Ok(Operator::ShiftRight), // a >> b
            x if x == Operator::LessOrEqual as u8 => Ok(Operator::LessOrEqual), // a <= b
            x if x == Operator::Equal as u8 => Ok(Operator::Equal), // a == b
            x if x == Operator::NotEqual as u8 => Ok(Operator::NotEqual), // a != b
            x if x == Operator::GreaterOrEqual as u8 => Ok(Operator::GreaterOrEqual), // a >= b
            x if x == Operator::AssignAdd as u8 => Ok(Operator::AssignAdd), // a += b (a = a + b)
            x if x == Operator::AssignSubtract as u8 => Ok(Operator::AssignSubtract), // a -= b (a = a - b)
            x if x == Operator::AssignMultiply as u8 => Ok(Operator::AssignMultiply), // a *= b (a = a * b)
            x if x == Operator::AssignDivide as u8 => Ok(Operator::AssignDivide), // a /= b (a = a / b)
            x if x == Operator::Plus as u8 => Ok(Operator::Plus),                 // +a
            x if x == Operator::Minus as u8 => Ok(Operator::Minus),               // -a
            x if x == Operator::Not as u8 => Ok(Operator::Not),                   // !a
            x if x == Operator::Negate as u8 => Ok(Operator::Negate),             // ~a
            //	LeftBracket     = 40,    // '('
            //	RightBracket    = 41,    // ')'
            //	Semicolon       = 42,    // ';'
            //	Comma           = 43,    // ','
            //	CurlyBracket    = 44,    // '{', '}'
            //	None            = 45,
            //	Float           = 51,
            //	Var             = 52,
            //	Operator        = 53,
            x if x == Operator::Ret as u8 => Ok(Operator::Ret),
            x if x == Operator::Call as u8 => Ok(Operator::Call),
            x if x == Operator::CallExternal as u8 => Ok(Operator::CallExternal),
            //	PopInt          = 63,
            x if x == Operator::PushInt as u8 => Ok(Operator::PushInt),
            x if x == Operator::PushVar as u8 => Ok(Operator::PushVar),
            //	PushString      = 66,
            x if x == Operator::PushInstance as u8 => Ok(Operator::PushInstance),
            //	PushIndex       = 68,
            //	PopVar          = 69,
            x if x == Operator::AssignString as u8 => Ok(Operator::AssignString),
            x if x == Operator::AssignStringRef as u8 => Ok(Operator::AssignStringRef),
            x if x == Operator::AssignFunc as u8 => Ok(Operator::AssignFunc),
            x if x == Operator::AssignFloat as u8 => Ok(Operator::AssignFloat),
            x if x == Operator::AssignInstance as u8 => Ok(Operator::AssignInstance),
            x if x == Operator::Jump as u8 => Ok(Operator::Jump),
            x if x == Operator::JumpIf as u8 => Ok(Operator::JumpIf),
            x if x == Operator::SetInstance as u8 => Ok(Operator::SetInstance),
            //	Skip            = 90,
            //	Label           = 91,
            //	Func            = 92,
            //	FuncEnd         = 93,
            //	Class           = 94,
            //	ClassEnd        = 95,
            //	Instance        = 96,
            //	InstanceEnd     = 97,
            //	String          = 98,
            //	Array           = 180,  // Var + 128
            x if x == Operator::PushArrayVar as u8 => Ok(Operator::PushArrayVar), // PushVar + Array
            _ => Err(()),
        }
    }
}
