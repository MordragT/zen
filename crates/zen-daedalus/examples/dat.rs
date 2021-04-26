use std::fs::File;
use zen_daedalus::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file =
        File::open("/home/tom/Steam/common/Gothic II/_work/Data/Scripts/_compiled/CAMERA.DAT")?;

    let code = Code::new(file)?;
    let mut machine = Machine::new(code);
    machine.run();
    Ok(())
}
