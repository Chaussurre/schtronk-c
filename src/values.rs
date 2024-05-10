use std::fmt::{Display, Formatter};
#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    INT(i32),
    STRING(String),
    NOTHING,
}

impl Value {
    pub fn as_int(&self) -> i32 {
        match self {
            Value::INT(val) => *val,
            _ => panic!("Not an int"),
        }
    }

    pub fn as_str(&self) -> &String {
        match self {
            Value::STRING(str) => str,
            _ => panic!("not a string"),
        }
    }

    pub fn parse_int(str: &str) -> Self {
        if let Ok(val) = str.parse::<i32>() {
            Self::INT(val)
        } else {
            Self::NOTHING
        }
    }

    pub fn parse_str(str: &str) -> Self {
        Self::STRING(str[1..(str.len() - 1)].to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::INT(val) => f.write_fmt(format_args!("{}", val)),
            Value::STRING(val) => f.write_str(val),
            Value::NOTHING => f.write_str("No value"),
        }
    }
}
