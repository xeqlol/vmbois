use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,

    LOAD,
    
    ADD,
    SUB,
    MUL,
    DIV,

    JMP,
    JMPF,
    JMPB,

    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,

    IGL,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Opcode::*;

        let opcode = match self {
            IGL => "illegal",
            HLT => "hlt",
            LOAD => "load",
            ADD => "add",
            SUB => "sub",
            MUL => "mul",
            DIV => "div",
            JMP => "jmp",
            JMPF => "jmpf",
            JMPB => "jmpb",
            EQ => "eq",
            NEQ => "neq",
            GT => "gt",
            LT => "lt",
            GTQ => "gtq",
            LTQ => "ltq",
            JEQ => "jeq"
        };

        write!(f, "{}", opcode)
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        use self::Opcode::*;

        match v {
            0x00 => HLT,
            0x01 => LOAD,
            0x02 => ADD,
            0x03 => SUB,
            0x04 => MUL,
            0x05 => DIV,
            0x06 => JMP,
            0x07 => JMPF,
            0x08 => JMPB,
            _ => IGL,
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
