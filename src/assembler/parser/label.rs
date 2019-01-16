use crate::assembler::Token;
use nom::types::CompleteStr;
use nom::{alphanumeric, multispace};

named!(pub parse_label_declaration<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: alphanumeric >>
            tag!(":") >>
            opt!(multispace) >>
            (
                Token::LabelDeclaration { name: name.to_string() }
            )
        )
    )
);

named!(pub parse_label_usage<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("@") >>
            name: alphanumeric >>
            opt!(multispace) >>
            (
                Token::LabelUsabe { name: name.to_string() }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        let result = parse_label_declaration(CompleteStr("test:"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelDeclaration {
                name: "test".to_string()
            }
        );
        let result = parse_label_declaration(CompleteStr("test"));
        assert_eq!(result.is_err(), true);
    }
}
