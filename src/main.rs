mod ast;
#[cfg(test)]
mod parser_tests;

use ast::parser_user::*;
use std::io::*;

fn main() {
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    loop {
        match get_input(">>>", &mut stdin, &mut stdout) {
            Ok(Some(input)) => parse(input, &mut stdout),
            Ok(None) => return,
            Err(error) => eprintln!("Error reading input: {}", error),
        }
    }
}

fn get_input(
    prompt: &str,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<Option<String>> {
    stdout.write(prompt.as_ref())?;
    stdout.flush()?;

    let mut input = String::new();

    let size = stdin.read_line(&mut input)?;
    stdin.consume(size);

    return if size > 0 { Ok(Some(input)) } else { Ok(None) };
}

fn display_ast(result: ParseResult, stdout: &mut StdoutLock) {
    match result {
        Ok(ast) => stdout
            .write_fmt(format_args!("Ast: {}\nResult: {}\n", ast, ast.eval()))
            .unwrap(),
        Err(error) => eprintln!("Parsing error: {:?}", error),
    }
}

fn parse(input: String, stdout: &mut StdoutLock) {
    let trimmed_input = input.trim();

    if !trimmed_input.is_empty() {
        let parse_res: ParseResult = calculator::ExprParser::new().parse(trimmed_input);
        display_ast(parse_res, stdout);
    }
}
