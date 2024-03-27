use super::parser_builder::*;
use std::fmt::{Debug, Display, Formatter, Write};

#[derive(Eq, PartialEq)]
pub enum OpCode {
    Mul,
    Div,
    Add,
    Sub,
}

impl OpCode {
    pub fn eval(&self, left: i32, right: i32) -> i32 {
        match self {
            Mul => left * right,
            Div => left / right,
            Add => left + right,
            Sub => left - right,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Mul => '*',
            Div => '/',
            Add => '+',
            Sub => '-',
        };

        f.write_char(char)
    }
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
