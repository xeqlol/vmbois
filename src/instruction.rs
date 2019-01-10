use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    IGL,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let opcode = match self {
            Opcode::IGL => "illegal",
            Opcode::HLT => "hlt",
            Opcode::LOAD => "load",
            Opcode::ADD => "add",
            Opcode::SUB => "sub",
            Opcode::MUL => "mul",
            Opcode::DIV => "div",
        };

        write!(f, "{}", opcode)
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Opcode::HLT,
            0x01 => Opcode::LOAD,
            0x02 => Opcode::ADD,
            0x03 => Opcode::SUB,
            0x04 => Opcode::MUL,
            0x05 => Opcode::DIV,
            _ => Opcode::IGL,
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode: opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let inst = Instruction::new(Opcode::HLT);
        assert_eq!(inst.opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_igl() {
        let inst = Instruction::new(Opcode::IGL);
        assert_eq!(inst.opcode, Opcode::IGL);
    }
}
