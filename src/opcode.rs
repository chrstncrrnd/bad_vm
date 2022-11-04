
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OPCode {
    HALT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
}

impl From<u8> for OPCode{
    fn from(n: u8) -> Self {
        match n {
            0 => Self::HALT,
            1 => Self::LOAD,
            2 => Self::ADD,
            3 => Self::SUB,
            4 => Self::MUL,
            5 => Self::DIV,
            _ => Self::IGL
        }
    }
}

impl Into<u8> for OPCode{
    fn into(self) -> u8 {
        match self {
            Self::HALT => 0,
            Self::LOAD => 1,
            Self::ADD => 2,
            Self::SUB => 3,
            Self::MUL => 4,
            Self::DIV => 5,
            // you shouldn't have anything else lol
            _ => 255,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn num_match(num: u8, opcode: OPCode){
        assert_eq!(num, OPCode::into(opcode));
        assert_eq!(opcode, OPCode::from(num));
    }

    #[test]
    fn from_into_match() {
        num_match(0, OPCode::HALT);
        num_match(1, OPCode::LOAD);
        num_match(2, OPCode::ADD);
        num_match(3, OPCode::SUB);
        num_match(4, OPCode::MUL);
        num_match(5, OPCode::DIV);
    }
}
