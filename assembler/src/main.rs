use std::{env, fs::File, io::{Read, Write}};


fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3{
        println!("Incorrect number of arguments");
        println!("Usage: assembler <input_file> <output_file>");
        return;
    }

    let mut file = File::open(args[1].clone()).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    
    let program = parse_program(input);
   
    let mut output_file = File::create(args[2].clone()).unwrap();
    output_file.write(&program).unwrap();

}


pub fn parse_program(input: String) -> Vec<u8>{
    let instructions = input.trim().split("\n").collect::<Vec<&str>>();

    let mut program = Vec::<u8>::new();

    for i in instructions {
        // ignore comments / blank lines
        if i.trim().is_empty() || i.starts_with(";"){
            continue;
        }
        let instruction = i.split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
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
                panic!("Unknown opcode mneumonic: {:?}", instruction[0]);
            }
        }
    }

    program

}

#[cfg(test)]
mod tests{
    use std::vec;


    #[test]
    fn test_halt(){
        let input = String::from("HALT");
        let program = crate::parse_program(input);
        assert_eq!(vec![0_u8], program)
    }

    #[test]
    fn test_load() {
        let input = String::from("LOAD 0 100");
        let program = crate::parse_program(input);
        assert_eq!(vec![1, 0, 0, 100], program);
    } 

    #[test]
    fn test_add() {
        let input = String::from(r#"
        LOAD 0 100
        LOAD 1 200
        ADD 0 1 2
        "#);
        let program = crate::parse_program(input);
        assert_eq!(vec![1, 0, 0, 100, 1, 1, 0, 200, 2, 0, 1, 2], program);
    }

    #[test]
    fn test_sub() {
        let input = String::from(r#"
        LOAD 0 100
        LOAD 1 200
        SUB 0 1 2
        "#);
        let program = crate::parse_program(input);
        assert_eq!(vec![1, 0, 0, 100, 1, 1, 0, 200, 3, 0, 1, 2], program);
    } 

    #[test]
    fn test_mul() {
        let input = String::from(r#"
        LOAD 0 100
        LOAD 1 200
        MUL 0 1 2
        "#);
        let program = crate::parse_program(input);
        assert_eq!(vec![1, 0, 0, 100, 1, 1, 0, 200, 4, 0, 1, 2], program);
    } 

    #[test]
    fn test_div() {
        let input = String::from(r#"
        LOAD 0 100
        LOAD 1 200
        DIV 0 1 2
        "#);
        let program = crate::parse_program(input);
        assert_eq!(vec![1, 0, 0, 100, 1, 1, 0, 200, 5, 0, 1, 2], program);
    } 

}
