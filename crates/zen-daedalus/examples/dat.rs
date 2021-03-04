use std::fs::File;
use zen_daedalus::{code::Code, machine::Machine};

pub fn main() {
    let file =
        File::open("/home/tom/Steam/common/Gothic II/_work/Data/Scripts/_compiled/CAMERA.DAT")
            .unwrap();

    let code = Code::open(file).unwrap();
    let mut machine = Machine::new(code);
    machine.run();
}
