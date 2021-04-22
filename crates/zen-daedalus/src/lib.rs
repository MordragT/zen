//! This crate allows Daedalus Bytecode to be executed on a virtual machine.
//!
//! You can load a DAT-File and run the bytecode the following way
//! ```rust
//! use std::fs::File;
//! use zen_daedalus::prelude::*;
//!
//! let file =
//!     File::open("/home/user/../Gothic II/_work/Data/Scripts/_compiled/CAMERA.DAT")
//!         .unwrap();

//! let code = Code::new(file).unwrap();
//! let mut machine = Machine::new(code);
//! machine.run();
//!```

#![feature(vec_into_raw_parts)]

pub mod code;
pub mod machine;
pub mod stack;

pub mod prelude {
    pub use crate::code::Code;
    pub use crate::machine::Machine;
}
