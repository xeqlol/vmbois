use super::instruction::AssemblerInstruction;
use super::integer_operand::parse_integer_operand;
use crate::assembler::Token;
use nom::alpha1;
use nom::types::CompleteStr;

named!(pub parse_directive_declaration<CompleteStr, Token>,
    do_parse!(
        tag!(".") >>
        name: alpha1 >>
        (
            Token::Directive { name: name.to_string() }
        )
    )
);

named!(parse_directive_combined<CompleteStr, AssemblerInstruction>,
    ws!(
        do_parse!(
            tag!(".") >>
            name: parse_directive_declaration >>
            o1: opt!(parse_integer_operand) >>
            o2: opt!(parse_integer_operand) >>
            o3: opt!(parse_integer_operand) >>
            (
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(name),
                    label: None,
                    operand1: o1,
                    operand2: o2,
                    operand3: o3
                }
            )
        )
    )
);

named!(pub parse_directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            parse_directive_combined
        ) >>
        (
            ins
        )
    )
);
