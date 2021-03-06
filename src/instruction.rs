use nom::types::CompleteStr;
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
    JNEQ,
    ALOC,

    IGL(u8),
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Opcode::*;

        let opcode = match self {
            IGL(code) => {
                return write!(f, "0x{:02X?}", code);
            }
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
            JEQ => "jeq",
            JNEQ => "jneq",
            ALOC => "aloc",
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
            0x09 => EQ,
            0x0A => NEQ,
            0x0B => GT,
            0x0C => LT,
            0x0D => GTQ,
            0x0E => LTQ,
            0x0F => JEQ,
            0x10 => JNEQ,
            0x11 => ALOC,
            code => IGL(code),
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        use self::Opcode::*;

        match v {
            CompleteStr("load") => LOAD,
            CompleteStr("add") => ADD,
            CompleteStr("sub") => SUB,
            CompleteStr("mul") => MUL,
            CompleteStr("div") => DIV,
            CompleteStr("hlt") => HLT,
            CompleteStr("jmp") => JMP,
            CompleteStr("jmpf") => JMPF,
            CompleteStr("jmpb") => JMPB,
            CompleteStr("eq") => EQ,
            CompleteStr("neq") => NEQ,
            CompleteStr("gtq") => GTQ,
            CompleteStr("gt") => GT,
            CompleteStr("ltq") => LTQ,
            CompleteStr("lt") => LT,
            CompleteStr("jneq") => JNEQ,
            CompleteStr("aloc") => ALOC,
            CompleteStr(_) => IGL(0xFF),
        }
    }
}

impl Opcode {
    pub fn to_u8(&self) -> u8 {
        use self::Opcode::*;

        match self {
            HLT => 0x00,
            LOAD => 0x01,
            ADD => 0x02,
            SUB => 0x03,
            MUL => 0x04,
            DIV => 0x05,
            JMP => 0x06,
            JMPF => 0x07,
            JMPB => 0x08,
            EQ => 0x09,
            NEQ => 0x0A,
            GT => 0x0B,
            LT => 0x0C,
            GTQ => 0x0D,
            LTQ => 0x0E,
            JEQ => 0x0F,
            JNEQ => 0x10,
            ALOC => 0x11,
            IGL(code) => *code,
        }
    }
}

#[allow(dead_code)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode }
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
        let inst = Instruction::new(Opcode::IGL(0x01));
        assert_eq!(inst.opcode, Opcode::IGL(0x01));
    }

    #[test]
    fn tets_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL(0xFF));
    }
}
