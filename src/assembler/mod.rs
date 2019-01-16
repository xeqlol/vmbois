use crate::instruction::Opcode;

pub mod parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsabe { name: String },
    Directive { name: String },
}
