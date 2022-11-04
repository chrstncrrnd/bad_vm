use crate::opcode::OPCode;

pub struct VM {
    program: Vec<u8>,
    pc: usize,
    registers: [i32; 32],
    remainder: u32,
}

impl VM {
    pub fn new() -> Self{
        Self{
            program: vec![],
            pc: 0,
            registers: [0; 32],
            remainder: 0
        }
    }

    pub fn new_with_program(program: Vec<u8>) -> Self{
        Self{
            program,
            pc: 0,
            registers: [0; 32],
            remainder: 0
        }
    }

    pub fn run(&mut self) {
        loop{   
            if self.pc >= self.program.len() {
                break;
            }

            let opcode = self.decode_opcode();
    
            // execute
            match opcode {
                OPCode::HALT => {
                    break;
                },
                OPCode::LOAD => {
                    let register = self.next_8_bits() as usize;
                    let number = self.next_16_bits() as i32;

                    self.registers[register] = number;
                },
                OPCode::ADD => {
                    let reg1 = self.registers[self.next_8_bits() as usize];
                    let reg2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = reg1 + reg2;
                    
                },
                OPCode::SUB => {
                    let reg1 = self.registers[self.next_8_bits() as usize];
                    let reg2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = reg1 - reg2;
                },
                OPCode::MUL => {
                    let reg1 = self.registers[self.next_8_bits() as usize];
                    let reg2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = reg1 * reg2;
                },
                OPCode::DIV => {
                    let reg1 = self.registers[self.next_8_bits() as usize];
                    let reg2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = reg1 / reg2;
                    self.remainder = (reg1 % reg2) as u32;
                },
                _ =>{
                    println!("INVALID OPCODE");
                }
            };

        }
    }

    fn next_8_bits(&mut self) -> u8 {
        let res = self.program[self.pc];
        self.pc += 1;
        res
    }

    fn next_16_bits(&mut self) -> u16 {
        let res = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        res
    }

    fn decode_opcode(&mut self) -> OPCode{
        let opcode = self.program[self.pc];
        self.pc += 1;
        OPCode::from(opcode)
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    
    #[test]
    fn load_opcode() {
        let opcode: u8 = OPCode::into(OPCode::LOAD);
        let register = 0;
        let value = 69;

        let mut vm = VM::new_with_program(vec![opcode, register, 0, value, OPCode::into(OPCode::HALT)]);

        vm.run();

        assert_eq!(vm.registers[register as usize], value as i32);
    }

    #[test]
    fn add_opcode() {
        // load 69 into first register
        // load 15 into second register,
        // add 69 and 15 and put it into output register

        let opcode_load = OPCode::into(OPCode::LOAD);
        let opcode_add = OPCode::into(OPCode::ADD);

        let first_num = 69;
        let second_num = 15;

        let first_reg = 0;
        let second_reg = 1;
        let output_reg = 2;

        let program: Vec<u8> = vec![
            opcode_load,
            first_reg,
            0,      // <- needs a 0 because 16 bit number
            first_num,
            opcode_load,
            second_reg,
            0,
            second_num,
            opcode_add,
            first_reg,
            second_reg,
            output_reg
        ];

        let mut vm = VM::new_with_program(program);

        vm.run();

        assert_eq!(vm.registers[output_reg as usize], (first_num + second_num) as i32);

    }


    #[test]
    fn sub_opcode() {
        // load 69 into first register
        // load 15 into second register,
        // sub 69 from 15 and put it into output register

        let opcode_load = OPCode::into(OPCode::LOAD);
        let opcode_add = OPCode::into(OPCode::SUB);

        let first_num = 69;
        let second_num = 15;

        let first_reg = 0;
        let second_reg = 1;
        let output_reg = 2;

        let program: Vec<u8> = vec![
            opcode_load,
            first_reg,
            0,      // <- needs a 0 because 16 bit number
            first_num,
            opcode_load,
            second_reg,
            0,
            second_num,
            opcode_add,
            first_reg,
            second_reg,
            output_reg
        ];

        let mut vm = VM::new_with_program(program);

        vm.run();

        assert_eq!(vm.registers[output_reg as usize], (first_num - second_num) as i32);

    }
}

