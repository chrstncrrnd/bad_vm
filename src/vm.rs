pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: vec![],
            pc: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

}