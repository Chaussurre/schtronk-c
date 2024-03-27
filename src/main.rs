mod ast;
mod input_reader;
#[cfg(test)]
mod tests;

use crate::input_reader::Input;
use ast::parser_user::*;
use std::io::*;

fn main() {
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();
    let parser = ProgParser::new();

    while let Some(input) = Input::read(">>>", &mut stdin, &mut stdout) {
        if let Some(input) = input.as_string() {
            let parsed_res = parse(&parser, input);
            display_ast(parsed_res, &mut stdout);
        }
    }
}

fn display_ast(result: Option<Ast>, stdout: &mut StdoutLock) {
    if let Some(ast) = result {
        match ast.eval() {
            None => stdout.write_fmt(format_args!("Ast: {}\nNo value\n", ast)),
            Some(res) => stdout.write_fmt(format_args!("Ast: {}\nValue: {}\n", ast, res)),
        }
        .unwrap();
    };
}

fn parse(parser: &ProgParser, input: &str) -> Option<Ast> {
    let parsed = parser.parse(input);
    if let Err(error) = &parsed {
            eprintln!("Error parsing: {:?}", error);
    };

    parsed.ok()
}
