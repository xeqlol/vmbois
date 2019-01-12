use super::integer_operand::parse_integer_operand;
use super::opcode::parse_opcode;
use super::register::parse_register;
use crate::assembler::Token;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        match &self.opcode {
            Token::Op { code } => {
                result.push(code.to_u8());
            }
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        }

        for operand in [&self.operand1, &self.operand2, &self.operand3].iter() {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut result),
                None => {}
            }
        }

        result
    }

    fn extract_operand(t: &Token, result: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                result.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;

                result.push(byte2 as u8);
                result.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

named!(pub parse_instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: parse_opcode >>
        r: parse_register >>
        i: parse_integer_operand >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
            }
        )
    )
);

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_one() {
        let result = parse_instruction_one(CompleteStr("load $0 #100\n")).unwrap();
        assert_eq!(
            result,
            (
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            )
        );
    }
}
