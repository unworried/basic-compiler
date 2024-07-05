use std::env;
use std::fs::File;
use std::io::{stdin, BufReader, Read};
use std::path::Path;

use flipvm::{LinearMemory, Machine, MappedMemoryBuffer, Register};

fn signal_halt(vm: &mut Machine, _: u16) -> Result<(), String> {
    vm.halt = true;
    Ok(())
}

pub fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let reader: Box<dyn Read> = match args[1].as_ref() {
        "-" => Box::new(stdin()),
        _ => {
            Box::new(File::open(Path::new(&args[1])).map_err(|e| format!("failed to open: {}", e))?)
        }
    };

    let mut reader = BufReader::new(reader);
    let mut program: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut program)
        .map_err(|e| format!("read: {}", e))?;

    let mut vm = Machine::new();
    vm.map(0x1000, 0x4000, Box::new(LinearMemory::new(1024 * 5)))?;
    vm.map(
        0x10,
        program.len(),
        Box::new(MappedMemoryBuffer::new(program)),
    )?;
    vm.set_register(Register::SP, 0x1000);
    vm.set_register(Register::PC, 0x10);

    vm.define_handler(0xf0, signal_halt);
    while !vm.halt {
        println!("{}", vm.state());
        vm.step()?;
    }
    println!("A = {}", vm.get_register(Register::A));
    Ok(())
}
