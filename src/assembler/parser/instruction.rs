use super::directive::parse_directive;
use super::integer_operand::parse_operand;
use super::label::parse_label_declaration;
use super::opcode::parse_opcode;
use crate::assembler::Token;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        match &self.opcode {
            Some(Token::Op { code }) => {
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

named!(pub parse_instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            parse_instruction_combined |
            parse_directive
        ) >>
        (
            ins
        )
    )
);

named!(parse_instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(parse_label_declaration) >>
        o: parse_opcode >>
        o1: opt!(parse_operand) >>
        o2: opt!(parse_operand) >>
        o3: opt!(parse_operand) >>
        (
            AssemblerInstruction {
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3
            }
        )
    )
);

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_one() {
        let result = parse_instruction_combined(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None,
                    label: None,
                    directive: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_two() {
        let result = parse_instruction_combined(CompleteStr("hlt"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    operand1: None,
                    operand2: None,
                    operand3: None,
                    label: None,
                    directive: None
                }
            ))
        )
    }
}
