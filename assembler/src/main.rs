use std::{env, fs::File, io::{Read, Write}};


fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3{
        println!("Incorrect number of arguments");
        println!("Usage: assembler <input_file> <output_file");
        return;
    }

    let mut file = File::open(args[1].clone()).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    
    let instructions = input.trim().split("\n").collect::<Vec<&str>>();

    let mut program = Vec::<u8>::new();

    for i in instructions {
        // ignore comments / blank lines
        if i.trim().is_empty() || i.starts_with(";"){
            continue;
        }
        let instruction = i.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
        match instruction[0]{
            "HALT" => {
                program.push(0)
            },
            "LOAD" => {
                program.push(1);
                program.push(instruction[1].parse().unwrap());
                let val: u16 = instruction[2].parse().unwrap();
                let high_byte: u8 = (val >> 8) as u8;
                let low_byte: u8 = (val & 0xff) as u8;
                program.push(high_byte);
                program.push(low_byte);
            },
            "ADD" => {
                program.push(2);
                program.push(instruction[1].parse().unwrap());
                program.push(instruction[2].parse().unwrap());
                program.push(instruction[3].parse().unwrap());
            },
            "SUB" => {
                program.push(3);
                program.push(instruction[1].parse().unwrap());
                program.push(instruction[2].parse().unwrap());
                program.push(instruction[3].parse().unwrap());
            },
            "MUL" => {
                program.push(4);
                program.push(instruction[1].parse().unwrap());
                program.push(instruction[2].parse().unwrap());
                program.push(instruction[3].parse().unwrap());
            },
            "DIV" => {
                program.push(5);
                program.push(instruction[1].parse().unwrap());
                program.push(instruction[2].parse().unwrap());
                program.push(instruction[3].parse().unwrap());
            },
            _ => {
                println!("Unknown opcode mneumonic: {}", instruction[0]);
            }
        }
    }
    let mut output_file = File::create(args[2].clone()).unwrap();
    output_file.write(&program).unwrap();

}
