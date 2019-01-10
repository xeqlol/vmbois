use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    IGL,
    HLT,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::HLT => write!(f, "hlt"),
            Opcode::IGL => write!(f, "illegal"),
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
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
