use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn from(name: &str) -> Self {
        let name = name.to_string();
        Self { name }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
