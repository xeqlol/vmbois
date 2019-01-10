#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL
}

pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction {
            opcode: opcode
        }
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