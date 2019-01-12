use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

named!(parse_register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register{
                    reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
        )
    )
);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = parse_register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = parse_register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = parse_register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }
}
