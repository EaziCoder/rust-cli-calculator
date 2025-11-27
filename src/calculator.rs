#![allow(dead_code)]

use std::fs;
use std::io::{Error, Write};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sqrt,
    Abs,
    Neg,
    Mod,
    Log,
}

#[derive(Debug)]
pub enum CalcError {
    DivisionByZero,
    UnknownOperation,
    ParseError(String),
    IoError(String),
}

impl From<Error> for CalcError {
    fn from(error: Error) -> Self {
        CalcError::IoError(error.to_string())
    }
    /*Now, any I/O function like fs::File::create or fs::read_to_string
    can ? its result and automatically return a CalcError::IoError. */
}

pub struct Calculator {
    pub history: Vec<String>,
}

impl Calculator {
    pub fn _new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn record(&mut self, operation: Operation, a: f64, b: f64, result: f64) {
        let entry = match operation {
            Operation::Sqrt | Operation::Abs | Operation::Neg => {
                // rust allows grouping multiple patterns using |
                format!("{operation:?} {a} = {result}")
            }
            _ => format!("{a} {:?} {b} = {result}", operation),
        };

        self.history.push(entry);
    }

    pub fn show_history(&self) -> String {
        let mut output = String::new();

        if self.history.is_empty() {
            return "No calculations yet".to_string();
        }

        for (index, entry) in self.history.iter().enumerate() {
            output.push_str(&format!("{}: {}\n", index, entry));
        }

        output
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        // .clear() method removes all elements from the vector
    }

    pub fn save_history(&self) -> Result<(), CalcError> {
        let mut file = fs::File::create("history.txt")?;

        for entry in &self.history {
            writeln!(file, "{}", entry)?;
            // writeln! macro writes a string to a file followed by a newline.
        }

        Ok(())
    }

    pub fn load_history(&mut self) -> Result<(), CalcError> {
        let contents = fs::read_to_string("history.txt")?;
        self.history.clear();

        for line in contents.lines() {
            // contents.lines() splits the file contents into individual lines, returning an iterator.
            self.history.push(line.to_string());
            // .to_string() converts &str to String.
        }

        Ok(())
    }
}

pub fn compute(operation: Operation, a: f64, b: f64) -> Result<f64, CalcError> {
    match operation {
        Operation::Add => Ok(a + b),
        Operation::Div => {
            if b == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        Operation::Mul => Ok(a * b),
        Operation::Sub => Ok(a - b),
        Operation::Pow => Ok(a.powf(b)),
        Operation::Mod => Ok(a % b),
        Operation::Log => {
            if a <= 0.0 || b <= 0.0 {
                Err(CalcError::ParseError("Log numbers must be positive".into()))
            } else {
                Ok(a.log(b))
            }
        }
        Operation::Sqrt => Ok(a.sqrt()),
        Operation::Abs => Ok(a.abs()),
        Operation::Neg => Ok(-a),
        _ => Err(CalcError::UnknownOperation),
    }
}
