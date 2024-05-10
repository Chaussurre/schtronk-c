use crate::ast::parser_user::*;
use crate::ast::Ast::*;
use crate::values::Value;

#[test]
fn single_value_parsing() {
    let parser = ProgParser::new();
    let mut symbols = SymbolsTable::new();

    debug_assert_eq!(
        parser.parse(&mut symbols, "22").unwrap(),
        Leaf(Value::INT(22))
    );
    debug_assert_eq!(
        parser.parse(&mut symbols, "(42)").unwrap(),
        Leaf(Value::INT(42))
    );
    debug_assert_eq!(
        parser.parse(&mut symbols, "((((666))))").unwrap(),
        Leaf(Value::INT(666))
    );
    debug_assert!(parser.parse(&mut symbols, "((22)").is_err());
}

#[test]
fn expression_parsing() {
    let parser = ProgParser::new();
    let mut symbols = SymbolsTable::new();

    let ast = parser.parse(&mut symbols, "2 + 20").unwrap();

    debug_assert_eq!(format!("{}", ast), "(2 + 20)");
    debug_assert_eq!(ast.eval(&mut symbols).as_int(), 22);

    let ast = parser.parse(&mut symbols, "21     *2").unwrap();

    debug_assert_eq!(format!("{}", ast), "(21 * 2)");
    debug_assert_eq!(ast.eval(&mut symbols).as_int(), 42);

    let ast = parser
        .parse(&mut symbols, "6 +(50     +    50     )*     6 + 60")
        .unwrap();

    debug_assert_eq!(format!("{}", ast), "((6 + ((50 + 50) * 6)) + 60)");
    debug_assert_eq!(ast.eval(&mut symbols).as_int(), 666);

    let ast = parser.parse(&mut symbols, "66 + ( 2 *3");

    assert!(ast.is_err());
}
