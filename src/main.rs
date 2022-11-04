use std::{env, io::{BufReader, Read}, fs::File};

use vm::VM;


pub mod vm;
pub mod opcode;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2{
        println!("Wrong number of parameters!");
        println!("Usage: bad_vm <file_name>");
        return;
    }

    let file_name = args[1].clone();

    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut program = Vec::new();
    reader.read_to_end(&mut program).unwrap();

    let mut vm = VM::new_with_program(program);

    vm.run();

    vm.print_registers();

}
