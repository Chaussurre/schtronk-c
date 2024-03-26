mod op_code;

use std::fmt::{Debug, Display, Formatter};
use parser_builder::*;

pub mod parser_builder {
    pub use super::*;
    pub use super::Ast::*;
    pub use op_code::OpCode;
    pub use op_code::OpCode::*;

}

pub mod parser_user {
    use lalrpop_util::lalrpop_mod;
    use lalrpop_util::lexer::Token;
    use crate::ast::Ast;

    lalrpop_mod!(pub calculator); // synthesized by LALRPOP my beloved <3
    pub type ParserError<'input> = lalrpop_util::ParseError<usize, Token<'input>, &'static str>;
    pub type ParseResult<'input> = Result<Ast, ParserError<'input>>;
}

#[derive(Eq, PartialEq)]
pub enum Ast {
    Number(i32),
    Op(Box<Ast>, OpCode, Box<Ast>),
}

impl Ast {
    pub fn eval(&self) -> i32 {
        match self {
            Number(val) => *val,
            Op(left, op, right) => op.eval(left.eval(), right.eval())
        }
    }

    pub fn parse_op(left: Ast, op: OpCode, right: Ast) -> Self {
        Op(Box::new(left), op, Box::new(right))
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(val) => Display::fmt(val, f),
            Op(left, op, right) => f.write_fmt(format_args!("({} {} {})", *left, op, *right)),
        }
    }
}

impl Debug for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
