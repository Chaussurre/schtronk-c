use crate::ast::parser_user::*;

#[test]
pub fn parsing_assignment() {
    let parser = calculator::ProgParser::new();

    debug_assert_eq!(
        format!("{}", parser.parse("let a = 42").unwrap()),
        "(let a = 42)"
    );
    debug_assert_eq!(
        format!("{}", parser.parse("let a = (1   +    2)").unwrap()),
        "(let a = (1 + 2))"
    );
    debug_assert_eq!(
        format!("{}", parser.parse("let a = (((3)))").unwrap()),
        "(let a = 3)"
    );
    debug_assert!(parser.parse("let a = ").is_err());
    debug_assert!(parser.parse("a = 3").is_err());
}
