use crate::calculator;
use crate::ast::parser_builder::*;

#[test]
fn single_value_parsing() {
    let parser = calculator::ExprParser::new();

    debug_assert_eq!(parser.parse("22").unwrap(), Number(22));
    debug_assert_eq!(parser.parse("(42)").unwrap(), Number(42));
    debug_assert_eq!(parser.parse("((((666))))").unwrap(), Number(666));
    debug_assert!(parser.parse("((22)").is_err());
}

#[test]
fn expression_parsing() {
    let parser = calculator::ExprParser::new();

    let ast = parser.parse("2 + 20").unwrap();

    debug_assert_eq!(format!("{}", ast), "(2 + 20)");
    debug_assert_eq!(ast.eval(), 22);

    let ast = parser.parse("21     *2").unwrap();

    debug_assert_eq!(format!("{}", ast), "(21 * 2)");
    debug_assert_eq!(ast.eval(), 42);

    let ast = parser.parse("6 +(50     +    50     )*     6 + 60").unwrap();

    debug_assert_eq!(format!("{}", ast), "((6 + ((50 + 50) * 6)) + 60)");
    debug_assert_eq!(ast.eval(), 666);

    let ast = parser.parse("66 + ( 2 *3");

    assert!(ast.is_err());
}