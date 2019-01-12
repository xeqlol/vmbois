use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;

named!(pub parse_opcode<CompleteStr, Token>,
    do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
        )
    );

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_opcode() {
        let result = parse_opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        let result = parse_opcode(CompleteStr("loda"));
        assert_eq!(result.is_ok(), false);
    }
}
