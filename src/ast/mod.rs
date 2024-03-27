mod identifiers;
mod op_code;

use parser_builder::*;
use std::fmt::{Debug, Display, Formatter};

pub mod parser_builder {
    pub use super::identifiers::*;
    pub use super::Ast::*;
    pub use super::*;
    pub use op_code::OpCode;
    pub use op_code::OpCode::*;

    pub use super::parse_assignment;
    pub use super::parse_op;
}

pub mod parser_user {
    use lalrpop_util::lalrpop_mod;lalrpop_mod!(pub calculator); // synthesized by LALRPOP my beloved <3

    pub use super::Ast;
    pub use calculator::ProgParser;
}

#[derive(Eq, PartialEq)]
pub enum Ast {
    Number(i32),
    Variable(Identifier),
    Assignment(Identifier, Box<Ast>),
    Op(Box<Ast>, OpCode, Box<Ast>),
}

impl Ast {
    pub fn eval(&self) -> Option<i32> {
        match self {
            Number(val) => Some(*val),
            Op(left, op, right) => Some(op.eval(left.eval()?, right.eval()?)),
            _ => None,
        }
    }
}
pub fn parse_op(left: Ast, op: OpCode, right: Ast) -> Ast {
    Op(Box::new(left), op, Box::new(right))
}

pub fn parse_assignment(identifier: Identifier, value: Ast) -> Ast {
    Assignment(identifier, Box::new(value))
}

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(val) => Display::fmt(val, f),
            Variable(identifier) => Display::fmt(identifier, f),
            Op(left, op, right) => f.write_fmt(format_args!("({} {} {})", *left, op, *right)),
            Assignment(identifier, value) => {
                f.write_fmt(format_args!("(let {} = {})", identifier, *value))
            }
        }
    }
}

impl Debug for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
