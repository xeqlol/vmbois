use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::alpha1;
use nom::types::CompleteStr;

named!(pub parse_opcode<CompleteStr, Token>,
    do_parse!(
        opcode: alpha1 >>
        (
            {
                Token::Op{code: Opcode::from(opcode)}
            }
        )
    )
);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_opcode() {
        let result = parse_opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        let result = parse_opcode(CompleteStr("loda"));
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Op {
                code: Opcode::IGL(0xFF)
            }
        )
    }
}
