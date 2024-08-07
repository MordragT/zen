//! This crate allows Daedalus Bytecode to be executed on a virtual machine.
//!
//! You can load a DAT-File and run the bytecode the following way
//! ```rust
//! use std::fs::File;
//! use zen_daedalus::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let file =
//!     File::open("/home/tom/Steam/common/Gothic II/_work/Data/Scripts/_compiled/CAMERA.DAT")?;

//! let code = Code::new(file)?;
//! let mut machine = Machine::new(code);
//! machine.run();
//! # Ok(())
//! # }
//!```

pub mod code;
pub mod machine;
pub mod stack;

pub mod prelude {
    pub use crate::code::Code;
    pub use crate::machine::Machine;
}
