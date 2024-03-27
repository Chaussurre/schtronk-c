use crate::ast::parser_user::*;

#[test]
pub fn parsing_assignment() {
    let parser = ProgParser::new();
    let mut symbols = SymbolsTable::new();

    debug_assert_eq!(
        format!("{}", parser.parse(&mut symbols, "let a = 42").unwrap()),
        "(let a = 42)"
    );
    debug_assert_eq!(
        format!(
            "{}",
            parser.parse(&mut symbols, "let a = (1   +    2)").unwrap()
        ),
        "(let a = (1 + 2))"
    );
    debug_assert_eq!(
        format!("{}", parser.parse(&mut symbols, "let a = (((3)))").unwrap()),
        "(let a = 3)"
    );
    debug_assert!(parser.parse(&mut symbols, "let a = ").is_err());
    debug_assert!(parser.parse(&mut symbols, "a = 3").is_err());
}
