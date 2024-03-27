use std::io;
use std::io::{BufRead, StdinLock, StdoutLock, Write};

use Input::*;

pub enum Input {
    Valid(String),
    Empty,
    End,
    Error(io::Error),
}

impl Input {
    pub fn read(prompt: &str, stdin: &mut StdinLock, stdout: &mut StdoutLock) -> Option<Self> {
        match Self::read_input(prompt, stdin, stdout) {
            Err(error) => {
                eprintln!("Invalid input: {:?}", error);
                Some(Error(error))
            }
            Ok(End) => None,
            Ok(input) => Some(input),
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Valid(string) => Some(string),
            _ => None,
        }
    }

    fn read_input(
        prompt: &str,
        stdin: &mut StdinLock,
        stdout: &mut StdoutLock,
    ) -> io::Result<Self> {
        let mut input = String::new();
        stdout.write_all(prompt.as_ref())?;
        stdout.flush()?;

        let size = stdin.read_line(&mut input)?;

        stdin.consume(size);

        if size == 0 {
            Ok(End)
        } else if input.trim().is_empty() {
            Ok(Empty)
        } else {
            Ok(Valid(input))
        }
    }
}
