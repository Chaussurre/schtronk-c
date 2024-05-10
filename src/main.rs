mod ast;
mod input_reader;
mod symbols_table;
#[cfg(test)]
mod tests;
mod values;

use crate::input_reader::Input;
use crate::values::Value;
use ast::parser_user::*;
use std::io::*;

fn main() {
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();
    let parser = ProgParser::new();
    let mut symbols = SymbolsTable::new();

    while let Some(input) = Input::read(">>>", &mut stdin, &mut stdout) {
        if let Some(input) = input.as_string() {
            let parsed_res = parse(&parser, &mut symbols, input);
            display_ast(parsed_res, &mut stdout, &symbols);
        }
    }
}

fn display_ast(result: Option<Ast>, stdout: &mut StdoutLock, symbols: &SymbolsTable) {
    if let Some(ast) = result {
        match ast.eval(symbols) {
            Value::NOTHING => stdout.write_fmt(format_args!("Ast: {}\nNo value\n", ast)),
            res => stdout.write_fmt(format_args!("Ast: {}\nValue: {}\n", ast, res)),
        }
        .unwrap();
    };
}

fn parse(parser: &ProgParser, symbols: &mut SymbolsTable, input: &str) -> Option<Ast> {
    let parsed = parser.parse(symbols, input);
    if let Err(error) = &parsed {
        eprintln!("Error parsing: {:?}", error);
    };

    parsed.ok()
}
