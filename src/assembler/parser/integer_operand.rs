use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;
use crate::assembler::parser::register::parse_register;

named!(pub parse_integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand { value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

named!(pub parse_operand<CompleteStr, Token>,
    alt!(
        parse_integer_operand |
        parse_register
    )
);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = parse_integer_operand(CompleteStr("#10"));
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntegerOperand { value: 10 });

        let result = parse_integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }
}
